use std::error::Error;

#[derive(Debug, Clone)]
struct BadInput;

impl Error for BadInput {}

impl std::fmt::Display for BadInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bad input")
    }
}

#[derive(Clone, Copy)]
enum Manifold {
    Spawner,
    Beam(usize),
    Splitter,
    Empty,
}

struct Aoc2025Day7 {
    manifold: Vec<Vec<Manifold>>,
}

impl Aoc2025Day7 {
    fn new(filename: &str) -> Result<Aoc2025Day7, Box<dyn Error>> {
        Ok(Aoc2025Day7 {
            manifold: std::fs::read_to_string(filename)?
                .lines()
                .map(|l| -> Result<Vec<Manifold>, Box<dyn Error>> {
                    l.chars()
                        .map(|c| -> Result<Manifold, Box<dyn Error>> {
                            match c {
                                'S' => Ok(Manifold::Spawner),
                                '|' => Ok(Manifold::Beam(1)),
                                '^' => Ok(Manifold::Splitter),
                                '.' => Ok(Manifold::Empty),
                                _ => Err(Box::new(BadInput)),
                            }
                        })
                        .collect()
                })
                .collect::<Result<Vec<Vec<Manifold>>, Box<dyn Error>>>()?,
        })
    }

    fn run(&mut self) -> (usize, usize) {
        let mut split_count = 0;
        let mut timeline_count = 0;

        for y in 0..self.manifold.len() {
            for x in 0..self.manifold[y].len() {
                match self.manifold[y][x] {
                    Manifold::Spawner => {
                        self.write_manifold(x, y + 1, Manifold::Beam(1));
                    }
                    Manifold::Beam(val) => match self.read_manifold(x, y + 1) {
                        Some(Manifold::Empty) => self.write_manifold(x, y + 1, Manifold::Beam(val)),
                        Some(Manifold::Splitter) => {
                            self.write_manifold(x - 1, y + 1, Manifold::Beam(val));
                            self.write_manifold(x + 1, y + 1, Manifold::Beam(val));
                            split_count += 1;
                        }
                        Some(Manifold::Beam(_)) => {
                            self.write_manifold(x, y + 1, Manifold::Beam(val))
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }

        for x in &self.manifold[self.manifold.len() - 1] {
            if let Manifold::Beam(val) = x {
                timeline_count += val;
            }
        }

        println!("Split Count (Part 1): {split_count}");
        println!("Timeline Count (Part 2): {timeline_count}");

        (split_count, timeline_count)
    }

    fn read_manifold(&self, x: usize, y: usize) -> Option<Manifold> {
        if y < self.manifold.len() && x < self.manifold[y].len() {
            Some(self.manifold[y][x])
        } else {
            None
        }
    }

    fn write_manifold(&mut self, x: usize, y: usize, val: Manifold) {
        if y < self.manifold.len() && x < self.manifold[y].len() {
            if let Manifold::Beam(count) = self.manifold[y][x]
                && let Manifold::Beam(count2) = val
            {
                self.manifold[y][x] = Manifold::Beam(count + count2)
            } else {
                self.manifold[y][x] = val;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut day7 = Aoc2025Day7::new("input.txt")?;
    day7.run();
    Ok(())
}
