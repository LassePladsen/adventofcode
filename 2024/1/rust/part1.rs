use input::read_cols;

mod input;

fn get_col_diffs(col1: Vec<i32>, col2: Vec<i32>) -> Vec<u32> {
    col1.iter()
        .zip(col2)
        .map(|(num1, num2)| num1.abs_diff(num2))
        .collect()
}

fn main() {
    let (mut col1, mut col2) = read_cols("../input");

    col1.sort();
    col2.sort();

    println!("{}", get_col_diffs(col1, col2).iter().sum::<u32>());
}
