mod part1;
mod part2;

fn main() {
    let input = include_str!("./data.txt");

    println!("task 1: {}", part1::task_1(input));
    println!("task 2: {}", part2::task_2(input));
}