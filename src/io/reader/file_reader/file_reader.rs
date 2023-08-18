use std::fs;

use crate::io::reader::reader::Reader;

pub struct FileReader {}

impl Reader<String> for FileReader {
    fn read(string: &str) -> String {
        fs::read_to_string(string).expect("File could not be opened.")
    }
}