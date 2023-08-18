use regex::Regex;
use crate::geometry::shape::ShapeEnum;
use crate::io::reader::kml_reader::common::multi_polygon_reader::MultiPolygonReader;
use crate::io::reader::kml_reader::common::polygon_reader::PolygonReader;
use crate::io::reader::reader::Reader;

pub struct ShapeReader {}

impl Reader<ShapeEnum> for ShapeReader {
    fn read(string: &str) -> ShapeEnum {
        let multi_pattern_str: Regex = Regex::new("<MultiGeometry>(.*?)</MultiGeometry>").unwrap();
        let poly_pattern_str: Regex = Regex::new("<Polygon>(.*?)</Polygon>").unwrap();
        match multi_pattern_str.captures(string) {
            Some(capture) => {
                let (_, [geometries_str]) = capture.extract();
                ShapeEnum::MultiPolygon(MultiPolygonReader::read(geometries_str))
            },
            None => {
                let (_, [polygon_str]) = poly_pattern_str
                    .captures(string)
                    .expect("Polygon string capture failed.")
                    .extract();
                ShapeEnum::Polygon(PolygonReader::read(polygon_str))
            }
        }
    }
}