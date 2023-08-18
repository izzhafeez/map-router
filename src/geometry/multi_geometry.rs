use crate::geometry::geometry::Geometry;

#[derive(Debug)]
pub struct MultiGeometry {
    geometries: Vec<Box<dyn Geometry>>
}

impl MultiGeometry {
    pub fn from_geometries(geometries: Vec<Box<dyn Geometry>>) -> Self {
        Self { geometries }
    }
}

impl Geometry for MultiGeometry {}