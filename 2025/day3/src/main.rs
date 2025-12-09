use std::error::Error;

struct Aoc2025Day3 {
    batteries: Vec<Vec<usize>>,
}

impl Aoc2025Day3 {
    fn new(filename: &str) -> Result<Aoc2025Day3, Box<dyn Error>> {
        Ok(Aoc2025Day3 {
            batteries: std::fs::read_to_string(filename)?
                .lines()
                .map(|s| -> Result<Vec<usize>, Box<dyn Error>> {
                    s.chars()
                        .map(|c| -> Result<usize, Box<dyn Error>> {
                            c.to_string().parse::<usize>().map_err(From::from)
                        })
                        .collect()
                })
                .collect::<Result<Vec<Vec<usize>>, Box<dyn Error>>>()?,
        })
    }

    fn run(&self) -> (usize, usize) {
        let mut joltages_part1: Vec<usize> = Vec::new();
        let mut joltages_part2: Vec<usize> = Vec::new();
        for line in &self.batteries {
            joltages_part1.push(Aoc2025Day3::get_joltage_for_batteries(line, 2));
            joltages_part2.push(Aoc2025Day3::get_joltage_for_batteries(line, 12));
        }

        let total_joltage_part1 = joltages_part1.iter().sum();
        let total_joltage_part2 = joltages_part2.iter().sum();
        println!("Total Joltage (Part 1): {total_joltage_part1}");
        println!("Total Joltage (Part 2): {total_joltage_part2}");

        (total_joltage_part1, total_joltage_part2)
    }

    fn get_joltage_for_batteries(bats: &[usize], bat_count: usize) -> usize {
        let mut joltage = 0;
        let mut start_ind = 0;
        for i in 0..bat_count {
            let (add, ind) = Aoc2025Day3::highest_from_index(bats, start_ind, (bat_count - 1) - i);
            joltage += add * 10usize.pow((bat_count as u32 - 1) - i as u32);
            start_ind = ind + 1;
        }

        joltage
    }

    fn highest_from_index(bats: &[usize], start_ind: usize, end_pad: usize) -> (usize, usize) {
        let mut highest_bat = (0, 0);
        for (ind, bat) in bats[start_ind..(bats.len() - end_pad)].iter().enumerate() {
            if *bat > highest_bat.0 {
                highest_bat = (*bat, ind + start_ind);
            }
        }

        highest_bat
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let day3 = Aoc2025Day3::new("input.txt")?;
    day3.run();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hfi_works() {
        let res = Aoc2025Day3::highest_from_index(&[1, 2, 6, 3, 5, 1, 1, 2, 9, 9, 9], 3, 3);
        assert_eq!(res, (5, 4));
    }
}
