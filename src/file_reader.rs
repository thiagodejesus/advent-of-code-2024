use std::fs;

pub fn read_file_to_string(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("File not found")
}
