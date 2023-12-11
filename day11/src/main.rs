use std::collections::HashSet;

type Coord = (usize, usize); // down, right

#[derive(Debug)]
struct Space {
    galaxies: HashSet<Coord>,
    height: usize,
    width: usize,
    distances: Vec<Vec<usize>>,
}

impl Space {
    fn distance(&self, from: Coord, to: Coord) -> usize {
        let mut current_position = from;
        let mut total_distance = 0;
        while current_position != to {
            if current_position.1 < to.1 {
                current_position.1 += 1;
            } else if current_position.1 > to.1 {
                current_position.1 -= 1;
            } else if current_position.0 < to.0 {
                current_position.0 += 1;
            } else if current_position.0 > to.0 {
                current_position.0 -= 1;
            }
            total_distance += self.distances[current_position.1][current_position.0];
        }
        total_distance
    }
}

fn parse_input(input: &str) -> Space {
    let mut galaxies: HashSet<Coord> = HashSet::new();
    let mut lines = input.lines();

    let mut height = 0;
    let mut width = 0;

    let mut galaxies = HashSet::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                dbg!((y, x));
                galaxies.insert((y, x));
            }
            width = x;
        }
        height = y;
    }

    fn row_galaxies(galaxies: &HashSet<Coord>, width: usize, y: usize) -> Vec<&Coord> {
        (0..width).filter_map(|x| galaxies.get(&(y, x))).collect()
    }

    fn column_galaxies(galaxies: &HashSet<Coord>, height: usize, x: usize) -> Vec<&Coord> {
        (0..height).filter_map(|y| galaxies.get(&(y, x))).collect()
    }

    let mut distances = vec![vec![0; width]; height];

    // rows
    for row in 0..height {
        let row_galaxies = row_galaxies(&galaxies, width, row);
        if row_galaxies.is_empty() {
            distances[row] = vec![2; width];
        } else {
            distances[row] = vec![1; width];
            for galaxy in row_galaxies {
                distances[row][galaxy.0] = 0;
            }
        }
    }

    // columns
    for column in 0..width {
        let column_galaxies = column_galaxies(&galaxies, height, column);
        if column_galaxies.is_empty() {
            for row in 0..height {
                distances[row][column] = 2;
            }
        } else {
            for galaxy in column_galaxies {
                distances[galaxy.1][column] = 0;
            }
        }
    }

    Space {
        galaxies,
        height,
        width,
        distances,
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let space = parse_input(input);
}

#[test]
fn test_distance() {
    let input = include_str!("../sample.txt");
    let space = parse_input(input);
    let from_galaxy = (0, 3);
    let to_galaxy = (8, 7);

    dbg!(&space.galaxies);

    assert!(space.galaxies.get(&from_galaxy).is_some());
    assert!(space.galaxies.get(&to_galaxy).is_some());

    assert_eq!(space.distance(from_galaxy, to_galaxy), 15);
}
