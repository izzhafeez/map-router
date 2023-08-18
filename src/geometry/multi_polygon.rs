use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::geometry::geometry::Geometry;
use crate::geometry::polygon::Polygon;
use crate::geometry::shape::Shape;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiPolygon {
    polygons: Vec<Polygon>
}

impl MultiPolygon {
    pub fn from_polygons(polygons: Vec<Polygon>) -> Self {
        Self { polygons: polygons }
    }
}

impl Geometry for MultiPolygon {}

impl Shape for MultiPolygon {
    fn is_in(&self) {
        todo!()
    }
}

impl Display for MultiPolygon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?})", self.polygons)
    }
}