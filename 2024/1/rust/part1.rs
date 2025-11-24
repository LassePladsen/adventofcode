use std::fs;

fn read_cols(filename: &str) -> (Vec<i32>, Vec<i32>) {
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

fn get_col_diffs(col1: Vec<i32>, col2: Vec<i32>) -> Vec<u32> {
    col1.iter()
        .zip(col2)
        .map(|(num1, num2)| num1.abs_diff(num2))
        .collect()
}

fn main() {
    let (mut col1, mut col2) = read_cols("../input");

    col1.sort();
    col2.sort();

    println!("{}", 
        get_col_diffs(col1, col2)
        .iter()
        .sum::<u32>()
    );
}
