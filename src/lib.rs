mod option;
mod point;
pub use napi::{bindgen_prelude::*, *};
use napi_derive::napi;
use option::*;
use point::*;
use rs_kdbush::KDBush;
use std::f64::consts::PI;
use std::ops::Add;
use serde_json::{json, Value};

#[napi]
pub struct SuperCluster {
  options: DefaultOptions,
  points: Option<Vec<Feature>>,
  trees: Vec<Option<KDBush>>,
  cluster: Vec<Option<Vec<Cluster>>>,
}

#[napi]
impl SuperCluster {
  #[napi(constructor)]
  pub fn new(options: Option<DefaultOptions>) -> SuperCluster {
    let options = options.and_then(|o| Some(o)).unwrap_or_default();

    let options = DefaultOptions::new().merge(&options);
    let capacity = 20;
    let mut trees = Vec::with_capacity(capacity);
    let mut cluster = Vec::with_capacity(capacity);
    for _ in 0..capacity {
      cluster.push(None);
      trees.push(None);
    }
    SuperCluster {
      options,
      points: None,
      trees,
      cluster,
    }
  }

  #[napi]
  pub fn load(&mut self, points: Vec<Feature>) -> SuperCluster {
    let (max_zoom, min_zoom, node_size) = (
      self.options.max_zoom.unwrap_or_default(),
      self.options.min_zoom.unwrap_or_default(),
      self.options.node_size.unwrap_or_default(),
    );

    let mut clusters: Vec<PointCluster> = vec![];
    for (index, point) in points.iter().enumerate() {
      clusters.push(self.create_point_cluster(point, index));
    }

    let mut _points: Vec<(f64, f64)> = vec![];

    for cluster in &clusters {
      _points.push((cluster.x, cluster.y));
    }

    self.cluster[(max_zoom + 1) as usize] = Some(
      clusters
        .clone()
        .into_iter()
        .map(|point| Cluster::PointClusterItem(point))
        .collect(),
    );
    self.trees[(max_zoom + 1) as usize] = Some(KDBush::create(_points, node_size));

    for z in (min_zoom..=max_zoom).rev() {
      let cluster = self._cluster(&mut clusters.clone(), z);
      let result: Vec<(f64, f64)> = cluster
        .iter()
        .map(|point| match point {
          Cluster::PointClusterItem(point) => (point.x, point.y),
          Cluster::CreateClusterItem(point) => (point.x, point.y),
        })
        .collect();
      self.cluster[z as usize] = Some(cluster);
      self.trees[z as usize] = Some(KDBush::create(result, node_size));
    }
    SuperCluster {
      options: self.options,
      points: Some(points),
      trees: self.trees.clone(),
      cluster: self.cluster.clone(),
    }
  }

  //noinspection ALL
  #[napi]
  pub fn get_clusters(&self, bbox: Vec<f64>, zoom: i8) {
    const _360: f64 = 360_f64;
    const _180: f64 = 180_f64;
    let mut min_lng = ((bbox[0].add(180 as f64)) % _360 + _360) % _360 - _180;
    let min_lat = bbox[1].max(-90.0).min(90.0);
    let mut max_lng = match bbox[2] {
      180.0 => 180_f64,
      _ => ((bbox[2].add(180 as f64)) % _360 + _360) % _360 - _180,
    };
    let max_lat = bbox[3].max(-90.0).min(90.0);

    if bbox[2] - bbox[0] >= _360 {
      min_lng = -_180;
      max_lng = _180;
    } else if min_lng > max_lng {
      let eastern_hem = self.get_clusters(Vec::from([min_lng, min_lat, _180, max_lng]), zoom);
      let western_hem = self.get_clusters(Vec::from([-_180, min_lat, max_lng, max_lat]), zoom);
    }

    let mut cluster = Vec::new();
    if let Some(tree) = &self.trees[self._limit_zoom(zoom)] {
      let mut ids = Vec::new();
      tree.range(
        self.lng_x(min_lng),
        self.lat_y(max_lat),
        self.lng_x(max_lng),
        self.lat_y(min_lat),
        |id| ids.push(id),
      );
      for id in ids.into_iter() {
        if let Some(points) = self.cluster[zoom as usize].clone() {
          if let Some(c) = points.get(id) {
            match c {
              Cluster::PointClusterItem(point) => {
                if let Some(_points) = &self.points {
                  cluster.push(_points.get(point.index));
                }
              }
              Cluster::CreateClusterItem(point) => {
                cluster.push(self.get_cluster_json(point))
              }
            };
          };
        }
      }
    }
    cluster
  }

