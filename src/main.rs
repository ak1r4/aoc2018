mod day_1;
mod day_2;
mod utils;

fn main() {
    println!("day_1.1: {:?}", day_1::solve_1("data/day_1.txt"));
    println!("day_1.2: {:?}", day_1::solve_2("data/day_1.txt"));
    println!("day_2.2: {:?}", day_2::solve_1("data/day_2.txt"));
    println!("day_2.2: {:?}", day_2::solve_2("data/day_2.txt"));
}
