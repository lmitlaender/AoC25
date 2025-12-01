use super::Day;

pub struct Day01;

impl Day01 {
    fn get_ticks(input: &str) -> Result<Vec<i64>, String> {
        let mut rotations = Vec::new();

        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let dir = &line[..1];
            let amount: i64 = match line[1..].trim().parse::<i64>() {
                Ok(n) => n,
                Err(_) => {
                    return Err(format!("Failed to parse amount in line: {}", line));
                }
            };

            match dir {
                "L" => rotations.push(-amount),
                "R" => rotations.push(amount),
                _ => {
                    return Err(format!("Invalid direction '{}' in line: {}", dir, line));
                }
            }
        }

        Ok(rotations)
    }
}

impl Day for Day01 {
    fn part1(&self, input: &str) -> String {
        let rotations = match Self::get_ticks(input) {
            Ok(rotations) => rotations,
            Err(err) => return err,
        };

        let mut pos: i64 = 50;
        const TOTAL_TICKS: i64 = 100;
        let mut count = 0;

        rotations.iter().for_each(
            |rota| {
                // rem_euclid results in the remainder in space of 0 <= rem < TOTAL_TICKS
                pos = (pos + rota).rem_euclid(TOTAL_TICKS);
                if pos == 0 {
                    count += 1
                }
            }
        );

        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let rotations = match Self::get_ticks(input) {
            Ok(rotations) => rotations,
            Err(err) => return err,
        };

        let mut pos: i64 = 50;
        const TOTAL_TICKS: i64 = 100;
        let mut count = 0;
        let mut last_count = 0;

        rotations.iter().for_each(
            |rota| {
                last_count = count;
                let new_pos = pos + rota;
                
                count += (new_pos / TOTAL_TICKS).abs();

                if new_pos < 0 && pos != 0 {
                    count += 1;
                }
                if new_pos == 0 {
                    count += 1;
                }
                
                pos = new_pos.rem_euclid(TOTAL_TICKS);
            }
        );

        return count.to_string()
    }
}
