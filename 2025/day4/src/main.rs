use std::error::Error;

#[derive(Debug, Clone)]
struct BadInput;

impl std::fmt::Display for BadInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid input")
    }
}

impl Error for BadInput {}

enum Paper {
    None,
    Some,
    SomeMFD, // MFD = Marked For Deletion
}

struct Aoc2025Day4 {
    paper_grid: Vec<Vec<Paper>>,
    results: (Option<usize>, usize),
    line_length: usize,
}

impl Aoc2025Day4 {
    fn new(filename: &str) -> Result<Aoc2025Day4, Box<dyn Error>> {
        let mut ret = Aoc2025Day4 {
            paper_grid: std::fs::read_to_string(filename)?
                .trim()
                .lines()
                .map(|l| -> Result<Vec<Paper>, Box<dyn Error>> {
                    l.chars()
                        .map(|c| -> Result<Paper, Box<dyn Error>> {
                            match c {
                                '.' => Ok(Paper::None),
                                '@' => Ok(Paper::Some),
                                _ => Err(Box::new(BadInput)),
                            }
                        })
                        .collect()
                })
                .collect::<Result<Vec<Vec<Paper>>, Box<dyn Error>>>()?,
            results: (None, 0),
            line_length: 0,
        };

        // Make sure all lines are the same length
        let mut line_length = None;
        for line in &ret.paper_grid {
            match line_length {
                None => line_length = Some(line.len()),
                Some(len) => assert_eq!(len, line.len()),
            }
        }
        ret.line_length = line_length.unwrap_or(1) - 1;

        Ok(ret)
    }

    fn run(&mut self) -> (usize, usize) {
        let mut paper_count = 0xBEEF;
        while paper_count != 0 {
            paper_count = 0;
            for yind in 0..self.paper_grid.len() {
                for xind in 0..self.paper_grid[yind].len() {
                    if let Paper::None = self.paper_grid[yind][xind] {
                    } else {
                        // It's paper, check adjacent tiles
                        if self.count_adjacent(xind, yind) < 4 {
                            paper_count += 1;
                            self.paper_grid[yind][xind] = Paper::SomeMFD;
                        }
                    }
                }
            }

            for line in self.paper_grid.iter_mut() {
                for paper in line.iter_mut() {
                    if let Paper::SomeMFD = paper {
                        *paper = Paper::None;
                    }
                }
            }

            if self.results.0.is_none() {
                self.results.0 = Some(paper_count);
            }
            self.results.1 += paper_count;
        }

        println!("Paper Count (Part 1): {}", self.results.0.unwrap_or(0));
        println!("Paper Count (Part 2): {}", self.results.1);

        (self.results.0.unwrap_or(0), self.results.1)
    }

    fn count_adjacent(&self, xind: usize, yind: usize) -> usize {
        let xstart = xind.saturating_sub(1);
        let ystart = yind.saturating_sub(1);
        let xend = (xind + 1).clamp(0, self.line_length);
        let yend = (yind + 1).clamp(0, self.line_length);

        let mut count = 0;
        for y in ystart..=yend {
            for x in xstart..=xend {
                if let Paper::None = self.paper_grid[y][x] {
                } else if !(x == xind && y == yind) {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut day4 = Aoc2025Day4::new("input.txt")?;
    day4.run();
    Ok(())
}
