use std::collections::HashMap;
use napi_derive::napi;

#[napi(object)]
#[derive(Clone, Debug, PartialEq)]
pub struct Geometry {
    pub _type: String,
    pub coordinates: Vec<f64>,
}

#[napi(object)]
#[derive(Debug,Clone, PartialEq)]
pub struct Feature {
    pub _type: String,
    pub properties: HashMap<String, String>,
    pub geometry: Geometry,
}