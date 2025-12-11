mod input;

const PAPER_CHAR: char = '@';
const DEBUG: bool = true;

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
    let input = input::read_input_2d_vec();
    println!("LP input: {input:?}");
    let mut count: usize = 0;
    for (row, line) in input.iter().enumerate() {
        for (col, char_) in line.iter().enumerate() {
            dbg2!("row={row}, col={col}, char={char_}");
            if *char_ != PAPER_CHAR {
                continue;
            }

            if paper_is_accessible(row, col, &input) {
                dbg2!("{char_}@({row},{col}) was accessible!");
                count += 1;
            }
        }
    }
    println!("Total count of accessible paper rolls is {count}");
}
