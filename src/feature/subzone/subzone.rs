use crate::geometry::shape::Shape;
use crate::feature::subzone_information::SubzoneInformation;

pub struct Subzone {
    information: SubzoneInformation,
    geometry: Box<dyn Shape>
}