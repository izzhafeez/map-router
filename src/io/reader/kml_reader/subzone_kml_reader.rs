use std::collections::HashMap;
use regex::Regex;
use crate::feature::kml::Kml;
use crate::feature::shape::region::Region;
use crate::feature::shape::subzone::Subzone;
use crate::io::reader::kml_reader::feature::region_reader::RegionReader;
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
        let regions_str: Vec<&str> = reg_exp
            .captures_iter(string)
            .map(|capture| {
                capture.name("data").unwrap().as_str()
            })
            .collect();

        let mut regions: HashMap<String, Region> = HashMap::new();
        for (code, r) in regions_str.into_iter().map(RegionReader::read) {
            match regions.get_mut(&code) {
                Some(region) => region.combine_with(r),
                None => { regions.insert(code, r); }
            }
        }

        Kml::new(regions)
    }
}