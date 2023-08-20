extern crate core;

use crate::feature::kml::Kml;
use crate::io::reader::reader::Reader;
use crate::io::reader::file_reader::file_reader::FileReader;
use crate::io::reader::json_reader::subzone_json_reader::SubzoneJsonReader;
use crate::io::reader::kml_reader::subzone_kml_reader::SubzoneKmlReader;
use crate::io::writer::json_writer::subzone_json_writer::SubzoneJsonWriter;
use crate::io::writer::writer::Writer;

mod router;
mod feature;
mod io;
mod geometry;

fn main() {
    let kml: Kml = SubzoneJsonReader::read(&SubzoneJsonReader::get_path());
    println!("{:?}", kml);
}
