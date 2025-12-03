use input::read_cols;

mod input;

fn main() {
    println!("hei");
    let (col1, col2) = read_cols("../input");

    let mut sum = 0;

    // add to sum: num * how many times num is in col2
    for num in &col1 {
        let count: i32 = col2
            .iter()
            .filter(|x| *x == num)
            .count()
            .try_into()
            .unwrap();
        if count < 1 {
            continue;
        }
        sum += count * num;
    }
    println!("{}", sum)
}
