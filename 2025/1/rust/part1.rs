use core::panic;

mod input;

const DEBUG: bool = true;

macro_rules! debugprint {
    ($($arg:tt)*) => {
        if DEBUG {
            println!($($arg)*);
        }
    };
}

/// Password is the number of times the dial has pointed at zero after a rotation
fn parse_password(input: &String) -> usize {
    const START_POS: isize = 50;
    const TARGET_POS: isize = 0;
    const MAX_POS: isize = 99;
    const MIN_POS: isize = 0;

    let mut at_zero_count: usize = 0;
    let mut pos: isize = START_POS;
    let mut new_pos: isize;
    let mut resolved_pos: isize;
    let mut direction: char;
    let mut pos_diff: isize;
    for line in input.lines() {
        direction = line.chars().nth(0).unwrap();

        debugprint!("LP direction: {:?}", direction);
        pos_diff = line[1..].parse().expect("Could not parse pos_diff");

        if direction == 'L' {
            pos_diff *= -1
        }

        debugprint!("LP pos_diff: {:?}", pos_diff);

        new_pos = pos + pos_diff;
        debugprint!("LP new pos before resolving: {new_pos:?}");

        // Modulo to resolve within bounds
        // Solve negative by adding a bit number
        resolved_pos = (new_pos + 10000000) % (MAX_POS + 1);

        debugprint!("LP resolved new pos: {pos:?}\n");

        if resolved_pos < MIN_POS || resolved_pos > MAX_POS {
            panic!("Final new pos is outside bounds!");
        }

        if resolved_pos == TARGET_POS {
            at_zero_count += 1;
        }
        pos = resolved_pos;
    }

    at_zero_count
}

fn main() {
    let input = input::read_input();

    if DEBUG {
        println!("Input is:");
        input.lines().for_each(|x| println!("{x}"));
        println!();
    }

    let password = parse_password(&input);
    println!("The password is {password}");
}
