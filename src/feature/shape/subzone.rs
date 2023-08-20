use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::feature::feature::Feature;
use crate::geometry::geometry::Geometry;
use crate::geometry::shape::ShapeEnum;

#[derive(Debug, Serialize, Deserialize)]
pub struct Subzone {
    id: i16,
    name: String,
    code: String,
    planning_area: String,
    planning_area_code: String,
    region: String,
    region_code: String,
    geometry: ShapeEnum
}

impl Subzone {
    pub fn new(id: i16, 
               name: String, code: String,
               planning_area: String, planning_area_code: String,
               region: String, region_code: String,
               geometry: ShapeEnum) -> Self {
        Self { 
            id, 
            name, code,
            planning_area, planning_area_code,
            region, region_code,
            geometry,
        }
    }

    pub fn get_header(&self) -> String {
        self.name.clone()
    }
}

impl Feature for Subzone {
    fn get_geometry(&self) -> Box<dyn Geometry> {
        Box::new(self.geometry.clone()) as Box<dyn Geometry>
    }
}

impl Display for Subzone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.name, self.planning_area, self.region)
    }
}