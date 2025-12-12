use std::error::Error;
use z3::{
    FuncDecl, SatResult, Solver, Sort,
    ast::{self, Array, BV, Bool, Int},
};

#[derive(Debug, Clone)]
struct BadInput;

impl Error for BadInput {}

impl std::fmt::Display for BadInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bad input")
    }
}

/*
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

*/

#[derive(Clone)]
struct Machine {
    indicator_light: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_req: Vec<usize>,
}

impl Machine {
    fn new(line: &str) -> Result<Machine, Box<dyn Error>> {
        let mut indicator_light = Vec::new();
        let mut buttons = Vec::new();
        let mut joltage_req = Vec::new();
        for i in line.split_whitespace() {
            match i.chars().collect::<Vec<char>>()[0] {
                '[' => {
                    indicator_light = i
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
            indicator_light,
            buttons,
            joltage_req,
        })
    }

    // Returns a count of minimum button presses
    #[rustfmt::skip]
    fn run(&mut self) -> Option<usize> {
        let solver = Solver::new();

        let start = BV::from_u64(0, 4);
        //let value = Int::fresh_const("uhh");
        let solution = BV::from_bits(&[false, true, true, false]).unwrap();

        let button_list = vec![
            vec![false, false, false, true],
            vec![false, true, false, true],
            vec![false, false, true, false],
            vec![false, false, true, true],
            vec![true, false, true, false],
            vec![true, true, false, false],
        ];

        /*
        let button_list: Vec<Vec<Bool>> = button_list
            .iter()
            .map(|l| -> Vec<Bool> {
                l.iter().map(|b| -> Bool {
                    Bool::from_bool(*b)
                }).collect()
            })
            .collect();
        */

        // Defines a 2d array that goes like Vec<Vec<bool>>... i think.
        let help = Array::fresh_const(
            "help",
            &Sort::int(),
            &Sort::bitvector(4),
        );

        for (ind, list) in button_list.iter().enumerate() {
            solver.assert(
                    help.select(&Int::from_u64(ind as u64)).as_bv().unwrap().eq(
                    BV::from_bits(list).unwrap()
                ));
        }

        // create variable for xor input
        // assert that it is one of the inputs in self.buttons


        
        //solver.assert(finish.eq(&solution));
        let mut iter_count = 0;
        let mut finish = BV::fresh_const("finish", 4);
        solver.assert(finish.eq(&start));
        loop {
            let value = Int::fresh_const("uhh");
            solver.assert(value.lt(Int::from_u64(button_list.len() as u64)));
            solver.assert(value.ge(0));
            let finish2 = BV::fresh_const("finish", 4);

            solver.assert(finish2.eq(finish.bvxor(help.select(&value).as_bv().unwrap())));
            solver.push();
            solver.assert(finish2.eq(&solution));
            if let SatResult::Sat = solver.check() {
                break;
            }
            solver.pop(1);
            finish = finish2;

            
            
            //let value = Int::fresh_const("uhh");
            //solver.assert(finish.eq(finish.bvxor(help.select(&value).as_bv().unwrap())));
            iter_count += 1;
            if iter_count == 4 {
                println!("{:?}", solver);
                panic!("");
            }
        }
        println!("{iter_count}");
        println!("{:?}", solver);

        match solver.check() {
            z3::SatResult::Sat => {
                let model = solver.get_model().unwrap();
                //let result = model.get_const_interp(
                //    &value
                //).unwrap();
                //println!("help??? {:?}", result);
                //let result2 = result.as_u64().unwrap();
                //println!("help? {}", result2);
            }
            _ => println!("fail"),
        }

        None
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
        //for m in &mut self.machines {
        self.machines[0].run();
        //}
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut day10 = Aoc2025Day10::new("input_test.txt")?;
    day10.run();
    Ok(())
}
