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

pub fn read_id_ranges() -> Vec<String> {
    read_input()
        .split(',')
        .map(|s| s.to_string().trim().to_string())
        .collect()
}
