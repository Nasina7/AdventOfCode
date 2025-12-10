use std::error::Error;

// TODO: This runs a bit slowly, make it faster.

struct Aoc2025Day8 {
    coordinates: Vec<(isize, isize, isize)>,
    distances: Vec<Vec<isize>>,
    distances_used: Vec<Vec<bool>>,
    circuits: Vec<Vec<usize>>,
}

impl Aoc2025Day8 {
    fn new(filename: &str) -> Result<Aoc2025Day8, Box<dyn Error>> {
        Ok(Aoc2025Day8 {
            coordinates: std::fs::read_to_string(filename)?
                .trim()
                .lines()
                .map(|l| -> Result<(isize, isize, isize), Box<dyn Error>> {
                    let nums = l.split(',').collect::<Vec<&str>>();
                    Ok((
                        nums[0].parse::<isize>()?,
                        nums[1].parse::<isize>()?,
                        nums[2].parse::<isize>()?,
                    ))
                })
                .collect::<Result<Vec<(isize, isize, isize)>, Box<dyn Error>>>()?,
            distances: Vec::new(),
            distances_used: Vec::new(),
            circuits: Vec::new(),
        })
    }

    fn run(&mut self, iter_count: usize) -> (usize, usize) {
        // Post-init self.circuits
        for i in 0..self.coordinates.len() {
            self.circuits.push(Vec::new());
            self.circuits[i].push(i);
        }

        for coord1 in &self.coordinates {
            self.distances.push(Vec::new());
            self.distances_used.push(Vec::new());
            let last = self.distances.len() - 1;
            for coord2 in &self.coordinates {
                self.distances[last].push(
                    ((coord1.0 - coord2.0).pow(2)
                        + (coord1.1 - coord2.1).pow(2)
                        + (coord1.2 - coord2.2).pow(2))
                    .isqrt(),
                );
                self.distances_used[last].push(false);
            }
        }

        // Now, we have our list of all our distances, time to find the smallest one...
        //for _ in 0..iter_count {
        let mut iter = 0;
        let mut size = 0;
        let xmult;
        loop {
            let mut smallest = (isize::MAX, usize::MAX, usize::MAX);
            for (ind1, coord_list) in self.distances.iter().enumerate() {
                for (ind2, dist) in coord_list.iter().enumerate() {
                    if smallest.0 > *dist && *dist != 0 && !self.distances_used[ind1][ind2] {
                        smallest.0 = *dist;
                        smallest.1 = ind1;
                        smallest.2 = ind2;
                    }
                }
            }
            self.distances_used[smallest.1][smallest.2] = true;
            self.distances_used[smallest.2][smallest.1] = true;

            let circuit1 = self.find_circuit_for_coord_index(smallest.1);
            let circuit2 = self.find_circuit_for_coord_index(smallest.2);
            if circuit1 != circuit2 {
                let mut c2 = self.circuits[circuit2].clone();
                self.circuits[circuit1].append(&mut c2);
                self.circuits.remove(circuit2);
            }

            iter += 1;
            if iter == iter_count {
                let mut size_list = Vec::new();
                for c in &self.circuits {
                    if !c.is_empty() {
                        size_list.push(c.len());
                    }
                }
                size_list.sort();
                size_list.reverse();
                size = size_list[0] * size_list[1] * size_list[2];
                println!("3 Largest Circuits Mult Size (Part 1): {size}");
            }

            if self.circuits.len() == 1 {
                xmult = self.coordinates[smallest.1].0 * self.coordinates[smallest.2].0;
                println!("X Mult at End (Part 2): {xmult}");
                break;
            }
        }
        (size, xmult as usize)
    }

    fn find_circuit_for_coord_index(&self, index: usize) -> usize {
        for (ind, circuit) in self.circuits.iter().enumerate() {
            if circuit.contains(&index) {
                return ind;
            }
        }
        panic!("Couldn't find coord in circuit index, shouldn't be possible. {index}");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut day8 = Aoc2025Day8::new("input.txt")?;
    day8.run(1000);
    Ok(())
}
