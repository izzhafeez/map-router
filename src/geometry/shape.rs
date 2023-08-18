use serde::{Serialize, Deserialize};

use crate::geometry::geometry::Geometry;
use crate::geometry::multi_polygon::MultiPolygon;
use crate::geometry::polygon::Polygon;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShapeEnum {
    Polygon(Polygon),
    MultiPolygon(MultiPolygon)
}

impl Geometry for ShapeEnum {}

pub trait Shape : Geometry {
    fn is_in(&self);
}