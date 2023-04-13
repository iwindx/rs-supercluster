mod option;
mod point;

use kdbush::KDBush;
pub use napi::{bindgen_prelude::*, *};
use napi_derive::napi;
use std::f64::consts::PI;
use std::fmt::Pointer;
use option::*;
use point::*;



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

    let options = DefaultOptions::new().merge(&options);
    let max_zoom = options.max_zoom.unwrap_or_default();
    let mut trees = Vec::with_capacity(max_zoom as usize + 1);
    for _ in 0..(max_zoom as usize + 1) {
      trees.push(None);
    }
    SuperCluster {
      options,
      points: None,
      trees
    }
  }

  #[napi]
  pub fn load(&mut self, points: Vec<Feature>) -> Self {

    let (max_zoom, min_zoom, node_size) = (
      self.options.max_zoom.unwrap_or_default(),
      self.options.min_zoom.unwrap_or_default(),
      self.options.node_size.unwrap_or_default(),
    );

    let mut clusters: Vec<PointCluster> = vec![];
    for (index, point) in points.iter().enumerate() {
      clusters.push(self.create_point_cluster(point.clone(), index));
    }
    let mut _points: Vec<(f64, f64)> = vec![];

    for cluster in &clusters {
      _points.push((cluster.x, cluster.y));
    }
    println!("(max_zoom + 1) as usize {:?}", (max_zoom + 1) as usize);

    self.trees[(max_zoom) as usize] = Some(KDBush::create(_points, node_size));

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
    // let cluster = [];
    let mut points = points.clone();
    let (radius, extent, min_points) = (
      self.options.radius.unwrap_or_default(),
      self.options.extent.unwrap_or_default(),
      self.options.min_points.unwrap_or_default(),
    );
    let power = 2_f64.powi(zoom as i32);

    let r = radius as f64 / (extent as f64 * power);
    for i in 0..points.len() {
      let mut p = &mut points[i];

      if p.zoom <= zoom as f64 {
        continue;
      }

      p.zoom = zoom as f64;

      let tree = match &self.trees[(zoom) as usize] {
        Some(tree) => tree,
        None => panic!("the option is empty")
      };

      let neighbor_ids = tree.within(p.x, p.y, r, |id| print!("{} ", id));
      let numPoints = 1;
      println!("neighbor_ids {:?}", neighbor_ids);
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
