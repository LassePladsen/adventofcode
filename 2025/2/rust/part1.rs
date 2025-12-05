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
    let mut prev_char: Option<char>;
    let mut invalid_sum: u128 = 0;
    let mut invalid_id: String;

    for range in id_ranges {
        for id in range.split("-") {
            debugprint!("id is {id}");

            prev_char = None;
            invalid_id = "".to_string();
            for cur in id.chars() {
                if let None = prev_char {
                    prev_char = Some(cur);
                    continue;
                }
                debugprint!("prev={}, cur={cur}", prev_char.unwrap());

                // Invalid if any digit is repeated
                if cur == prev_char.unwrap() {
                    // invalid += cur.to_digit(10).expect("Could not parse char to digit");
                    debugprint!("We had a duplicate of the char {cur}, pushing it to invalid_id!");
                    invalid_id = format!("{invalid_id}{}{cur}", prev_char.unwrap());
                    debugprint!("LP invalid_id: {invalid_id:?}");
                }

                prev_char = Some(cur);
            }

            if !invalid_id.is_empty() {
                debugprint!("For id={id} we got invalid id of {invalid_id:?}, adding to sum");
                invalid_sum += invalid_id
                    .parse::<u128>()
                    .expect("Error parsing '{invalid_id}' to int");
                debugprint!("LP invalid_sum: {invalid_sum:?}");
            }
            debugprint!();
        }
    }

    println!("Total sum of invalid ids: {invalid_sum}");
}
