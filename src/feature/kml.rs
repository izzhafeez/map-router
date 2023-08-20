use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::feature::shape::region::Region;

#[derive(Debug, Serialize, Deserialize)]
pub struct Kml {
    regions: HashMap<String, Region>
}

impl Kml {
    pub fn new(regions: HashMap<String, Region>) -> Self {
        Self { regions }
    }
}

impl Display for Kml {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?})", self.regions)
    }
}
