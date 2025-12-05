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
            .expect("Error parsing range to int");
        let end: u128 = range_iter
            .next()
            .unwrap()
            .parse()
            .expect("Error parsing range to int");

        // Loop over every id in the range, check for invalids
        for id in start..end + 1 {
            debugprint!("id is {id}");

            if is_invalid(id) {
                debugprint!("Id {id} is invalid! Adding to sum");
                invalid_sum += id;
                debugprint!("LP new invalid_sum: {invalid_sum:?}");
            }
            debugprint!();
        }
    }

    println!("Total sum of invalid ids: {invalid_sum}");
}

/// An id is invalid if it consists only of chars that are duplicated exactly once
/// TODO: needs to check not only one character at a time, but possible substring.
/// i.e. 123123 is invalid cause its 123 twice
fn is_invalid(id: u128) -> bool {
    // Track count of every digit found
    let mut digit_counts: HashMap<char, usize> = HashMap::new();

    for c in id.to_string().chars() {
        let count = digit_counts.entry(c).or_insert(0);
        *count += 1;

        // We know early this is valid if seen more than twice
        if *count > 2 {
            return false;
        }
        debugprint!("LP digit_counts: {digit_counts:?}");
    }

    for count in digit_counts.values() {
        if *count != 2 {
            return false;
        }
    }
    return true;
}
