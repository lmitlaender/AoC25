pub trait Day {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

use std::vec;

use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;
use day06::Day06;
use day07::Day07;


pub fn get_days() -> Vec<(i32, Box<dyn Day>)> {
    let mut map: Vec<(i32, Box<dyn Day>)> = vec![];
    
    // Add all days
    map.push((1, Box::new(Day01)));
    map.push((2, Box::new(Day02)));
    map.push((3, Box::new(Day03)));
    map.push((4, Box::new(Day04)));
    map.push((5, Box::new(Day05)));
    map.push((6, Box::new(Day06)));
    map.push((7, Box::new(Day07)));

    map
}