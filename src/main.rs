mod days;

use std::io::{self, Write};
use std::time::Instant; // Import Instant for timing

fn main() {
    let days_vec = days::get_days();

    println!("❄️🎄🎁 Advent of Code 2025 🎁🎄❄️");
    println!("This year the elves have found out about project management - I hope it wasn't scrum or you can imagine the horrors..");
    println!("Currently the following doors have been opened (solved):");
    for d in &days_vec {
        println!("  Day {}", d.0);
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

    let day = match days_vec.get((day_num - 1) as usize) {
        Some((_, day)) => day,
        None => {
            println!("The selected day is still unsolved, christmas has not been saved yet..");
            return;
        }
    };

    let filename = format!("inputs/day{:02}.txt", day_num);
    let input = std::fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Could not read input file {}", filename));

    println!("\n❄️🎄🎁 Day {} results are in hohoho 🎁🎄❄️", day_num);

    // Start timing for Part 1
    let start = Instant::now();
    let part1_result = day.part1(&input.trim());
    let duration_part1 = start.elapsed();
    println!("Part 1: {} (completed in {:?})", part1_result, duration_part1);

    // Start timing for Part 2
    let start = Instant::now();
    let part2_result = day.part2(&input.trim());
    let duration_part2 = start.elapsed();
    println!("Part 2: {} (completed in {:?})", part2_result, duration_part2);

    println!("Thanks for helping save christmas! See you tomorrow 🎄");
}