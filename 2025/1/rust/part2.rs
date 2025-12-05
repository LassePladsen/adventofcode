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
    let mut new_pos: isize;
    let mut resolved_pos: isize;
    let mut direction: char;
    let mut pos_diff: isize;
    for line in input.lines() {
        direction = line.chars().nth(0).unwrap();

        debugprint!("LP start pos: {pos}");

        debugprint!("LP direction: {:?}", direction);
        pos_diff = line[1..].parse().expect("Could not parse pos_diff");

        if direction == 'L' {
            pos_diff *= -1
        }

        debugprint!("LP pos_diff: {:?}", pos_diff);

        new_pos = pos + pos_diff;
        debugprint!("LP new pos before resolving: {new_pos:?}");

        // Modulo to resolve within bounds
        resolved_pos = new_pos % (MAX_POS + 1);

        // Wrap around 99 if negative
        if resolved_pos < MIN_POS {
            resolved_pos = MAX_POS + 1 + resolved_pos;
        }
        if resolved_pos < MIN_POS || resolved_pos > MAX_POS {
            panic!(
                "Final new new_pos is outside bounds with new_pos={new_pos}, min={MIN_POS}, max={MAX_POS}"
            );
        }
        println!("LP resolved_pos: {resolved_pos:?}");

        // Number of times passed zero is the diff between new_pos before and after resolving back inside bounds, divided by 100.
        // But we need to handle the edge case where we started/landed at exactly zero
        this_turn_hit_zero_count = (resolved_pos - new_pos).abs() / (MAX_POS + 1);
        println!(
            "LP this_turn_hit_zero_count before resolving edge cases at zero: {this_turn_hit_zero_count:?}"
        );

        // If started at zero exactly, but didnt make it across the entire wheel back to
        // zero
        if pos == 0 && pos_diff < (MAX_POS + 1) && this_turn_hit_zero_count >= 1 {
            this_turn_hit_zero_count -= 1;
        }
        // If landed at zero exactly
        if resolved_pos == 0 && this_turn_hit_zero_count == 0 {
            this_turn_hit_zero_count += 1;
        }

        if this_turn_hit_zero_count < 0 {
            panic!("This_turn_hit_zero_count can't be negative! got {this_turn_hit_zero_count}");
        }

        debugprint!("LP this_turn_hit_zero_count: {this_turn_hit_zero_count:?}\n");

        hit_zero_count += this_turn_hit_zero_count as usize;
        pos = resolved_pos;
    }

    hit_zero_count
}

fn main() {
    let input = input::read_input("../input.txt");

    // if DEBUG {
    //     println!("Input is:");
    //     input.lines().for_each(|x| println!("{x}"));
    //     println!();
    // }

    let password = parse_password(&input);
    println!("The password is {password}");
}
