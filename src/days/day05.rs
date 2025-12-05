use std::{f64::consts::PI};

use super::Day;
use num::complex::Complex;

pub struct Day05;

impl Day05 {
    fn parse(input: &str) -> (Vec<Vec<i64>>, Vec<i64>) {
        let normalized = input.replace("\r\n", "\n");
        let split_empty = normalized.split("\n\n").collect::<Vec<&str>>();

        let mut ranges = split_empty[0].lines().map(|line| line.split("-").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>();
        ranges.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let mut final_ranges: Vec<Vec<i64>> = vec![];

        // combine overlapping ranges into single 
        // Combine them if the range either overlap, or if they continue on cleanly (new range starts 1 over range of last)
        for range in ranges {
            if final_ranges.len() == 0 {
                final_ranges.push(range);
                continue;
            }

            let last = final_ranges.last_mut().unwrap();
            
            if range[0] <= last[1] + 1 {
                if range[1] > last[1] {
                    last[1] = range[1];
                }
            } else {
                final_ranges.push(range);
            }
        }

        // IDs
        let ids = split_empty[1].lines().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        (final_ranges, ids)
    }
}

impl Day for Day05 {
    fn part1(&self, input: &str) -> String {
        let (final_ranges, ids) = Self::parse(input);

        ids.iter().fold(0, |acc, id| {
            if final_ranges.iter().any(|range| range[0] <= *id && *id <= range[1]) {
                acc + 1
            } else {
                acc
            }
        }).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (final_ranges, _) = Self::parse(input);

        let mut final_fresh = 0i64;

        for range in final_ranges.iter().rev() {
            final_fresh += range[1] - range[0] + 1;
        }

        final_fresh.to_string()
    }
}