  fn _limit_zoom(&self, z: i8) -> usize {
    let (max_zoom, min_zoom) = (
      self.options.max_zoom.unwrap_or_default(),
      self.options.min_zoom.unwrap_or_default(),
    );
    u8::max(min_zoom, u8::min(f64::floor(z as f64) as u8, max_zoom + 1)) as usize
  }

  fn _cluster(&self, points: &mut Vec<PointCluster>, zoom: u8) -> Vec<Cluster> {
    let mut clusters: Vec<Cluster> = Vec::with_capacity(points.len());

    let (radius, extent, min_points) = (
      self.options.radius.unwrap_or_default(),
      self.options.extent.unwrap_or_default(),
      self.options.min_points.unwrap_or_default(),
    );
    let power = 2_f64.powi(zoom as i32);

    let r = radius as f64 / (extent as f64 * power);
    for i in 0..points.len() {
      let mut p = points[i].clone();

      if p.zoom <= zoom as f64 {
        continue;
      }

      p.zoom = zoom as f64;
      let tree = match &self.trees[(zoom + 1) as usize] {
        Some(tree) => tree,
        None => panic!("the option is empty"),
      };

      let mut neighbor_ids = Vec::new();
      tree.within(p.x, p.y, r, |id| neighbor_ids.push(id));

      let num_points_origin = 1;
      let mut num_points = num_points_origin;

      for neighbor_id in &neighbor_ids {
        if let Some(b) = &points.get(*neighbor_id as usize) {
          if b.zoom > zoom as f64 {
            num_points += 1
          }
        }
      }

      if num_points > num_points_origin && num_points >= min_points {
        let (mut wx, mut wy) = (
          p.x * num_points_origin as f64,
          p.y * num_points_origin as f64,
        );

        let cluster_properties = self._map(&mut p, true);
        let id = (i << 5) + (zoom as usize + 1);
        for neighbor_id in neighbor_ids {
          if let Some(b) = points.get_mut(neighbor_id) {
            if b.zoom <= zoom as f64 {
              continue;
            }
            b.zoom = zoom as f64;

            let num_points: f64 = 1_f64;
            wx += b.x * num_points;
            wy += b.y * num_points;
            b.parent_id = id as i8;
          }
        }
        p.parent_id = id as i8;
        let create_cluster = self.create_cluster(
          wx / num_points as f64,
          wy / num_points as f64,
          id,
          num_points,
        );
        clusters.push(Cluster::CreateClusterItem(create_cluster));
      } else {
        clusters.push(Cluster::PointClusterItem(p));
      }
    }
    return clusters;
  }

  fn _map(&self, p: &mut PointCluster, clone: bool) {
    let points = &self.points;
    if let Some(points) = points {
      let index = p.index;
      let original = &points[index].properties;
    }
  }

  fn create_point_cluster(&self, p: &Feature, id: usize) -> PointCluster {
    let p = p.clone();
    let coordinates = &p.geometry.coordinates;
    let [x, y]: [f64; 2] = [coordinates[0], coordinates[1]];

    PointCluster {
      x: self.fround(self.lng_x(x)),
      y: self.fround(self.lat_y(y)),
      zoom: f64::INFINITY,
      index: id,
      parent_id: -1,
    }
  }

  fn create_cluster(&self, x: f64, y: f64, id: usize, num_points: u8) -> CreateCluster {
    CreateCluster {
      x: self.fround(x),
      y: self.fround(y),
      zoom: f64::INFINITY,
      id,
      parent_id: -1,
      num_points,
    }
  }

  fn get_cluster_json(&self, cluster: CreateCluster) -> Value {
    json!({
      "type": "Feature",
      "id": cluster.id,
      "geometry" : {
        "type": "Point",
        "coordinates": [self.x_lng(cluster.x), self.y_lat(cluster.y)]
      }
    })
  }

  pub fn lng_x(&self, lng: f64) -> f64 {
    (lng / 360 as f64) + 0.5
  }

  pub fn lat_y(&self, lat: f64) -> f64 {
    let sin = (lat * PI / 180.0).sin();
    let y = 0.5 - 0.25 * ((1.0 + sin) / (1.0 - sin)).ln() / PI;

    match y {
      y if y < 0.0 => 0.0,
      y if y > 1.0 => 1.0,
      _ => y,
    }
  }

  fn x_lng(&self, x: f64) -> f64 {
    (x - 0.5) * 360_f64
  }

  fn y_lat(&self, y: f64) -> f64 {
    let y2 = (180.0 - y * 360.0) * PI / 180.0;
    360.0 * f64::atan(f64::exp(y2)) / PI - 90.0
  }

  pub fn fround(&self, x: f64) -> f64 {
    f64::from_bits(x.to_bits() as u64)
  }
}
