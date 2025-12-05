use std::collections::{HashMap, btree_map::Values};

mod input;

const DEBUG: bool = true;

macro_rules! debugprint {
    ($($arg:tt)*) => {
        if DEBUG {
            println!($($arg)*);
        }
    };
}

fn main() {
    let id_ranges = input::read_id_ranges();
    debugprint!("LP id_ranges: {id_ranges:?}\n");
    let mut invalid_sum: u128 = 0;

    for range in id_ranges {
        let mut range_iter = range.split("-");
        let start: u128 = range_iter
            .next()
            .unwrap()
            .parse()
            .expect("Error parsing range start to int");
        let end: u128 = range_iter
            .next()
            .unwrap()
            .parse()
            .expect("Error parsing range end to int");

        // Loop over every id in the range, check for invalids
        for id in start..end + 1 {
            debugprint!("id is {id}");

            if !valid(id) {
                debugprint!("Id {id} is invalid! Adding to sum");
                invalid_sum += id;
                debugprint!("LP new invalid_sum: {invalid_sum:?}");
            }
            debugprint!();
        }
    }

    println!("Total sum of invalid ids: {invalid_sum}");
}

/// An id is invalid if it only consists of some sequence of digits repeated exactly once (i.e. shows up twice)
/// To decide what length the substring we should check, we iterate through the digits until we hit a new one.
fn valid(id: u128) -> bool {
    let s = id.to_string();

    if s.is_empty() {
        panic!("Got an id that returned and empty string? Something unexpected, id={id}, str={s}");
    }

    let mut chars = s.chars();
    let mut prev = chars.next().unwrap();
    let mut strlen: Option<usize> = None;

    for (i, c) in s.chars().enumerate() {
        if c != prev {
            // we found the substring length
            strlen = Some(i);
            break;
        }

        prev = c;
    }

    // If string consists of all same characters, then we know its valid
    if strlen.is_none() {
        return true;
    }
    strlen = strlen.unwrap();

    // Now that we have the substring length we wish to use, parse through the string with this length and check for duplicate counts
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    "test".split(pat);
    return false;
}
