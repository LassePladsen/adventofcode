use std::collections::HashMap;

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

        debugprint!("The range is {start}-{end}");

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
fn valid(id: u128) -> bool {
    const MAX_COUNT: usize = 2;
    let s = id.to_string();

    if s.is_empty() {
        panic!("Got an id that returned and empty string? Something unexpected, id={id}, str={s}");
    }

    // We know immidietely if the string length is odd, that is impossible to only consist of a pattern repeated only twice
    if s.len() % 2 != 0 {
        return true;
    }

    // We store the length and pattern of the string slice where we first encounter a nonrepeat
    let mut chars = s.chars();
    let mut prev = chars.next().unwrap();
    let mut strlen: Option<usize> = None;
    for (i, c) in s.chars().enumerate() {
        if c != prev {
            // we found the substring length
            strlen = Some(i + 1);
            break;
        }

        prev = c;
    }

    // THIS IS NOT TRUE, e.g. 11 would be an invalid, but it all has the same chars
    // If string consists of all same characters, then we know its valid immidietly
    // if strlen.is_none() {
    //     return true;
    // }

    // Specially handle where the strlen wasn't found OR its equal to the string length: this is like "11", "13". 11 is invalid, but 13 is valid.
    if strlen.is_none() || strlen.unwrap() == s.len() {
        // Is valid only if the whole string is not the same characters
        return s.chars().filter(|c| *c == prev).count() != s.len();
    }

    let strlen = strlen.unwrap();
    debugprint!("LP strlen: {strlen:?}");

    let mut char_counts = HashMap::<char, usize>::new();

    // Returns false if a count got too big, meaning the id is valid early!
    fn increment_char_count(char_counts: &mut HashMap<char, usize>, char: char) -> bool {
        let count = char_counts.entry(char).or_insert(0);
        *count += 1;
        if *count > MAX_COUNT {
            debugprint!("Saw too big count {count} for char {char}! Its valid early");
            return false;
        }
        return true;
    };

    // Continually split string at the found strlen, then count characters into the map
    let (mut split1, mut split2) = s.split_at(strlen);
    debugprint!("LP before loop split1: {split1:?}");
    debugprint!("LP before loop split2: {split2:?}");
    while split2.len() >= strlen {
        for char in split1.chars() {
            if !increment_char_count(&mut char_counts, char) {
                return true;
            }
        }

        (split1, split2) = split2.split_at(strlen);
        debugprint!("LP split1: {split1:?}");
        debugprint!("LP split2: {split2:?}");
    }
    // Do one more increment for the last split1, split2
    for char in split1.chars() {
        if !increment_char_count(&mut char_counts, char) {
            return true;
        }
    }
    if !split2.is_empty() {
        panic!("Split2 was not empty after loop!: {split2:?}");
    }

    debugprint!("LP char_counts: {char_counts:?}");

    return false;
}
