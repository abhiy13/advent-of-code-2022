extern crate core;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

mod reader;

fn main() {
    day5::solve::solve().expect("Failed");
}