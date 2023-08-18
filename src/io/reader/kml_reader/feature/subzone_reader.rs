use std::collections::HashMap;
use regex::{Captures, Regex};
use crate::feature::subzone::subzone::Subzone;
use crate::geometry::shape::ShapeEnum;
use crate::io::reader::kml_reader::common::field_reader::FieldsReader;
use crate::io::reader::kml_reader::common::shape_reader::ShapeReader;
use crate::io::reader::reader::Reader;

pub struct SubzoneReader {}

impl Reader<Subzone> for SubzoneReader {
    fn read(string: &str) -> Subzone {
        let information_str_regex: Regex = Regex::new("<ExtendedData>(?<information_str>.*?)</ExtendedData>(?<geometry_str>.*?)").unwrap();
        let capture: Captures = information_str_regex.captures(string).unwrap();
        let information_str: &str = capture.name("information_str").unwrap().as_str();
        let geometry_str: &str = capture.name("geometry_str").unwrap().as_str();

        let fields: HashMap<String, String> = FieldsReader::read(information_str);
        let id: i16 = fields
            .get("SUBZONE_NO")
            .expect("Subzone has no number.")
            .parse::<i16>()
            .expect("Subzone number is not an integer.");
        let name: String = fields
            .get("SUBZONE_N")
            .expect("Subzone has no name.")
            .to_string();
        let code: String = fields
            .get("SUBZONE_C")
            .expect("Subzone has no code.")
            .to_string();
        let planning_area: String = fields
            .get("PLN_AREA_N")
            .expect("Subzone has no planning area.")
            .to_string();
        let planning_area_code: String = fields
            .get("PLN_AREA_C")
            .expect("Subzone has no planning area code.")
            .to_string();
        let region: String = fields
            .get("REGION_N")
            .expect("Subzone has no region.")
            .to_string();
        let region_code: String = fields
            .get("REGION_C")
            .expect("Subzone has no region code.")
            .to_string();

        let geometry: ShapeEnum = ShapeReader::read(geometry_str);

        Subzone::new(id, name, code, planning_area, planning_area_code, region, region_code, geometry)
    }
}