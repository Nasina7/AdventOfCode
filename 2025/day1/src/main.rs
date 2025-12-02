use std::{error::Error, fmt, str::FromStr};

#[derive(Debug, Clone)]
struct BadDirection;

impl fmt::Display for BadDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid direction")
    }
}

impl Error for BadDirection {}

#[derive(Debug, Clone)]
enum Rotation {
    Left(usize),
    Right(usize),
}

impl FromStr for Rotation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, rot) = s.split_at(1);
        let rot = rot.parse::<usize>()?;

        Ok(match dir {
            "L" => Rotation::Left(rot),
            "R" => Rotation::Right(rot),
            _ => return Err(Box::new(BadDirection)),
        })
    }
}

struct Aoc2025Day1 {
    dial: usize,
    pass1: usize,
    pass2: usize,

    rot_list: Vec<Rotation>,
}

impl Default for Aoc2025Day1 {
    fn default() -> Self {
        Self {
            dial: 50,
            pass1: 0,
            pass2: 0,

            rot_list: Vec::new(),
        }
    }
}

impl Aoc2025Day1 {
    fn from_filename(s: &str) -> Result<Aoc2025Day1, Box<dyn Error>> {
        Aoc2025Day1::from_str(std::fs::read_to_string(s)?)
    }

    fn from_str(s: String) -> Result<Aoc2025Day1, Box<dyn Error>> {
        Ok(Aoc2025Day1 {
            rot_list: Aoc2025Day1::parse_rotations_from_input(s)?,
            ..Default::default()
        })
    }

    fn parse_rotations_from_input(input: String) -> Result<Vec<Rotation>, Box<dyn Error>> {
        let mut rot_list = Vec::new();

        for i in input.split("\n") {
            if !i.is_empty() {
                rot_list.push(Rotation::from_str(i)?);
            }
        }

        Ok(rot_list)
    }

    fn run(&mut self) -> (usize, usize) {
        for iter in 0..self.rot_list.len() {
            self.turn_dial(iter);
        }

        (self.pass1, self.pass2)
    }

    fn turn_dial(&mut self, iter: usize) {
        match self.rot_list[iter] {
            Rotation::Left(amount) => {
                self.dial = ((100 - self.dial) % 100) + amount;
                self.pass2 += self.dial / 100;
                self.dial = (100 - (self.dial % 100)) % 100;
            }
            Rotation::Right(amount) => {
                self.dial += amount;
                self.pass2 += self.dial / 100;
                self.dial %= 100;
            }
        }

        self.pass1 += (self.dial == 0) as usize;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut day1 = Aoc2025Day1::from_filename("input.txt")?;
    let (pass1, pass2) = day1.run();

    println!("Part 1 Password is {pass1}");
    println!("Part 2 Password is {pass2}");
    Ok(())
}
