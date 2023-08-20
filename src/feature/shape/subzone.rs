use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::feature::feature::Feature;
use crate::geometry::geometry::Geometry;
use crate::geometry::shape::ShapeEnum;

#[derive(Debug, Serialize, Deserialize)]
pub struct Subzone {
    id: i16,
    name: String,
    geometry: ShapeEnum
}

impl Subzone {
    pub fn new(id: i16, name: String, geometry: ShapeEnum) -> Self {
        Self { id, name, geometry }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Feature for Subzone {
    fn get_geometry(&self) -> Box<dyn Geometry> {
        Box::new(self.geometry.clone()) as Box<dyn Geometry>
    }
}