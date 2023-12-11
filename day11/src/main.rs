use std::collections::BTreeMap;

type Coord = (usize, usize); // down, right

#[derive(Debug)]
struct Space {
    galaxies: Vec<Coord>,
}

impl Space {
    fn distance(&self, from: Coord, to: Coord) -> usize {
        let (from_y, from_x) = from;
        let (to_y, to_x) = to;
        let y_distance = (from_y as isize - to_y as isize).unsigned_abs();
        let x_distance = (from_x as isize - to_x as isize).unsigned_abs();
        y_distance + x_distance
    }
}

fn parse_input(input: &str, expansion_rate: usize) -> Space {
    let mut galaxies: Vec<Coord> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((y, x));
            }
        }
    }

    // expand rows
    let mut galaxies_expanded_rows = Vec::new();
    let mut expanded_rows = 0;

    let galaxies_by_row: BTreeMap<usize, Vec<Coord>> =
        galaxies.iter().fold(BTreeMap::new(), |mut acc, (y, x)| {
            acc.entry(*y).or_default().push((*y, *x));
            acc
        });

    // first row isn't expanded
    for (y, x) in galaxies_by_row.values().next().unwrap() {
        galaxies_expanded_rows.push((*y, *x));
    }

    for (l_row, (r_row, r_galaxies)) in galaxies_by_row.keys().zip(galaxies_by_row.iter().skip(1)) {
        let new_rows = (expansion_rate - 1) * (r_row - l_row - 1);
        expanded_rows += new_rows;
        for (y, x) in r_galaxies {
            galaxies_expanded_rows.push((*y + expanded_rows, *x));
        }
    }

    // expand columns
    let mut galaxies_expanded = Vec::new();
    let mut expanded_cols = 0;

    let galaxies_by_col: BTreeMap<usize, Vec<Coord>> =
        galaxies_expanded_rows
            .iter()
            .fold(BTreeMap::new(), |mut acc, (y, x)| {
                acc.entry(*x).or_default().push((*y, *x));
                acc
            });

    // first column isn't expanded
    for (y, x) in galaxies_by_col.values().next().unwrap() {
        galaxies_expanded.push((*y, *x + expanded_cols));
    }

    for (l_col, (r_col, r_galaxies)) in galaxies_by_col.keys().zip(galaxies_by_col.iter().skip(1)) {
        let new_cols = (expansion_rate - 1) * (r_col - l_col - 1);
        expanded_cols += new_cols;
        for (y, x) in r_galaxies {
            galaxies_expanded.push((*y, *x + expanded_cols));
        }
    }

    Space {
        galaxies: galaxies_expanded,
    }
}

fn compute_total_distance(space: &Space) -> usize {
    let mut total_distance = 0;
    for galaxy_from in &space.galaxies {
        for galaxy_to in &space.galaxies {
            if galaxy_from == galaxy_to {
                continue;
            }
            let distance = space.distance(*galaxy_from, *galaxy_to);
            total_distance += distance;
        }
    }
    total_distance / 2
}

fn main() {
    let input = include_str!("../input.txt");
    let expansion_rate = 2;
    let space = parse_input(input, expansion_rate);
    let total_distance = compute_total_distance(&space);
    println!("Total distance: {total_distance} with expansion rate = {expansion_rate}");

    let expansion_rate = 1_000_000;
    let space = parse_input(input, expansion_rate);
    let total_distance = compute_total_distance(&space);
    println!("Total distance: {total_distance} with expansion rate = {expansion_rate}");
}

#[test]
fn test_distance() {
    let input = include_str!("../sample.txt");
    let space = parse_input(input, 2);

    let from_galaxy = (0, 4);
    let to_galaxy = (10, 9);
    assert!(space.galaxies.contains(&from_galaxy));
    assert!(space.galaxies.contains(&to_galaxy));
    assert_eq!(space.distance(from_galaxy, to_galaxy), 15);
    assert_eq!(space.distance(to_galaxy, from_galaxy), 15);

    let from_galaxy = (2, 0);
    let to_galaxy = (7, 12);
    assert!(space.galaxies.contains(&from_galaxy));
    assert!(space.galaxies.contains(&to_galaxy));
    assert_eq!(space.distance(from_galaxy, to_galaxy), 17);
    assert_eq!(space.distance(to_galaxy, from_galaxy), 17);

    let from_galaxy = (11, 0);
    let to_galaxy = (11, 5);
    assert!(space.galaxies.contains(&from_galaxy));
    assert!(space.galaxies.contains(&to_galaxy));
    assert_eq!(space.distance(from_galaxy, to_galaxy), 5);
    assert_eq!(space.distance(to_galaxy, from_galaxy), 5);
}

#[test]
fn test_total_distance_1() {
    let input = include_str!("../sample.txt");
    let space = parse_input(input, 2);
    let total_distance = compute_total_distance(&space);
    assert_eq!(total_distance, 374);
}

#[test]
fn test_total_distance_10() {
    let input = include_str!("../sample.txt");
    let space = parse_input(input, 10);
    let total_distance = compute_total_distance(&space);
    assert_eq!(total_distance, 1030);
}

#[test]
fn test_total_distance_100() {
    let input = include_str!("../sample.txt");
    let space = parse_input(input, 100);
    let total_distance = compute_total_distance(&space);
    assert_eq!(total_distance, 8410);
}
