mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let day = std::env::args().nth(1).expect("Please provide day as a CLI argument");

    match day.trim() {
        "day01" => day01::main(),
        "day02" => day02::main(),
        "day03" => day03::main(),
        "day04" => day04::main(),
        "day05" => day05::main(),
        "day06" => day06::main(),
        "day07" => day07::main(),
        _ => {
            println!("Undefined day");
        }
    }
}