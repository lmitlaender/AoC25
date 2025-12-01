pub trait Day {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub mod day01;

use day01::Day01;

use std::collections::HashMap;

pub fn get_days() -> HashMap<u32, Box<dyn Day>> {
    let mut map: HashMap<u32, Box<dyn Day>> = HashMap::new();

    // Add all days
    map.insert(1, Box::new(Day01));

    map
}