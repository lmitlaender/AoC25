pub trait Day {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub mod day01;
pub mod day02;

use std::vec;

use day01::Day01;
use day02::Day02;


pub fn get_days() -> Vec<(i32, Box<dyn Day>)> {
    let mut map: Vec<(i32, Box<dyn Day>)> = vec![];

    // Add all days
    map.push((1, Box::new(Day01)));
    map.push((2, Box::new(Day02)));

    map
}