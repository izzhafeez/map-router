use serde::{Serialize, Deserialize};
use crate::geometry::geometry::Geometry;

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiGeometry<T: Geometry> {
    geometries: Vec<Box<T>>
}

impl<T> MultiGeometry<T> {
    pub fn from_geometries(geometries: Vec<T>) -> Self {
        Self { geometries }
    }
}

impl Geometry for MultiGeometry<T> {}