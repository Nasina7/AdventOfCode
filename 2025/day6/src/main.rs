use std::{char, error::Error};

// TODO: Combine Aoc2025Day6 and Aoc2025Day6Part2
// TODO: Assert that all line lengths are the same

#[derive(Debug, Clone)]
struct BadInput;

impl Error for BadInput {}

impl std::fmt::Display for BadInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bad input")
    }
}

enum Operator {
    Add,
    Multiply,
}

struct Aoc2025Day6 {
    numbers: Vec<Vec<usize>>,
    operators: Vec<Operator>,
}

impl Aoc2025Day6 {
    fn new(filename: &str) -> Result<Aoc2025Day6, Box<dyn Error>> {
        let f = std::fs::read_to_string(filename)?;
        let lines = f.lines().collect::<Vec<&str>>();
        let mut numbers: Vec<Vec<usize>> = Vec::new();
        for problem_index in 0..lines[0].split_ascii_whitespace().count() {
            for line in lines.iter().take(lines.len() - 1) {
                if numbers.is_empty() {
                    numbers.resize(line.len(), Vec::new());
                }
                numbers[problem_index].push(
                    line.split_ascii_whitespace().collect::<Vec<&str>>()[problem_index]
                        .parse::<usize>()?,
                );
            }
        }

        let operators: Vec<Operator> = lines[lines.len() - 1]
            .split_ascii_whitespace()
            .map(|c| -> Result<Operator, Box<dyn Error>> {
                match c {
                    "+" => Ok(Operator::Add),
                    "*" => Ok(Operator::Multiply),
                    _ => Err(Box::new(BadInput)),
                }
            })
            .collect::<Result<Vec<Operator>, Box<dyn Error>>>()?;

        Ok(Aoc2025Day6 { numbers, operators })
    }

    fn run(&mut self) -> usize {
        let mut answers = Vec::new();
        for (index, operator) in self.operators.iter().enumerate() {
            let mut problem_result = 0;
            match operator {
                Operator::Add => {
                    for num in &self.numbers[index] {
                        problem_result += num;
                    }
                }
                Operator::Multiply => {
                    problem_result = 1;
                    for num in &self.numbers[index] {
                        problem_result *= num;
                    }
                }
            }
            answers.push(problem_result);
        }

        println!("Part 1 Solution: {}", answers.iter().sum::<usize>());

        answers.iter().sum()
    }
}

struct Aoc2025Day6Part2 {
    numbers: Vec<Vec<usize>>,
    operators: Vec<Operator>,
}

impl Aoc2025Day6Part2 {
    fn new(filename: &str) -> Result<Aoc2025Day6Part2, Box<dyn Error>> {
        let f = std::fs::read_to_string(filename)?;
        let lines = f
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut numbers: Vec<Vec<usize>> = Vec::new();
        numbers.push(Vec::new());
        let mut problem_index = 0;
        for x in 0..lines[0].len() {
            let mut new_number = String::new();
            for line in lines.iter().take(lines.len() - 1) {
                new_number.push(line[x]);
            }
            if new_number.trim().is_empty() {
                problem_index += 1;
                numbers.push(Vec::new());
            } else {
                numbers[problem_index].push(new_number.trim().parse::<usize>()?);
            }
        }

        let mut operators = Vec::new();
        for char in &lines[lines.len() - 1] {
            operators.push(match char {
                '+' => Operator::Add,
                '*' => Operator::Multiply,
                ' ' => continue,
                _ => return Err(Box::new(BadInput)),
            });
        }

        Ok(Aoc2025Day6Part2 { numbers, operators })
    }

    fn run(&mut self) -> usize {
        let mut answers = Vec::new();
        for (index, operator) in self.operators.iter().enumerate() {
            let mut problem_result = 0;
            match operator {
                Operator::Add => {
                    for num in &self.numbers[index] {
                        problem_result += num;
                    }
                }
                Operator::Multiply => {
                    problem_result = 1;
                    for num in &self.numbers[index] {
                        problem_result *= num;
                    }
                }
            }
            answers.push(problem_result);
        }

        println!("Part 2 Solution: {}", answers.iter().sum::<usize>());

        answers.iter().sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut day6p1 = Aoc2025Day6::new("input.txt")?;
    day6p1.run();
    let mut day6p2 = Aoc2025Day6Part2::new("input.txt")?;
    day6p2.run();
    Ok(())
}
