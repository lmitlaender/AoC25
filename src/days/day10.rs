use std::vec;

use super::Day;
use regex::Regex;
use good_lp::{Expression, Solution, SolverModel, Variable, constraint, default_solver, microlp, solvers::highs, variable, variables};

pub struct Day10;


impl Day10 {
    fn parse(input: &str) -> Vec<(Vec<u8>, Vec<Vec<i64>>, Vec<i64>)> {
        let square_re = Regex::new(r"(\[[^\]\r\n]+\])")
            .expect("Invalid regex");
        let round_re = Regex::new(r"(\([^\)\r\n]+\))")
            .expect("Invalid regex");
        let wavey_re = Regex::new(r"(\{[^}\r\n]+\})")
            .expect("Invalid regex");

        input.lines().map(|row| {
            let square = square_re.captures(row).unwrap()[1]
                .chars()
                .filter(|&c| c != '[' && c != ']')
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Unexpected character in square brackets"),
                })
                .collect::<Vec<u8>>();
            let round = round_re.find_iter(row)
                .map(|m| {
                    m.as_str()
                    .trim_matches(&['(', ')'][..])
                    .split(',')
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
                })
                .collect();
            let wavey = wavey_re.captures(row)
                .map(|cap| {
                    cap[1][1..cap[1].len()-1] // Slice the matched substring, not the entire row
                        .split(",")
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                }).unwrap();
            (square, round, wavey)
        }).collect()
    }


    // Failed DP branch attempt
    fn dp_step(buttons: &Vec<Vec<i64>>, start_state: Vec<i64>, target: &Vec<i64>, memo: &mut std::collections::HashMap<Vec<i64>, i64>, depth: i64) -> i64 {
        if start_state.iter().zip(target.iter()).any(|(a, b)| a > b) {
            return i32::MAX as i64; // Invalid state
        }

        if let Some(&res) = memo.get(&start_state) {
            return res;
        }

        if start_state == *target {
            return 0;
        }

        let min_cost = buttons.iter().map(|button| {
            let mut new_state = start_state.clone();
            for &pos in button {
                new_state[pos as usize] += 1;
            }
            Self::dp_step(buttons, new_state, target, memo, depth + 1)
        }).min().unwrap_or(i32::MAX as i64) + 1;
        memo.insert(start_state, min_cost);
        min_cost
    }
}

impl Day for Day10 {
    fn part1(&self, input: &str) -> String {
        let data = Self::parse(input);
        let mut sum = 0;

        for row in data.iter() {
            let target = &row.0;
            let buttons = &row.1;

            // We need to see that pushing any button twice is the same as pushing it zero times
            // this is so because pushing the button a second time will just revert the first push exactly
            // As such we generate all combinations of button presses where each button is either pressed 0 or 1 time.
            let num_buttons = buttons.len();
            let mut combinations = vec![];

            for mask in 1..(1 << num_buttons) {
                let mut combo = Vec::new();
                for i in 0..num_buttons {
                    if (mask & (1 << i)) != 0 {
                        combo.push(i + 1);
                    }
                }
                combinations.push(combo);
            }

            combinations.sort_by(|a, b| a.len().cmp(&b.len()));

            for combo in &combinations {
                let mut start = vec![0; target.len()];
                for &button_index in combo {
                    let button = &buttons[button_index - 1];
                    for &pos in button {
                        start[pos as usize] ^= 1;
                    }
                }
                if start == *target {
                    sum += combo.len();
                    break;
                }
            }
        }

        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        // This is a linear programming problem! Min sum = min(x_i) where x_i is the number of times a button is pressed
        // this probably works for part 1 too i assume but I didnt test.
        let data = Self::parse(input);
        let mut sum = 0;

        for row in data.iter() {
            let joltage_targets = &row.2;
            let buttons = &row.1;

            let mut vars = variables!();

            // Create variables for all of our buttons
            let x: Vec<Variable> = (0..buttons.len())
                .map(|_| vars.add(variable().integer().min(0)))
                .collect();

            // The problem we try to minimize is the sum of our button presses
            let total_presses = x.iter().sum::<good_lp::Expression>();
            let mut problem = vars.minimise(total_presses).using(good_lp::highs);

            // For each joltage target we create a constraint that is the sum of coefficient * x_i 
            for i in 0..joltage_targets.len() {
                let mut expr = good_lp::Expression::from(0.0);
                for (j, button) in buttons.iter().enumerate() {
                    if button.contains(&(i as i64)) {
                        expr = expr + x[j];
                    }
                }
                // Add equality constraint
                problem = problem.with(expr.eq(joltage_targets[i] as f64));
            }

            // Solve it and evaluate solution
            let solution = problem.solve().expect("Solver failed");
            sum += solution.eval(&x.iter().sum::<good_lp::Expression>()) as i64
        }

        sum.to_string()
    }
}
