use std::collections::HashMap;
use regex::Regex;
use crate::feature::kml::Kml;
use crate::feature::subzone::subzone::Subzone;
use crate::io::reader::kml_reader::feature::subzone_reader::SubzoneReader;
use crate::io::reader::kml_reader::kml_reader::KmlReader;
use crate::io::reader::reader::Reader;

pub struct SubzoneKmlReader {}

impl KmlReader<Kml> for SubzoneKmlReader {
    fn get_pattern() -> Regex {
        Regex::new("(?P<data><Placemark.*?</Placemark>)").expect("Invalid regex given.")
    }
}

impl Reader<Kml> for SubzoneKmlReader {
    fn read(string: &str) -> Kml {
        let reg_exp: Regex = SubzoneKmlReader::get_pattern();
        let subzones_str: Vec<&str> = reg_exp
            .captures_iter(string)
            .map(|capture| {
                capture.name("data").unwrap().as_str()
            })
            .collect();

        let subzones: HashMap<String, Subzone> = subzones_str
            .into_iter()
            .map(SubzoneReader::read)
            .map(|subzone| (subzone.get_header(), subzone))
            .collect();

        Kml::new(subzones)
    }
}