use regex::Regex;
use crate::geometry::coord::Coord;

use crate::geometry::polygon::Polygon;
use crate::io::reader::kml_reader::common::coords_reader::CoordsReader;
use crate::io::reader::reader::Reader;

pub struct PolygonReader {}

impl Reader<Polygon> for PolygonReader {
    fn read(string: &str) -> Polygon {
        let reg_exp: Regex = Regex::new("<coordinates>(.*?)</coordinates>").unwrap();

        let boundaries_str: Vec<&str> = reg_exp
            .captures_iter(string)
            .map(|capture| capture.extract()) // Gets first capture.
            .map(|(_, [boundary_str])| boundary_str)
            .collect();

        let boundaries: Vec<Vec<Coord>> = boundaries_str
            .into_iter()
            .map(CoordsReader::read)
            .collect();

        Polygon::from_boundaries(boundaries)
    }
}