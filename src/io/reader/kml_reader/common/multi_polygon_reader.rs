use regex::Regex;
use crate::geometry::geometry::Geometry;
use crate::geometry::multi_polygon::MultiPolygon;
use crate::geometry::polygon::Polygon;
use crate::io::reader::kml_reader::common::polygon_reader::PolygonReader;
use crate::io::reader::reader::Reader;

pub struct MultiPolygonReader {}

impl Reader<MultiPolygon> for MultiPolygonReader {
    fn read(string: &str) -> MultiPolygon {
        let reg_exp: Regex = Regex::new("<Polygon>(.*?)</Polygon>").unwrap();
        let polygons_str: Vec<&str> = reg_exp
            .captures_iter(string)
            .map(|capture| capture.extract()) // Gets first capture.
            .map(|(_, [geometry_str])| geometry_str)
            .collect();

        let polygons: Vec<Polygon> = polygons_str
            .into_iter()
            .map(PolygonReader::read)
            .collect();

        MultiPolygon::from_polygons(polygons)
    }
}