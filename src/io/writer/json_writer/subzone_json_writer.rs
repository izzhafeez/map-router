use std::fs::File;
use std::io::{BufWriter, Write};
use crate::feature::kml::Kml;
use crate::io::writer::writer::Writer;

pub struct SubzoneJsonWriter {}

impl Writer<Kml> for SubzoneJsonWriter {
    fn write(s: Kml, path: &str) -> Result<(), ()> {
        let file: File = File::create(path).expect(&format!("Can't create file at path {}.", path));
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &s).expect(&format!("Unable to write at path {}.", path));
        writer.flush().expect("Unable to flush.");
        Ok(())
    }
}

