use crate::input::read_input;

mod input;

const DEBUG: bool = false;

macro_rules! dbg2 {
    ($($arg:tt)*) => {
        if DEBUG {
            println!($($arg)*);
        }
    };
}

fn main() {
    let input = read_input();

    // Each line is a bank, where each digit is a battery with that number of jolts
    // We need to find the largest jolt combination of two batteries, i.e the l
    let mut total_largest: u32 = 0;
    for line in input.lines() {
        dbg2!("line: {line:?}");

        let mut largest: u32 = 0;
        let digits: Vec<char> = line.chars().collect(); // collect since we will iter it multiple times
        for (i, digitone) in digits.iter().enumerate() {
            dbg2!("digitone: {digitone}");
            for digittwo in digits[i + 1..].iter() {
                dbg2!("digittwo: {digittwo}");

                let combined_int = format!("{digitone}{digittwo}").parse().unwrap();
                if combined_int > largest {
                    largest = combined_int;
                    dbg2!("This was larger, updated largest to {largest}");
                }
            }
            dbg2!("After nested loop, largest={largest}");
        }
        dbg2!();

        total_largest += largest;
    }
    println!("Total max jolts is {total_largest}");
}
