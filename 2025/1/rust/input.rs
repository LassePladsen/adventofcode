use std::fs;

pub fn read_input(file_path: &str) -> String {
    let input = fs::read_to_string(file_path).expect(&format!(
        "Could not open file '{file_path}'! Make sure it exists and you have read permissions"
    ));
    input
}
