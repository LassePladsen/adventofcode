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
    let input = input::read_input(Some("../input.txt"));

    if DEBUG {
        println!("Input is:");
        input.lines().for_each(|x| println!("{x}"));
        println!();
    }

    let password = parse_password(&input);
    println!("The password is {password}");
}

/// Password is the number of times the dial is poining at zero (after a rotation)
fn parse_password(input: &String) -> usize {
    const START_POS: isize = 50;
    const TARGET_POS: isize = 0;
    const MAX_POS: isize = 99;

    let mut at_zero_count: usize = 0;
    let mut pos: isize = START_POS;
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

        pos += pos_diff;
        debugprint!("LP new pos before resolving: {pos:?}");
        if pos > MAX_POS {
            pos = pos - (MAX_POS + 1)
        } else if pos < 0 {
            pos = MAX_POS + 1 + pos
        }

        debugprint!("LP final new pos: {pos:?}\n");
        if pos == TARGET_POS {
            at_zero_count += 1;
        }
    }

    at_zero_count
}
