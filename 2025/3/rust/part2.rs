use crate::input::read_input;

mod input;

/// Each line is a bank, where each digit is a battery with that number of jolts
/// We need to find the largest jolt combination of 12 batteries, i.e the string of chars
/// that make the largest jolt of 12 digits.
/// Do this recursively.
fn get_largest_jolt(line: &str) -> u128 {
    if line.len() < 12 {
        panic!("Line length less than 12!?");
    }
    fn iter(s: &str, largest_jolt: u128, jolt: u128) -> u128 {
        // done
        if s.len() == 1 {
            return jolt;
        }

        if s.chars().nth(1).unwrap() {

        }

        iter(dbg!(&s[1..]), largest_jolt, dbg!(jolt))
    }

    // init jolt to compare against as first 12
    let init_jolt: u128 = dbg!(&line[..12]).parse().unwrap();
    iter(dbg!(&line[1..]), dbg!(init_jolt), 0)
}

fn main() {
    let input = read_input();

    let mut total_largest = 0;
    for line in input.lines() {
        println!("\nLP line: {line:?}");
        total_largest += dbg!(get_largest_jolt(line));
    }
    println!("Total max jolts is {total_largest}");
}
