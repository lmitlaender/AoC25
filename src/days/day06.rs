use super::Day;


pub struct Day06;

impl Day06 {
    fn transpose(matrix: &[Vec<String>]) -> Vec<Vec<String>> {
        if matrix.is_empty() {
            return Vec::new();
        }
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut transposed = vec![Vec::with_capacity(rows); cols];

        for j in 0..cols {
            let mut col = Vec::with_capacity(rows);
            for i in 0..rows {
                if j < matrix[i].len() {
                    col.insert(0, matrix[i][j].clone());
                } else {
                    col.push(String::new());
                }
            }
            transposed[j] = col;
        }

        transposed
    }

    fn parse(input: &str) -> Vec<Vec<String>> {
        let lines = input.lines();

        Self::transpose(&lines.map(|row| {
            row.trim()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        }).collect::<Vec<Vec<String>>>())
    }

    fn parse_2(array: &str) -> Vec<Vec<String>>{
        let lines: Vec<Vec<char>> = array
            .lines()
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect();

        let mut res: Vec<Vec<String>> = vec![Vec::new()];
        
        let mut op = String::new();

        for i in 0..lines[0].len() {
            let mut num = String::new();
            let mut all_empty = true;
            let last = res.last_mut().unwrap();
            for y in 0..lines.len() {
                if i >= lines[y].len() {
                    continue; // Skip if the index is out of bounds for this row
                }
                let char = lines[y][i];

                if char.is_whitespace() {
                    continue;
                } else if char == '*' || char == '+' {
                    op.push(char);
                } else {
                    num.push(char)
                }
                all_empty = false;
            }

            if all_empty {
                last.insert(0, op.to_string());
                res.push(Vec::new());
                op = String::new();
            } else {
                last.insert(0, num.to_string());
            }
        }

        res.last_mut().unwrap().insert(0, op.to_string());

        res
    }

    fn compute_row_sum(rows: Vec<Vec<String>>) -> i64 {
        let mut total_sum: i64 = 0;

        for row in rows {
            let mut res = row[1].parse::<i64>().unwrap();
            let op = &row[0];

            for i in 2..row.len() {
                match op.as_str() {
                    "*" => {
                        res *= row[i].parse::<i64>().unwrap();
                    }
                    "+" => {
                        res += row[i].parse::<i64>().unwrap();
                    }
                    _ => {
                        panic!("Unknown operator: {}", op);
                    }
                }
            }

            total_sum += res;
        }

        total_sum
    }
}

impl Day for Day06 {
    fn part1(&self, input: &str) -> String {
        let rows_to_evaluate = Self::parse(input);
        
        Self::compute_row_sum(rows_to_evaluate).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let rows_to_evaluate = Self::parse_2(input);
        
        Self::compute_row_sum(rows_to_evaluate).to_string()
    }
}
