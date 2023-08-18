use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubzoneInformation {
    id: i16,
    name: String,
    code: String,
    planning_area: String,
    planning_area_code: String,
    region: String,
    region_code: String
}