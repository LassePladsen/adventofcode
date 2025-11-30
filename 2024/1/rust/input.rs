use std::fs;

pub fn read_cols(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(filename).unwrap();
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    for line in input.split("\n") {
        let mut nums = line.split_whitespace();
        if let Some(n) = nums.next() {
            col1.push(n.parse().unwrap());
        }
        if let Some(n) = nums.next() {
            col2.push(n.parse().unwrap());
        }
    }
    (col1, col2)
}
