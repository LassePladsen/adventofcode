use std::fs;

pub fn read_input(file_path: Option<&str>) -> String {
    let file_path = file_path.unwrap_or("../input.txt");
    let input = fs::read_to_string(file_path).expect(
        "Could not open file {file_path}! Make sure it exists and you have read permissions",
    );
    input
}
