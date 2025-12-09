use fancy_regex::Regex;
use rayon::prelude::*;

mod input;

const DEBUG: bool = false;

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
        // for id in start..end + 1 {
        //     debugprint!("id is {id}");
        //     if !valid(id) {
        //         debugprint!("Id {id} is invalid! Adding to sum");
        //         invalid_sum += id;
        //         debugprint!("LP new invalid_sum: {invalid_sum:?}");
        //     }
        //     debugprint!();
        // }

        // Above is SLOOOOW. Batch it into parallell threads instead
        invalid_sum += (start..end+ 1).into_par_iter().map(|id| {
            if !valid(id) {
                return id;
            }
            return 0;
        }).sum::<u128>();
    }

    println!("Total sum of invalid ids: {invalid_sum}");
}

fn valid(id: u128) -> bool {
    // save regex time by knowing its valid early if the string length is odd
    let s = id.to_string();
    if s.len() % 2 == 2 {
        return true;
    }

    let re = Regex::new(r"^(\d+)\1$").expect("Invalid regex");
    match re.captures(&s).expect("Error running regex") {
        Some(captures) => {
            debugprint!("{} captures: {captures:?}", captures.len());
            return captures.len() != 2;
        }

        None => {
            debugprint!("No captures, its valid!");
            return true;
        }
    }
}
