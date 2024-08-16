use std::{
    fs::{self, File},
    io::Read,
};

use serde_json::{json, Map, Value};

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();

    return json.as_object().unwrap().clone();
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    fs::write(file_name.to_string(), json!(state).to_string()).expect("Unable to write file!");
}
