use std::collections::HashMap;
use regex::Regex;
use crate::io::reader::reader::Reader;

pub struct FieldsReader {}

impl Reader<HashMap<String, String>> for FieldsReader {
    fn read(string: &str) -> HashMap<String, String> {
        let reg_exp: Regex = Regex::new(r#"<SimpleData name="(?<name>.*?)">(?<value>.*?)</SimpleData>"#).unwrap();
        reg_exp
            .captures_iter(string)
            .map(|capture| {
                let name: String = capture.name("name").unwrap().as_str().to_string();
                let value: String = capture.name("value").unwrap().as_str().to_string();
                (name, value)
            })
            .collect()
    }
}