use std::collections::HashMap;
use napi_derive::napi;
use napi::{JsUnknown};
#[napi(object)]
#[derive(Clone, Debug, PartialEq)]
pub struct Geometry {
    pub _type: String,
    pub coordinates: Vec<f64>,
}

#[napi(object)]
pub struct Feature {
    pub _type: String,
    pub properties:HashMap<String, JsUnknown>,
    pub geometry: Geometry,
}