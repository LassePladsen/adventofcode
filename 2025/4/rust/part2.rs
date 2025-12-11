mod input;

const PAPER_CHAR: char = '@';
const EMPTY_CHAR: char = '.';
const DEBUG: bool = false;

macro_rules! dbg2 {
    ($($arg:tt)*) => {
        if DEBUG {
            println!($($arg)*);
        }
    };
}

fn paper_is_accessible(row: usize, col: usize, input: &Vec<Vec<char>>) -> bool {
    const MAX_ADJACENT_PAPERS: usize = 3;

    let n_rows = input.len();
    let n_cols = input[0].len();
    let mut adj_count = 0;

    // Top left
    if row > 0 && col > 0 && input[row - 1][col - 1] == PAPER_CHAR {
        dbg2!("Top left is a paper roll");
        adj_count += 1;
        if adj_count > MAX_ADJACENT_PAPERS {
            return false;
        }
    }
    // Top
    if row > 0 && input[row - 1][col] == PAPER_CHAR {
        dbg2!("Top is a paper roll");
        adj_count += 1;
        if adj_count > MAX_ADJACENT_PAPERS {
            return false;
        }
    }
    // Top right
    if row > 0 && col < n_cols - 1 && input[row - 1][col + 1] == PAPER_CHAR {
        dbg2!("Top right is a paper roll");
        adj_count += 1;
        if adj_count > MAX_ADJACENT_PAPERS {
            return false;
        }
    }
    // left
    if col > 0 && input[row][col - 1] == PAPER_CHAR {
        dbg2!("left is a paper roll");
        adj_count += 1;
        if adj_count > MAX_ADJACENT_PAPERS {
            return false;
        }
    }
    // right
    if col < n_cols - 1 && input[row][col + 1] == PAPER_CHAR {
        dbg2!("right is a paper roll");
        adj_count += 1;
        if adj_count > MAX_ADJACENT_PAPERS {
            return false;
        }
    }
    // bottom left
    if row < n_rows - 1 && col > 0 && input[row + 1][col - 1] == PAPER_CHAR {
        dbg2!("bottom left is a paper roll");
        adj_count += 1;
        if adj_count > MAX_ADJACENT_PAPERS {
            return false;
        }
    }
    // bottom
    if row < n_rows - 1 && input[row + 1][col] == PAPER_CHAR {
        dbg2!("bottom is a paper roll");
        adj_count += 1;
        if adj_count > MAX_ADJACENT_PAPERS {
            return false;
        }
    }
    // bottom right
    if row < n_rows - 1 && col < n_cols - 1 && input[row + 1][col + 1] == PAPER_CHAR {
        dbg2!("bottom right is a paper roll");
        adj_count += 1;
        if adj_count > MAX_ADJACENT_PAPERS {
            return false;
        }
    }

    true
}

fn main() {
    let mut input = input::read_input_2d_vec();
    dbg2!("LP input: {input:?}");
    let mut count: usize = 0;
    let mut to_be_removed: Vec<(usize, usize)> = Vec::new();

    // Do iterations until we are done removing all we can
    'iterloop: loop {
        for (row, line) in input.iter().enumerate() {
            for (col, char_) in line.iter().enumerate() {
                dbg2!("row={row}, col={col}, char={char_}");
                if *char_ != PAPER_CHAR {
                    continue;
                }

                if paper_is_accessible(row, col, &input) {
                    dbg2!("{char_}@({row},{col}) was accessible, we're removing it next iter!");
                    count += 1;
                    to_be_removed.push((row, col));
                }
            }
        }

        // Done parsing the entire input, remove the accessible papers and do another iteration
        // But we are finished if there is nothing more to remove
        if to_be_removed.is_empty() {
            dbg2!("We're finished, nothing more to remove!");
            break 'iterloop;
        }

        dbg2!("We're now looping over and removing these indices: {to_be_removed:?}");
        for (row, col) in &to_be_removed {
            input[*row][*col] = EMPTY_CHAR;
        }
        to_be_removed.clear();

    }
    println!("Total count of removed paper rolls is {count}");
}
