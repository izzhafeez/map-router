use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::feature::shape::subzone::Subzone;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningArea {
    name: String,
    subzones: HashMap<String, Subzone>
}

impl PlanningArea {
    pub fn new(name: String, subzones: HashMap<String, Subzone>) -> Self {
        Self { name, subzones }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn combine_with(&mut self, planning_area: PlanningArea) -> () {
        assert_eq!(self.name, planning_area.name);

        for (code, sz) in planning_area.subzones {
            assert!(!self.subzones.contains_key(&code));
            self.subzones.insert(code, sz);
        }
    }
}