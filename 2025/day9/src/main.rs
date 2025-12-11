use std::error::Error;

// TODO: We should be using isize instead of usize here
// TODO: The handmade algo I wrote for part 2 has an edge case where a line going into a triangle,
// moving one tile, and exiting is counted as unplausible when it shouldn't be.  Fix this.
// TODO: The code for part 2 needs cleaned up and variables need renamed.

struct Rect {
    area: usize,
    coord1: (usize, usize),
    coord2: (usize, usize),
    coord1ind: usize,
}

impl Rect {
    fn new(coord1: (usize, usize), coord2: (usize, usize), coord1ind: usize) -> Rect {
        Rect {
            area: (coord1.0.max(coord2.0) - (coord1.0.min(coord2.0) - 1))
                * (coord1.1.max(coord2.1) - (coord1.1.min(coord2.1) - 1)),
            coord1,
            coord2,
            coord1ind,
        }
    }
}

struct Aoc2025Day9 {
    coordinates: Vec<(usize, usize)>,
    rectangles: Vec<Rect>,
}

impl Aoc2025Day9 {
    #[rustfmt::skip]
    fn new(filename: &str) -> Result<Aoc2025Day9, Box<dyn Error>> {
        let coordinates = std::fs::read_to_string(filename)?.trim().lines()
            .map(|l| -> Result<(usize, usize), Box<dyn Error>> {
                let n = l.split_once(',').unwrap();
                Ok((n.0.parse::<usize>()?, n.1.parse::<usize>()?))
            },
        ).collect::<Result<Vec<(usize, usize)>, Box<dyn Error>>>()?;

        Ok(Aoc2025Day9 {
            coordinates,
            rectangles: Vec::new(),
        })
    }

    fn run(&mut self) -> (usize, usize) {
        for (ind, coord1) in self.coordinates.iter().enumerate() {
            for coord2 in &self.coordinates {
                self.rectangles.push(Rect::new(*coord1, *coord2, ind));
            }
        }

        self.rectangles.sort_by(|a, b| a.area.cmp(&b.area));
        let l = self.rectangles.last().unwrap();
        println!("Part 1 Answer: {}", l.area);

        let mut good_rect = None;
        for (ind, rect) in self.rectangles.iter().rev().enumerate() {
            let mut plausible = true;
            let mut plausible2 = (false, false, false, false);

            let mut i = rect.coord1ind;
            loop {
                let cur = self.coordinates[i];
                let in_x_range_1 = cur.0 > (rect.coord1.0.min(rect.coord2.0));
                let in_x_range_2 = cur.0 < (rect.coord1.0.max(rect.coord2.0));
                let in_x_range = in_x_range_1 && in_x_range_2;
                let in_y_range_1 = cur.1 > (rect.coord1.1.min(rect.coord2.1));
                let in_y_range_2 = cur.1 < (rect.coord1.1.max(rect.coord2.1));
                let in_y_range = in_y_range_1 && in_y_range_2;
                let in_x_range_3 = cur.0 >= (rect.coord1.0.max(rect.coord2.0));
                let in_x_range_4 = cur.0 <= (rect.coord1.0.min(rect.coord2.0));
                let in_y_range_3 = cur.1 >= (rect.coord1.1.max(rect.coord2.1));
                let in_y_range_4 = cur.1 <= (rect.coord1.1.min(rect.coord2.1));

                plausible2.0 |= in_x_range_4 && in_y_range_4;
                plausible2.1 |= in_x_range_3 && in_y_range_4;
                plausible2.2 |= in_x_range_4 && in_y_range_3;
                plausible2.3 |= in_x_range_3 && in_y_range_3;

                i += 1;
                if i == self.coordinates.len() {
                    i = 0;
                }

                let nxt = self.coordinates[i];

                if in_x_range && in_y_range {
                    // Point is inside the rect.  This *probably* isn't a valid rect then.
                    plausible = false;
                    break;
                } else if in_x_range && !in_y_range {
                    if nxt.1.min(cur.1) < (rect.coord1.1.min(rect.coord2.1))
                        && nxt.1.max(cur.1) > (rect.coord1.1.min(rect.coord2.1))
                    {
                        plausible = false;
                        break;
                    }
                } else if !in_x_range && in_y_range {
                    if nxt.0.min(cur.0) < (rect.coord1.0.min(rect.coord2.0))
                        && nxt.0.max(cur.0) > (rect.coord1.0.min(rect.coord2.0))
                    {
                        plausible = false;
                        break;
                    }
                } else {
                    // Coord is safe, don't need to do anything.
                }

                if i == rect.coord1ind {
                    break;
                }
            }

            if plausible && plausible2.0 && plausible2.1 && plausible2.2 && plausible2.3 {
                good_rect = Some((self.rectangles.len() - 1) - ind);
                break;
            }
        }

        let p2a;
        if let Some(ind) = good_rect {
            let r = &self.rectangles[ind];
            println!("Part 2 Answer might be {}", r.area);
            p2a = r.area;
        } else {
            println!("Failed to find a suitable rectangle for part 2...");
            p2a = 0;
        }

        (l.area, p2a)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut day9 = Aoc2025Day9::new("input.txt")?;
    day9.run();
    Ok(())
}
