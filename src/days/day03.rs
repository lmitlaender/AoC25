use super::Day;

pub struct Day03;

impl Day03 {
    fn parse(input: &str) -> Vec<Vec<u8>> {
        input.lines().map(
            |line| {
                line.chars().map(
                    |c| c.to_digit(10).unwrap() as u8
                ).collect::<Vec<u8>>()
            }
        ).collect::<Vec<Vec<u8>>>()
    }
}

impl Day for Day03 {
    fn part1(&self, input: &str) -> String {
        let banks = Self::parse(input);
        let mut sum: i128 = 0;

        for bank in &banks {
            let mut max_val = 0;
            let mut second_max_from_max_val = 0;
            for i in bank[..bank.len() - 1].iter() {
                if *i > max_val {
                    max_val = *i;
                    second_max_from_max_val = 0;
                    continue;
                }
                if *i > second_max_from_max_val {
                    second_max_from_max_val = *i;
                }
            }
            if second_max_from_max_val < bank[bank.len() - 1] {
                second_max_from_max_val = bank[bank.len() - 1]
            }
            
            // V1, a lot slower than the above cause it goes over each bank like 3 times at least. The above only goes over it once at most.
            /*let max_val = bank[..bank.len() - 1].iter().max().unwrap();
            let max_val_idx = bank.iter().position(|x| x == max_val).unwrap();
            let second_max_from_max_val = bank[max_val_idx + 1..].iter().max().unwrap();*/

            sum += (max_val * 10 + second_max_from_max_val) as i128;
        }
        
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        const ARR_SIZE: usize = 12;
        let banks = Self::parse(input);
        let mut sum: u64 = 0;
        
        // This solution is a further development from part 1 and can be applied to part 1 too
        // It just doesnt stop the iteration over the bank early instead looking at all bank entries and then checking
        // valid left final numer slots if it should go there
        // If it should all digits to the right of it have to be reset to 0 as the battery can't be left of it.
        // I kept part 1 and 2 seperate and duplicate to show the evolution of my solution, but this same method should be useable for no 2, just by replacing the 12 const with 2
        for bank in &banks {
            let mut max_vals: [u8; ARR_SIZE] = [0; ARR_SIZE];
            
            // Optimization potential: instead of iterating over each battery in the bank, is it faster to
            // fill each digit by iterating over the left subset of the bank? feels like should be similar but yes
            let mut empty_from = 0;
            for (idx, i) in bank.iter().enumerate() {
                let left_to_end = bank.len() - idx;
                let start = if left_to_end >= ARR_SIZE { 0 } else { ARR_SIZE - left_to_end };
                
                for y in start..ARR_SIZE {
                    if *i > max_vals[y] || empty_from <= y {
                        max_vals[y] = *i;
                        empty_from = y + 1;
                        break;
                    }
                }
            }

            for i in 0..ARR_SIZE {
                let pow = 10u64.pow((ARR_SIZE - 1 - i) as u32);
                sum += max_vals[i] as u64 * pow;
            }
        }

        sum.to_string()
    }


    // Alternative Part 2 nice solutions:

}
