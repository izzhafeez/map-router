use regex::Regex;
use crate::io::reader::kml_reader::subzone_kml_reader::SubzoneKmlReader;
use crate::io::reader::reader::Reader;

pub enum KmlReaderEnum {
    Subzone(SubzoneKmlReader)
}

pub trait KmlReader<T> : Reader<T> {
    fn get_pattern() -> Regex;
}