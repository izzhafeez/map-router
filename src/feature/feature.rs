use crate::geometry::geometry::Geometry;

pub trait Feature {
    fn get_geometry(&self) -> Box<dyn Geometry>;
}