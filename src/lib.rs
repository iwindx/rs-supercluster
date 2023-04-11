pub use napi::{*, bindgen_prelude::*};
use napi_derive::napi;
use std::f64::consts::PI;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct PointCluster {
  x: f64, // projected point coordinates
  y: f64,
  zoom: f64, // the last zoom the point was processed at
  index: usize, // index of the source feature in the original input array,
  parent_id: i8 // parent cluster id
}

#[napi(object)]
#[derive(Clone, Debug, PartialEq)]
pub struct Geometry {
  pub _type: String,
  pub coordinates: Vec<f64>,
}

#[napi(object)]
#[derive(Debug, PartialEq)]
pub struct Feature {
  pub _type: String,
  pub properties: HashMap<String, String>,
  pub geometry: Geometry
}

// impl Clone for Feature {
//   fn clone(&self) -> Feature {
//     println!("====");
//
//     let feature = self.clone();
//     Feature {
//       _type: feature._type,
//       properties: feature.properties.clone(),
//       geometry: feature.geometry.clone()
//     }
//   }
// }

#[napi(object)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultOptions {
  pub min_zoom: Option<u8>,
  pub max_zoom: Option<u8>,
  pub min_points: Option<u8>,
  pub radius: Option<u8>,
  pub extent: Option<u16>,
  pub node_size: Option<u8>,
  pub log: Option<bool>,
  pub generate_id: Option<bool>,
}

impl DefaultOptions {
  pub fn new() -> Self {
    DefaultOptions {
      min_zoom: Some(0),
      max_zoom: Some(18),
      min_points: Some(2),
      radius: Some(40),
      extent: Some(512),
      node_size: Some(64),
      log: Some(false),
      generate_id: Some(false),
    }
  }

  pub fn merge(&self, other: &DefaultOptions) -> DefaultOptions {
    let DefaultOptions {
      min_zoom,
      max_zoom,
      min_points,
      radius,
      extent,
      node_size,
      log,
      generate_id,
    } = self.clone();

    DefaultOptions {
      min_zoom: Some(other.min_zoom.unwrap_or(min_zoom.unwrap())),
      max_zoom: Some(other.max_zoom.unwrap_or(max_zoom.unwrap())),
      min_points: Some(other.min_points.unwrap_or(min_points.unwrap())),
      radius: Some(other.radius.unwrap_or(radius.unwrap())),
      extent: Some(other.extent.unwrap_or(extent.unwrap())),
      node_size: Some(other.node_size.unwrap_or(node_size.unwrap())),
      log: Some(other.log.unwrap_or(log.unwrap())),
      generate_id: Some(other.generate_id.unwrap_or(generate_id.unwrap())),
    }
  }
}

#[napi]
pub struct SuperCluster {
  options: DefaultOptions,
  points: Option<Vec<Feature>>
}

#[napi]
impl SuperCluster {

  #[napi(constructor)]
  pub fn new(options: Option<DefaultOptions>) -> SuperCluster {
    let options = options.and_then(|o| Some(o)).unwrap_or_default();

    SuperCluster {
      options: DefaultOptions::new().merge(&options),
      points: None
    }
  }

  #[napi]
  pub fn load(&mut self, points: Vec<Feature>) -> Self {
    let DefaultOptions {
      log,
      min_zoom,
      max_zoom,
      node_size,
      ..
    } = self.options;
    let mut clusters:Vec<PointCluster> = vec![];
    for (index, point) in points.iter().enumerate() {
      println!("index {:?}, point {:?}", index, point);
      // self.create_point_cluster(point, index);
      clusters.push(self.create_point_cluster(point.clone(), index));
    }
    println!("clusters: {:?}", clusters);

    SuperCluster {
      options: self.options,
      points: Some(points)
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
      parent_id: -1
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
