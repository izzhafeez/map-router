use crate::io::reader::reader::Reader;
use crate::geometry::coord::Coord;

pub struct CoordsReader {}

impl Reader<Vec<Coord>> for CoordsReader {
    fn read(string: &str) -> Vec<Coord> {
        string
            .split(" ")
            .map(CoordReader::read)
            .collect()
    }
}

struct CoordReader {}

impl Reader<Coord> for CoordReader {
    fn read(string: &str) -> Coord {
        let mut coord_iter = string.split(",");

        let lat: f32 = coord_iter
            .next()
            .expect("Lat not found.")
            .parse::<f32>()
            .expect("Lat is not a float.");

        let lng: f32 = coord_iter
            .next()
            .expect("Lng not found.")
            .parse::<f32>()
            .expect("Lng is not a float.");

        let alt: f32 = coord_iter
            .next()
            .expect("Alt not found.")
            .parse::<f32>()
            .expect("Alt is not a float.");

        Coord::new(lat, lng, alt)
    }
}