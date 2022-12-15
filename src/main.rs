mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

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
        "day08" => day08::main(),
        "day09" => day09::main(),
        "day10" => day10::main(),
        "day11" => day11::main(),
        "day12" => day12::main(),
        "day13" => day13::main(),
        "day14" => day14::main(),
        "day15" => day15::main(),

        _ => {
            println!("Undefined day");
        }
    }
}