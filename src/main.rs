mod days;

use std::env;
use std::io::{self, Write};
use std::time::Instant; // Import Instant for timing

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut day_arg: Option<u32> = None;
    let mut num_runs: u32 = 1;

    // Parse command-line arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-d" => {
                if i + 1 < args.len() {
                    day_arg = args[i + 1].parse().ok();
                    i += 1;
                } else {
                    eprintln!("Error: -d requires a day number");
                    return;
                }
            }
            "-n" => {
                if i + 1 < args.len() {
                    num_runs = args[i + 1].parse().unwrap_or(1);
                    i += 1;
                } else {
                    eprintln!("Error: -n requires a number of runs");
                    return;
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                return;
            }
        }
        i += 1;
    }

    let days_vec = days::get_days();

    println!("❄️🎄🎁 Advent of Code 2025 🎁🎄❄️");
    println!("This year the elves have found out about project management - I hope it wasn't scrum or you can imagine the horrors..");
    println!("Currently the following doors have been opened (solved):");
    for d in &days_vec {
        println!("  Day {}", d.0);
    }

    let day_num = if let Some(day) = day_arg {
        day
    } else {
        print!("Select a door to solve: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number.");
                return;
            }
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

    // Warm-up run
    let _ = day.part1(&input.trim());
    let _ = day.part2(&input.trim());

    // Repeat both parts n times
    let mut total_duration_part1 = 0;
    let mut total_duration_part2 = 0;
    let mut part1_result = String::new();
    let mut part2_result = String::new();

    for _ in 0..num_runs {
        // Measure Part 1
        let start = Instant::now();
        part1_result = day.part1(&input.trim());
        total_duration_part1 += start.elapsed().as_micros();

        // Measure Part 2
        let start = Instant::now();
        part2_result = day.part2(&input.trim());
        total_duration_part2 += start.elapsed().as_micros();
    }

    let avg_duration_part1 = total_duration_part1 / num_runs as u128;
    let avg_duration_part2 = total_duration_part2 / num_runs as u128;

    println!("Part 1: {} (average time: {:?} µs)", part1_result, avg_duration_part1);
    println!("Part 2: {} (average time: {:?} µs)", part2_result, avg_duration_part2);

    println!("Thanks for helping save christmas! See you tomorrow 🎄");
}