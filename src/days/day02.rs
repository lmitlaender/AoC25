use super::Day;

use std::collections::HashMap;

pub struct Day02;

impl Day02 {
    fn parse(input: &str) -> Vec<(i64, i64)> {
        let range_pairs = input.replace("\n", "").split(",").map(
            |range| {
                let range_vec = range.split("-").map(
                    |string_id| {
                        string_id.trim().parse::<i64>().unwrap()
                    }
                ).collect::<Vec<i64>>();

                (range_vec[0], range_vec[1])
            }
        ).collect::<Vec<(i64, i64)>>();

        range_pairs
    }
}

impl Day for Day02 {
    fn part1(&self, input: &str) -> String {
        let ranges = Self::parse(input);
        let mut sum: i64 = 0;
        let mut memo: HashMap<(i64, i64), i64> = HashMap::new();

        ranges.iter().for_each(|range| {
            let mut range_sum: i64 = 0;
            for i in range.0 .. range.1 + 1 {
                let num_string = i.to_string();
                if num_string.len() % 2 != 0 {
                    continue;
                }

                if num_string[..num_string.len()/2].eq(&num_string[num_string.len()/2..]) {
                    range_sum += i;
                }
            }

            memo.insert(*range, range_sum);
            sum += range_sum
        });

        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let ranges = Self::parse(input);
        let mut sum: i64 = 0;

        ranges.iter().for_each(|range| {
            let mut range_sum: i64 = 0;
            for i in range.0 .. range.1 + 1 {
                let num_string = i.to_string();
                let num_string_len = num_string.len();

                // for each prefix we check if it resolves the whole id (replacing it makes the string empty)
                for y in 1..(num_string_len / 2 + 1) {
                    // Can only resolve if the total len of string is multiple of prefix
                    if num_string_len % y != 0 {
                        continue;
                    }

                    let prefix = num_string[..y].to_string();

                    if num_string.replace(&prefix, "").is_empty() {
                        range_sum += i;
                        break;
                    }
                }
            }

            sum += range_sum
        });

        sum.to_string()
    }
}
