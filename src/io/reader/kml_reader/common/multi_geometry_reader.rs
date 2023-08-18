use regex::Regex;
use crate::geometry::geometry::Geometry;
use crate::geometry::multi_geometry::MultiGeometry;
use crate::geometry::polygon::Polygon;
use crate::io::reader::kml_reader::common::polygon_reader::PolygonReader;
use crate::io::reader::reader::Reader;

struct MultiGeometryReader {}

impl Reader<MultiGeometry> for MultiGeometryReader {
    fn read(string: &str) -> MultiGeometry {
        let reg_exp: Regex = Regex::new("<Polygon>(.*?)</Polygon>").unwrap();
        let geometries_str: Vec<&str> = reg_exp
            .captures_iter(string)
            .map(|capture| capture.extract()) // Gets first capture.
            .map(|(_, [geometry_str])| geometry_str)
            .collect();

        let geometries: Vec<Box<dyn Geometry>> = geometries_str
            .into_iter()
            .map(PolygonReader::read)
            .map(Box::new)
            .map(|b| b as Box<dyn Geometry>)
            .collect();

        MultiGeometry::from_geometries(geometries)
    }
}