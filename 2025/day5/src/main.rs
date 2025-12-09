use std::error::Error;
use std::ops::Range;

#[derive(Debug, Clone)]
struct BadInput;

impl Error for BadInput {}

impl std::fmt::Display for BadInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bad input")
    }
}

struct Aoc2025Day5 {
    ranges: Vec<Range<usize>>,
    ids: Vec<usize>,
}

impl Aoc2025Day5 {
    fn new(filename: &str) -> Result<Aoc2025Day5, Box<dyn Error>> {
        let mut ranges = Vec::new();
        let mut ids = Vec::new();

        let mut currently_ranges = true;
        for line in std::fs::read_to_string(filename)?.trim().lines() {
            if currently_ranges {
                if line.is_empty() {
                    currently_ranges = false;
                    continue;
                }

                let (rstart, rend) = line.split_once('-').ok_or(BadInput)?;
                ranges.push(Range {
                    start: rstart.parse::<usize>()?,
                    end: rend.parse::<usize>()? + 1,
                });
            } else {
                // Handle ID lines
                ids.push(line.parse::<usize>()?);
            }
        }

        Ok(Aoc2025Day5 { ranges, ids })
    }

    fn run(&mut self) -> (usize, usize) {
        (self.part1(), self.part2())
    }

    fn part1(&mut self) -> usize {
        let mut fresh_count = 0;
        for id in &self.ids {
            for range in &self.ranges {
                if range.contains(id) {
                    fresh_count += 1;
                    break;
                }
            }
        }

        println!("Fresh Ingredients (Part 1): {fresh_count}");

        fresh_count
    }

    fn part2(&mut self) -> usize {
        self.ranges.sort_by(|a, b| a.start.cmp(&b.start));
        for range in 0..self.ranges.len() {
            let nxt_range = (range + 1).clamp(0, self.ranges.len() - 1);
            if range != nxt_range {
                if self.ranges[range].end > self.ranges[nxt_range].end {
                    // Current range fully encompasses the next range
                    // Make the next range the current range, and make the current range do
                    // nothing.
                    self.ranges[nxt_range].start = self.ranges[range].start;
                    self.ranges[nxt_range].end = self.ranges[range].end;
                    self.ranges[range].end = self.ranges[range].start;
                } else if self.ranges[range].end > self.ranges[nxt_range].start {
                    // Make the current range end at the beginning of the next range.
                    // Note: It is impossible for the current range's start to be greater than the
                    // next range's start.
                    self.ranges[range].end = self.ranges[nxt_range].start;
                }
            }
        }

        let mut fresh_ids = 0;
        for range in &self.ranges {
            fresh_ids += range.end - range.start;
        }

        println!("Total Fresh Ids   (Part 2): {}", fresh_ids);

        fresh_ids
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut day5 = Aoc2025Day5::new("input.txt")?;
    day5.run();
    Ok(())
}
