use std::error::Error;

#[derive(Debug, Clone)]
struct BadInput;

impl std::fmt::Display for BadInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid input")
    }
}

impl Error for BadInput {}

struct Aoc2025Day2 {
    ranges: Vec<(usize, usize)>,
}

impl Aoc2025Day2 {
    fn new(filename: &str) -> Result<Aoc2025Day2, Box<dyn Error>> {
        let mut failure = false;

        let f: Vec<(usize, usize)> = std::fs::read_to_string(filename)?
            .split(',')
            .map(|rng| {
                let tmp: Vec<usize> = rng
                    .trim()
                    .split('-')
                    .map(|n| {
                        n.parse::<usize>().unwrap_or_else(|_| {
                            failure = true;
                            0
                        })
                    })
                    .collect();

                if tmp.len() != 2 {
                    failure = true;
                    (0, 0)
                } else {
                    (tmp[0], tmp[1])
                }
            })
            .collect();

        if failure {
            return Err(Box::new(BadInput));
        }

        Ok(Aoc2025Day2 { ranges: f })
    }

    fn run(&mut self) -> Result<(usize, usize), Box<dyn Error>> {
        let mut matches_halves_only: Vec<usize> = Vec::new();
        let mut matches: Vec<usize> = Vec::new();

        for range in &self.ranges {
            for i in range.0..=range.1 {
                let i_str = i.to_string();
                let mut patterns = Vec::new();
                for x in 1..=i_str.len() / 2 {
                    let spl = i_str.split_at(x);
                    patterns.push(spl.0);
                }

                for pat in patterns {
                    let (matched, count) = Aoc2025Day2::pat_match(pat, &i_str);
                    if matched {
                        if !matches.contains(&i) {
                            matches.push(i);
                        }
                        if count == 2 {
                            matches_halves_only.push(i);
                        }
                    }
                }
            }
        }

        let total = matches_halves_only.iter().sum();
        let total2: usize = matches.iter().sum();
        println!("Day 1 Answer is {total}");
        println!("Day 2 Answer is {total2}");

        Ok((total, total2))
    }

    fn pat_match(pat: &str, num: &str) -> (bool, usize) {
        let inst: Vec<&str> = num.split(pat).collect();
        for i in &inst {
            if !i.is_empty() {
                return (false, 0);
            }
        }

        (true, inst.len() - 1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut day2 = Aoc2025Day2::new("input.txt")?;

    day2.run()?;

    Ok(())
}
