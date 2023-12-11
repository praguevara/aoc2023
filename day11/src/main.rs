use std::collections::{BTreeSet, HashSet};

type Coord = (usize, usize); // down, right

#[derive(Debug)]
struct Space {
    galaxies: HashSet<Coord>,
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
    let mut galaxies: HashSet<Coord> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.insert((y, x));
            }
        }
    }

    // rows
    let mut galaxies_expanded_rows = HashSet::new();
    let mut expanded_rows = 0;
    let occuppied_rows = galaxies.iter().map(|(y, _)| *y).collect::<BTreeSet<_>>();

    let row_galaxies_1 = galaxies
        .iter()
        .filter(|(y, _)| y == occuppied_rows.iter().next().unwrap())
        .collect::<Vec<_>>();
    for (y, x) in row_galaxies_1 {
        galaxies_expanded_rows.insert((*y + expanded_rows, *x));
    }

    for (l_row, r_row) in occuppied_rows.iter().zip(occuppied_rows.iter().skip(1)) {
        let new_rows = (expansion_rate - 1) * (r_row - l_row - 1);
        expanded_rows += new_rows;
        for (y, x) in galaxies.iter().filter(|(y, _)| y == r_row) {
            galaxies_expanded_rows.insert((*y + expanded_rows, *x));
        }
    }

    // columns
    let mut galaxies_expanded = HashSet::new();
    let mut expanded_cols = 0;
    let occupied_cols = galaxies_expanded_rows
        .iter()
        .map(|(_, x)| *x)
        .collect::<BTreeSet<_>>();

    let col_galaxies_1 = galaxies_expanded_rows
        .iter()
        .filter(|(_, x)| x == occupied_cols.iter().next().unwrap())
        .collect::<Vec<_>>();
    for (y, x) in col_galaxies_1 {
        galaxies_expanded.insert((*y, *x + expanded_cols));
    }

    for (l_col, r_col) in occupied_cols.iter().zip(occupied_cols.iter().skip(1)) {
        let new_cols = (expansion_rate - 1) * (r_col - l_col - 1);
        expanded_cols += new_cols;
        for (y, x) in galaxies_expanded_rows.iter().filter(|(_, x)| x == r_col) {
            galaxies_expanded.insert((*y, *x + expanded_cols));
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
    let space = parse_input(input, 2);
    let total_distance = compute_total_distance(&space);
    println!("Total distance: {}", total_distance);

    let space = parse_input(input, 1_000_000);
    let total_distance = compute_total_distance(&space);
    println!("Total distance: {}", total_distance);
}

#[test]
fn test_distance() {
    let input = include_str!("../sample.txt");
    let space = parse_input(input, 2);

    dbg!(&space.galaxies.iter().collect::<BTreeSet<_>>());

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
