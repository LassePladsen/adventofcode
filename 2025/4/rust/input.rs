use std::{env, fs};

fn read_filepath() -> String {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(s) => s,
        None => "../input.txt".to_string(),
    }
}

pub fn read_input() -> String {
    let filepath = read_filepath();
    let input = fs::read_to_string(&filepath).expect(&format!(
        "Could not open file '{filepath}'! Make sure it exists and you have read permissions"
    ));
    input
}

pub fn read_input_2d_vec() -> Vec<Vec<char>> {
    let input = read_input();
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}
