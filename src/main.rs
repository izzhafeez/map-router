extern crate core;

use crate::feature::kml::Kml;
use crate::io::reader::reader::Reader;
use crate::io::reader::file_reader::file_reader::FileReader;
use crate::io::reader::kml_reader::subzone_kml_reader::SubzoneKmlReader;

mod router;
mod feature;
mod io;
mod geometry;

fn main() {
    let text: String = FileReader::read("./src/data/master-plan-2019-subzone-boundary-no-sea-kml.kml");
    let subzones: Kml = SubzoneKmlReader::read(&text.replace("\n", ""));
}
