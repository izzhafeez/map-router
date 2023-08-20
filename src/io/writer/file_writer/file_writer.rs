use std::fs::File;
use std::io::Write;
use crate::io::writer::writer::Writer;

pub struct FileWriter {}

impl Writer<&str> for FileWriter {
    fn write(s: &str, path: &str) -> Result<(), ()> {
        let mut file: File = File::create(path).expect(&format!("Can't create file at path {}.", path));
        file.write(s.as_ref()).expect(&format!("Can't write to {}.", path));
        Ok(())
    }
}