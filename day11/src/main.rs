use std::collections::HashSet;

type Coord = (usize, usize); // down, right

#[derive(Debug)]
struct Space {
    galaxies: HashSet<Coord>,
    height: usize,
    width: usize,
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

fn parse_input(input: &str) -> Space {
    let mut height = 0;
    let mut width = 0;

    let mut galaxies: HashSet<Coord> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.insert((y, x));
            }
            width = x + 1;
        }
        height = y + 1;
    }

    let mut galaxies_expanded_rows = HashSet::new();
    let mut expanded_rows = 0;
    // rows
    for row in 0..height {
        let row_galaxies = (0..=width)
            .filter_map(|x| galaxies.get(&(row, x)))
            .collect::<Vec<_>>();
        if row_galaxies.is_empty() {
            expanded_rows += 1;
        } else {
            for (y, x) in row_galaxies {
                galaxies_expanded_rows.insert((*y + expanded_rows, *x));
            }
        }
    }

    // columns
    let mut galaxies_expanded = HashSet::new();
    let mut expanded_columns = 0;
    for col in 0..width + expanded_rows {
        let column_galaxies = (0..=height + expanded_rows)
            .filter_map(|y| galaxies_expanded_rows.get(&(y, col)))
            .collect::<Vec<_>>();
        if column_galaxies.is_empty() {
            expanded_columns += 1;
        } else {
            for (y, x) in column_galaxies {
                let expanded_coords = (*y, *x + expanded_columns);
                galaxies_expanded.insert(expanded_coords);
            }
        }
    }

    Space {
        galaxies: galaxies_expanded,
        height: height + expanded_rows,
        width: width + expanded_columns,
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
    let space = parse_input(input);
    let total_distance = compute_total_distance(&space);
    println!("Total distance: {}", total_distance);
}

#[test]
fn test_distance() {
    let input = include_str!("../sample.txt");
    let space = parse_input(input);

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
fn test_total_distance() {
    let input = include_str!("../sample.txt");
    let space = parse_input(input);
    let total_distance = compute_total_distance(&space);
    assert_eq!(total_distance, 374);
}
