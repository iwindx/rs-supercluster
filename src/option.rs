
use napi_derive::napi;

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
            max_zoom: Some(16),
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

#[derive(Clone, Debug, PartialEq)]
pub struct PointCluster {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
    pub index: usize,
    pub parent_id: i8,
}