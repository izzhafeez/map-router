use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::feature::shape::planning_area::PlanningArea;

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    name: String,
    planning_areas: HashMap<String, PlanningArea>
}

impl Region {
    pub fn new(name: String, planning_areas: HashMap<String, PlanningArea>) -> Self {
        Self { name, planning_areas }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn combine_with(&mut self, region: Region) -> () {
        assert_eq!(self.name, region.name);

        for (code, pa) in region.planning_areas {
            match self.planning_areas.get_mut(&code) {
                Some(planning_area) => PlanningArea::combine_with(planning_area, pa),
                None => { self.planning_areas.insert(code, pa); }
            };
        }
    }
}