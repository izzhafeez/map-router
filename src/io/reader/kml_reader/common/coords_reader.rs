use crate::io::reader::reader::Reader;
use crate::geometry::coord::Coord;

pub struct CoordsReader {}

impl Reader<Vec<Coord>> for CoordsReader {
    fn read(string: &str) -> Vec<Coord> {
        string
            .split(" ")
            .filter_map(|coord_str| CoordsReader::read_coord(coord_str).ok())
            .collect()
    }
}

impl CoordsReader {
    fn read_coord(coord_str: &str) -> Result<Coord, ()> {
        let mut coord_iter = coord_str.split(",");

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

        Ok(Coord::new(lat, lng, alt))
    }
}