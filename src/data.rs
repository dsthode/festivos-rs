extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub struct DataError;

#[derive(Debug, Clone)]
pub struct Data {
    data: HashMap<String, Vec<String>>
}

impl Data {
    pub fn new() -> Result<Data, DataError> {
        let rx = Regex::new(r"^(\d{4})/(\d{2})/(\d{2})$").expect("failed to create regex");
        let mut data = Data { data: HashMap::new() };
        let file_data = fs::read_to_string("./festivos.json").expect("error reading file");
        let json_data: Vec<HashMap<String, Vec<String>>> = serde_json::from_str(&file_data).expect("failed to decode json");
        for item in json_data {
            for (key, value) in item {
                if rx.is_match(&key) {
                    let caps = rx.captures(&key).expect("failed capturing text");
                    let year = caps.get(1).unwrap().as_str().parse::<u32>().expect("failed parsing year");
                    let month = caps.get(2).unwrap().as_str().parse::<u32>().expect("failed parsing month");
                    let day = caps.get(3).unwrap().as_str().parse::<u32>().expect("failed parsing day");
                    data.add_date(year, month, day, value);
                }
            }
        }
        Ok(data)
    }

    fn add_date(&mut self, year: u32, month: u32, day: u32, locations: Vec<String>) {
        let entry = format!("{}-{}-{}", year, month, day);
        self.data.entry(entry).and_modify(|items| { locations.iter().for_each(|location| items.push(location.to_string())); }).or_insert(locations);
    }

    pub fn get_date(&self, year: u32, month: u32, day: u32) -> Option<&Vec<String>> {
        self.data.get(&format!("{}-{}-{}", year, month, day))
    }
}
