use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::geometry::geometry::Geometry;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coord {
    lat: f32,
    lng: f32,
    alt: f32
}

impl Coord {
    pub fn new(lat: f32, lng: f32, alt: f32) -> Self {
        Self { lat, lng, alt }
    }
}

impl Geometry for Coord {}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.lat, self.lng, self.alt)
    }
}