mod input;

const DEBUG: bool = true;

macro_rules! debugprint {
    ($($arg:tt)*) => {
        if DEBUG {
            println!($($arg)*);
        }
    };
}

/// Password is the number of times the dial has rotated at or past zero
fn parse_password(input: &String) -> usize {
    const START_POS: isize = 50;
    const TARGET_POS: isize = 0;
    const MAX_POS: isize = 99;
    const MIN_POS: isize = 0;

    let mut hit_zero_count: usize = 0;
    let mut iter_hit_zero_count: usize;
    let mut pos: isize = START_POS;
    let mut new_pos: isize;
    let mut resolved_pos: isize;
    let mut direction: char;
    let mut rotate_by: isize;
    for line in input.lines() {
        iter_hit_zero_count = 0;
        direction = line.chars().nth(0).unwrap();

        debugprint!("LP start pos: {pos}");

        debugprint!("LP direction: {:?}", direction);
        rotate_by = line[1..].parse().expect("Could not parse pos_diff");

        debugprint!("LP rotate_by: {:?}", rotate_by);

        // Do one turn at a time so we can easily use solution from part1
        for _ in 0..rotate_by.abs() {
            match direction {
                'L' => new_pos = pos - 1,
                'R' => new_pos = pos + 1,
                d => panic!("Got unknown direction {d}"),
            }

            // Modulo to resolve within bounds
            // Solve negative by adding a bit number
            resolved_pos = (new_pos + 10000000) % (MAX_POS + 1);
            // debugprint!("LP new_pos: {new_pos:?}");

            if resolved_pos < MIN_POS || resolved_pos > MAX_POS {
                panic!(
                    "Final new new_pos is outside bounds with new_pos={new_pos}, min={MIN_POS}, max={MAX_POS}"
                );
            }

            if resolved_pos == TARGET_POS {
                iter_hit_zero_count += 1;
            }
            pos = resolved_pos;
        }
        debugprint!("Hit zero {iter_hit_zero_count} times");
        debugprint!("New resolved pos after loop turning: {pos}\n");
        hit_zero_count += iter_hit_zero_count;
    }
    hit_zero_count
}

fn main() {
    let input = input::read_input();

    // if DEBUG {
    //     println!("Input is:");
    //     input.lines().for_each(|x| println!("{x}"));
    //     println!();
    // }

    let password = parse_password(&input);
    println!("The password is {password}");
}
