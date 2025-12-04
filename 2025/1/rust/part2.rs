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
    const MAX_POS: isize = 99;
    const MIN_POS: isize = 0;

    let mut hit_zero_count: usize = 0;
    let mut this_turn_hit_zero_count: isize;
    let mut pos: isize = START_POS;
    let mut new_pos: isize = START_POS;
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
        debugprint!("LP new pos before resolving: {pos:?}");

        // Modulo to resolve within bounds
        new_pos = new_pos % (MAX_POS + 1);

        // Wrap around 99 if negative
        if new_pos < MIN_POS {
            new_pos = MAX_POS + 1 + new_pos;
        }
        debugprint!("LP final new new_pos: {new_pos:?}");
        if new_pos < MIN_POS || new_pos > MAX_POS {
            panic!(
                "Final new new_pos is outside bounds with new_pos={new_pos}, min={MIN_POS}, max={MAX_POS}"
            );
        }

        this_turn_hit_zero_count = pos / (MAX_POS + 1);
        println!("LP this_turn_hit_zero_count before resolving landing at zero: {this_turn_hit_zero_count:?}");
        if this_turn_hit_zero_count < 0 {
            panic!("This_turn_hit_zero_count can't be negative! got {this_turn_hit_zero_count}");
        }
        // If landed at zero exactly
        if new_pos == 0 && pos != 0 {
            this_turn_hit_zero_count += 1;
        }
        debugprint!("LP this_turn_hit_zero_count: {this_turn_hit_zero_count:?}\n");

        hit_zero_count += this_turn_hit_zero_count as usize;
        pos = new_pos
    }

    hit_zero_count
}

fn main() {
    let input = input::read_input("../demoinput.txt");

    if DEBUG {
        println!("Input is:");
        input.lines().for_each(|x| println!("{x}"));
        println!();
    }

    let password = parse_password(&input);
    println!("The password is {password}");
}
