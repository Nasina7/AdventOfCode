use std::error::Error;

#[derive(Debug, Clone)]
struct BadInput;

impl Error for BadInput {}

impl std::fmt::Display for BadInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bad input")
    }
}

#[derive(Clone)]
struct Machine {
    indicator_light_current: Vec<bool>,
    indicator_light_target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_req: Vec<usize>,
    timelines: Vec<Machine>,
}

impl Machine {
    fn new(line: &str) -> Result<Machine, Box<dyn Error>> {
        let mut indicator_light_target = Vec::new();
        let mut buttons = Vec::new();
        let mut joltage_req = Vec::new();
        for i in line.split_whitespace() {
            match i.chars().collect::<Vec<char>>()[0] {
                '[' => {
                    indicator_light_target = i
                        .trim_matches(['[', ']'])
                        .chars()
                        .map(|c| -> Result<bool, Box<dyn Error>> {
                            Ok(match c {
                                '.' => false,
                                '#' => true,
                                _ => return Err(Box::new(BadInput)),
                            })
                        })
                        .collect::<Result<Vec<bool>, Box<dyn Error>>>()?;
                }
                '(' => {
                    buttons.push(
                        i.trim_matches(['(', ')'])
                            .split(',')
                            .map(|num| -> Result<usize, Box<dyn Error>> {
                                num.parse::<usize>().map_err(From::from)
                            })
                            .collect::<Result<Vec<usize>, Box<dyn Error>>>()?,
                    );
                }
                '{' => {
                    joltage_req = i
                        .trim_matches(['{', '}'])
                        .split(',')
                        .map(|num| -> Result<usize, Box<dyn Error>> {
                            num.parse::<usize>().map_err(From::from)
                        })
                        .collect::<Result<Vec<usize>, Box<dyn Error>>>()?;
                }
                _ => return Err(Box::new(BadInput)),
            }
        }

        Ok(Machine {
            indicator_light_current: vec![false; indicator_light_target.len()],
            indicator_light_target,
            buttons,
            joltage_req,
            timelines: Vec::new(),
        })
    }

    // Returns a count of minimum button presses
    fn run(&mut self) -> Option<usize> {
        // Clone for each button press
        if self.timelines.is_empty() {
            for button in &self.buttons {
                let mut tmp = self.clone();
                tmp.timelines.clear();
                if tmp.press_button(button) {
                    return Some(1);
                }

                self.timelines.push(tmp);
            }
        } else {
            for machine in &mut self.timelines {
                if let Some(v) = machine.run() {
                    return Some(v + 1);
                }
            }
        }

        None
    }

    fn check_good(&self) -> bool {
        for i in 0..self.indicator_light_target.len() {
            if self.indicator_light_current[i] != self.indicator_light_target[i] {
                return false;
            }
        }

        true
    }

    fn press_button(&mut self, button: &Vec<usize>) -> bool {
        for b in button {
            self.indicator_light_current[*b] = !self.indicator_light_current[*b];
        }

        self.check_good()
    }
}

struct Aoc2025Day10 {
    machines: Vec<Machine>,
}

impl Aoc2025Day10 {
    fn new(filename: &str) -> Result<Aoc2025Day10, Box<dyn Error>> {
        Ok(Aoc2025Day10 {
            machines: std::fs::read_to_string(filename)?
                .trim()
                .lines()
                .map(|l| -> Result<Machine, Box<dyn Error>> { Machine::new(l) })
                .collect::<Result<Vec<Machine>, Box<dyn Error>>>()?,
        })
    }

    fn run(&mut self) {
        let mut press_count = 0;
        for m in &mut self.machines {
            //press_count += m.run();
            loop {
                if let Some(v) = m.run() {
                    press_count += v;
                    println!("{v}");
                    break;
                }
            }
        }
        println!("{press_count}");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut day10 = Aoc2025Day10::new("input.txt")?;
    day10.run();
    Ok(())
}
