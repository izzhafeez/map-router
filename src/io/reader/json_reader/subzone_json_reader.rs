use crate::feature::kml::Kml;
use crate::io::reader::file_reader::file_reader::FileReader;
use crate::io::reader::reader::Reader;

pub struct SubzoneJsonReader {}

impl SubzoneJsonReader {
    pub fn get_path() -> String {
        String::from("./src/data/subzones.json")
    }
}

impl Reader<Kml> for SubzoneJsonReader {
    fn read(string: &str) -> Kml {
        let text: String = FileReader::read(string);
        serde_json::from_str::<Kml>(&text).expect("Can't parse json string for Kml.")
    }
}