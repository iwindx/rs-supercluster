mod option;
mod point;
use rs_kdbush::KDBush;
pub use napi::{bindgen_prelude::*, *};
use napi_derive::napi;
use option::*;
use point::*;
use std::f64::consts::PI;

#[napi]
pub struct SuperCluster {
  options: DefaultOptions,
  points: Option<Vec<Feature>>,
  trees: Vec<Option<KDBush>>,
}

#[napi]
impl SuperCluster {
  #[napi(constructor)]
  pub fn new(options: Option<DefaultOptions>) -> SuperCluster {
    let options = options.and_then(|o| Some(o)).unwrap_or_default();

    let options = DefaultOptions::new().merge(&options);
    let capacity = 20;
    let mut trees = Vec::with_capacity(capacity);
    for _ in 0..capacity {
      trees.push(None);
    }
    SuperCluster {
      options,
      points: None,
      trees,
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
      self.trees[z as usize] = Some(KDBush::create(result, node_size));
    }
    SuperCluster {
      options: self.options,
      points: Some(points),
      trees: self.trees.clone()
    }
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
      y: self.fround(self.lng_y(y)),
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

  pub fn lng_x(&self, lng: f64) -> f64 {
    (lng / 360 as f64) + 0.5
  }

  pub fn lng_y(&self, lat: f64) -> f64 {
    let sin = (lat * PI / 180.0).sin();
    let y = 0.5 - 0.25 * ((1.0 + sin) / (1.0 - sin)).ln() / PI;

    match y {
      y if y < 0.0 => 0.0,
      y if y > 1.0 => 1.0,
      _ => y,
    }
  }

  pub fn fround(&self, x: f64) -> f64 {
    f64::from_bits(x.to_bits() as u64)
  }
}
