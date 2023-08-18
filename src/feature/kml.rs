use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::feature::subzone::subzone::Subzone;

pub struct Kml {
    subzones: HashMap<String, Subzone>
}

impl Kml {
    pub fn new(subzones: HashMap<String, Subzone>) -> Self {
        Self { subzones }
    }
}

impl Display for Kml {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?})", self.subzones)
    }
}