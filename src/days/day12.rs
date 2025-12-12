pub struct Day12;
use std::vec;

use super::Day;


impl Day12 {
    fn parse(input: &str) -> (
        Vec<(usize, Vec<String>, usize)>,
        Vec<(usize, usize, Vec<usize>)>,
    ) {
        let mut gifts = Vec::<(usize, Vec<String>, usize)>::new();
        let mut trees = Vec::<(usize, usize, Vec<usize>)>::new();

        let mut lines = input.lines().peekable();

        while let Some(line) = lines.next() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if let Some((first_part, second_part)) = line.split_once(':') {
                if first_part.chars().all(|c| c.is_digit(10)) {
                    // Parse gift shape
                    let index: usize = first_part.trim().parse().unwrap();
                    let mut shape = Vec::new();
                    let mut area = 0;

                    // Until we have an empty line it's part of the shape
                    while let Some(&next_line) = lines.peek() {
                        if next_line.trim().is_empty() {
                            break;
                        }
                        // Only consume at this point
                        let shape_line = lines.next().unwrap().to_string();
                        area += shape_line.chars().filter(|&c| c == '#').count();
                        shape.push(shape_line);
                    }

                    gifts.push((index, shape, area));
                } else if first_part.contains('x') {
                    // Parse region
                    let dims = first_part
                        .split('x')
                        .map(|x| x.trim().parse::<usize>().unwrap())
                        .collect::<Vec<_>>();

                    let width = dims[0];
                    let height = dims[1];

                    let counts = second_part
                        .split_whitespace()
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();

                    trees.push((width, height, counts));
                }
            }
        }

        (gifts, trees)
    }
}

impl Day for Day12 {
    fn part1(&self, input: &str) -> String {
        let (gifts, trees) = Self::parse(input);

        let mut can_fit_maybe = 0;
        let mut trees_to_test = vec![];
        for (width, height, counts) in &trees {
            let mut sum_spaces_needed: usize = 0;
            for (i, count) in counts.iter().enumerate() {
                // Count how many tiles are needed at least for it to be possible maybe
                sum_spaces_needed += count * gifts[i].2;
            }

            // If they can possibly fit then add to the list to test properly after
            if sum_spaces_needed <= width * height {
                can_fit_maybe += 1;
                trees_to_test.push((width, height, counts));
            }
        }
        
        // THIS WORKED??? bruh i was thinking about that all the algorithms I found looked VERY complex xD
        // Good thing I test on examples
        can_fit_maybe.to_string()
    }

    fn part2(&self, input: &str) -> String {
        "We did it wahoo".to_string()
    }
}
