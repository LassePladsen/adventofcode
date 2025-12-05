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

            if invalid(id) {
                debugprint!("Id {id} is invalid! Adding to sum");
                invalid_sum += id;
                debugprint!("LP new invalid_sum: {invalid_sum:?}");
            }
            debugprint!();
        }
    }

    println!("Total sum of invalid ids: {invalid_sum}");
}

/// An id is invalid as long as it only consists of some sequence of digits repeated exactly once.
/// TODO: needs to check not only one character at a time, but all possible substrings.
/// i.e. 123123 is invalid cause its 123 twice
/// Plan is to maybe store all combinations of substrings (all different lenghts possible) in a
/// hashmap, create a count and somehow parse the hashmap to find if can be marked as invalid.
fn invalid(id: u128) -> bool {
    let s = id.to_string();
    // Track count of a string slice in the id
    type DigitMap<'a> = HashMap<&'a str, usize>;

    // For all possible substring lengths, keep track of the substring count in this id
    // Key is length of substring
    let mut substr_maps: HashMap<usize, DigitMap> = HashMap::new();

    // Loop over possible substring lengths
    // TODO: we might only need to do up to and including half the length?
    for strlen in 1..s.len() {
        // Populate this substring length with all substrings
        // Key is substring, value is the count
        let mut substr_counts: DigitMap = HashMap::new();

        for substr in s.splitn(strlen, substr) {
            substr_counts.insert(substr, s.matches(substr).count());
        }
        substr_maps.insert(strlen, substr_counts);
    }

    for c in id.to_string().chars() {
        let count = digit_counts.entry(c).or_insert(0);
        *count += 1;

        // We know immediately this is valid if seen more than twice
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
