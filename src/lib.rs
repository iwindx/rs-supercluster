mod option;
mod point;

use kdbush::KDBush;
pub use napi::{bindgen_prelude::*, *};
use napi_derive::napi;
use std::f64::consts::PI;
use option::DefaultOptions;
use point::*;


#[derive(Clone, Debug, PartialEq)]
pub struct PointCluster {
  x: f64,
  y: f64,
  zoom: f64,
  index: usize,
  parent_id: i8,
}

#[napi]
pub struct SuperCluster {
  options: DefaultOptions,
  points: Option<Vec<Feature>>,
  trees: Vec<Option<KDBush>>
}

#[napi]
impl SuperCluster {
  #[napi(constructor)]
  pub fn new(options: Option<DefaultOptions>) -> SuperCluster {
    let options = options.and_then(|o| Some(o)).unwrap_or_default();

    SuperCluster {
      options: DefaultOptions::new().merge(&options),
      points: None,
      trees: vec![None]
    }
  }

  #[napi]
  pub fn load(&mut self, points: Vec<Feature>) -> Self {
    let DefaultOptions {
      log,
      min_zoom: Some(min_zoom),
      max_zoom: Some(max_zoom),
      node_size,
      ..
    } = self.options;
    let mut clusters: Vec<PointCluster> = vec![];
    for (index, point) in points.iter().enumerate() {
      clusters.push(self.create_point_cluster(point.clone(), index));
    }
    let mut _points: Vec<(f64, f64)> = vec![];

    for cluster in &clusters {
      _points.push((cluster.x, cluster.y));
    }
    self.trees[(max_zoom + 1) as usize] = Some(KDBush::create(_points, node_size.unwrap()));

    for z in (min_zoom..=max_zoom).rev() {
      let cluster = self._cluster(clusters.clone(), z);
    }

    SuperCluster {
      options: self.options,
      points: Some(points),
      trees: vec![None]
    }
  }

  fn _cluster(&self, points: Vec<PointCluster>, zoom: u8) {
    let cluster = [];
    let DefaultOptions {
      radius: Some(radius),
      extent: Some(extent),
      min_points,
      ..
    } = self.options;
    let power = 2_f64.powi(zoom as i32);
    let r = radius as u16 / (extent * power as u16);

    for i in 0..points.len() {
      let mut p = &mut points[i];

      if p.zoom <= zoom as f64 {
        continue;
      }

      p.zoom = zoom as f64;
      let Some(tree) = &self.trees[(zoom + 1) as usize];

      let neighbor_ids = tree.within(p.x, p.y, r as f64, |id| print!("{} ", id));
      let numPoints = 1;
      neighbor_ids
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
