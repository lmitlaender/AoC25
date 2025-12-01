mod days;

use std::io::{self, Write};

fn main() {
    let days_map = days::get_days();

    println!("❄️🎄🎁 Advent of Code 2025 🎁🎄❄️");
    println!("This year the elves have found out about project management - I hope it wasn't scrum or you can imagine the horrors..");
    println!("Currently the following doors have been opened (solved):");
    for d in days_map.keys() {
        println!("  Day {}", d);
    }

    print!("Select a door to solve: ");

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let day_num: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    let Some(day) = days_map.get(&day_num) else {
        println!("The selected day is still unsolved, christmas has not been saved yet..");
        return
    };

    let filename = format!("inputs/day{:02}.txt", day_num);
    let input = std::fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Could not read input file {}", filename));

    println!("\n❄️🎄🎁 Day {} results are in hohoho 🎁🎄❄️", day_num);
    println!("Part 1: {}", day.part1(&input.trim()));
    println!("Part 2: {}", day.part2(&input.trim()));
    println!("Thanks for helping save christmas! See you tomorrow 🎄");
}