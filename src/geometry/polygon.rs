use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use std::vec::IntoIter;
use crate::geometry::coord::Coord;
use crate::geometry::geometry::Geometry;
use crate::geometry::shape::Shape;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Polygon {
    inner_boundaries: Vec<Polygon>,
    outer_boundary: Vec<Coord>
}

impl Polygon {
    pub fn without_inner(outer_boundary: Vec<Coord>) -> Self {
        let inner_boundaries: Vec<Polygon> = Vec::new();
        Self { inner_boundaries, outer_boundary }
    }

    pub fn with_inner(outer_boundary: Vec<Coord>, inner_boundaries: Vec<Polygon>) -> Self {
        Self { inner_boundaries, outer_boundary }
    }

    pub fn from_boundaries(boundaries: Vec<Vec<Coord>>) -> Self {
        let mut boundaries_iter: IntoIter<Vec<Coord>> = boundaries.into_iter();

        let outer_boundary: Vec<Coord> = boundaries_iter.next().unwrap();
        let inner_boundaries: Vec<Polygon> = boundaries_iter.map(Polygon::without_inner).collect();

        Polygon::with_inner(outer_boundary, inner_boundaries)
    }
}

impl Geometry for Polygon {}

impl Shape for Polygon {
    fn is_in(&self) {
        todo!()
    }
}

impl Display for Polygon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Inner: {:?}, Outer: {:?})", self.inner_boundaries, self.outer_boundary)
    }
}