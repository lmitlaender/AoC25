use std::{f64::consts::PI};

use super::Day;
use num::complex::Complex;

pub struct Day04;

impl Day04 {
    fn parse(input: &str) -> Vec<Vec<u8>> {
        let grid = input.lines().map(|line| {
            line.chars().map(|c| {
                match c {
                    '@' => 1u8,
                    _ => 0u8
                }
            }).collect::<Vec<u8>>()
        }).collect::<Vec<Vec<u8>>>();

        grid
    }

    fn dot_product(kernel: &Vec<Vec<u8>>, grid_slice: &Vec<Vec<u8>>) -> u8 {
        let mut sum = 0;
        for i in 0..kernel.len() {
            for j in 0..kernel[0].len() {
                sum += kernel[i][j] * grid_slice[i][j];
            }
        }
        sum
    }

    fn pad(grid: &Vec<Vec<u8>>, kernel_size: usize) -> Vec<Vec<u8>> {
        let mut padded_grid = grid.clone();
        let pad_size = kernel_size / 2;

        for row in &mut padded_grid {
            for _ in 0..pad_size {
                row.insert(0, 0);
                row.push(0);
            }
        }

        // Create rows of zeros for top and bottom padding
        let row_length = padded_grid[0].len();
        let zero_row = vec![0; row_length];
        for _ in 0..pad_size {
            padded_grid.insert(0, zero_row.clone());
            padded_grid.push(zero_row.clone());
        }

        padded_grid
    }

    fn unpad(grid: Vec<Vec<u8>>, pad_size: usize) -> Vec<Vec<u8>> {
        grid[pad_size..grid.len() - pad_size]
            .iter()
            .map(|row| row[pad_size..row.len() - pad_size].to_vec())
            .collect()
    }

    fn mask_result(grid: &Vec<Vec<u8>>, result: &mut Vec<Vec<u8>>) {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    result[i][j] = 0;
                }
            }
        }
    }

    fn direct_conv(grid: &Vec<Vec<u8>>, kernel: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let kernel_size = kernel.len();
        let padded_grid = Day04::pad(&grid, kernel_size);

        let mut result = vec![vec![0; grid[0].len()]; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                // Extract the slice of the padded grid that fits the kernel
                let grid_slice = padded_grid[i..i + kernel_size]
                    .iter()
                    .map(|row| row[j..j + kernel_size].to_vec())
                    .collect::<Vec<Vec<u8>>>();

                // Apply the dot product function to sum slice and apply kernel
                result[i][j] = Day04::dot_product(kernel, &grid_slice);
            }
        }

        result
    }

    fn count_cells(grid: &Vec<Vec<u8>>, result: &Vec<Vec<u8>>) -> u32 {
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 0 && result[i][j] < 4 {
                    count += 1;
                }
            }
        }
        count
    }

    fn print_grid(grid: &Vec<Vec<u8>>) {
        for row in grid {
            println!("{}", row.iter().map(|&cell| cell.to_string()).collect::<Vec<String>>().join(" "));
        }
    }

    fn update_neighbors(grid: &mut Vec<Vec<u8>>, result: &mut Vec<Vec<u8>>, i: usize, j: usize, kernel: &Vec<Vec<u8>>) {
        let kernel_size = kernel.len();
        let pad = kernel_size / 2;

        // To adapt to different kernal sizes we compute all the neighbour indexes 
        // that took this cell into account in their convolution
        for ki in 0..kernel_size {
            for kj in 0..kernel_size {
                // Calc actual neighbour index
                let ni = i as isize + ki as isize - pad as isize;
                let nj = j as isize + kj as isize - pad as isize;

                // if Neighbour real cell (in grid boundaries) then subtract one.
                if ni >= 0 && nj >= 0 && (ni as usize) < grid.len() && (nj as usize) < grid[0].len() {
                    if result[ni as usize][nj as usize] > 0 {
                        result[ni as usize][nj as usize] -= 1;
                    }
                }
            }
        }
    }
}

impl Day for Day04 {
    fn part1(&self, input: &str) -> String {
        let grid = Self::parse(input);

        let kernel = vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1],
        ];

        // Perform direct convolution
        let mut result = Self::direct_conv(&grid, &kernel);

        // Mask the result with the original grid
        Self::mask_result(&grid, &mut result);

        // Count cells based on the condition
        let count = Self::count_cells(&grid, &result);

        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut grid = Self::parse(input);
        let mut total_removed = 0;

        let kernel = vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1],
        ];

        let mut result = Self::direct_conv(&grid, &kernel);
        Self::mask_result(&grid, &mut result);

        loop {
            let mut count = 0;
            let mut to_remove = Vec::new();

            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] != 0 && result[i][j] < 4 {
                        to_remove.push((i, j));
                        total_removed += 1;
                        count += 1;
                    }
                }
            }

            for &(i, j) in &to_remove {
                grid[i][j] = 0;
                Self::update_neighbors(&mut grid, &mut result, i, j, &kernel);
            }

            if count == 0 {
                break;
            }
        }

        println!("Final grid:");
        total_removed.to_string()
    }
}
