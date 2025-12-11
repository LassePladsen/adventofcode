mod input;

const PAPER_CHAR: char = '@';
const MAX_ADJECENT_PAPERS: usize = 3;
const DEBUG: bool = true;

macro_rules! dbg2 {
    ($($arg:tt)*) => {
        if DEBUG {
            println!($($arg)*);
        }
    };
}

fn adjacent_chars(row: usize, col: usize, input: &str) -> Vec<char> {
    todo!()

}

fn main() {
    let input = crate::input::read_input_2d_vec();
    // for (row, line) in input.iter().enumerate() {
    //     for (col, char_) in line.iter().enumerate() {
    //         dbg2!("row={row}, col={col}, char={char_}");
    //         if char_ != PAPER_CHAR {
    //             continue;
    //         }

    //         let mut adj_count: usize = 0;
    //         // Count adjecent paper rolls
    //         for adj_char in adjacent_chars(row, col, &input) {
    //             dbg2!("LP adj_char: {adj_char:?}");
    //             if adj_char == PAPER_CHAR {
    //                 adj_count += 1;
    //             }
    //         }

    //     }

    // }

}
