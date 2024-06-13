use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,
    Blocked,
}

type Coord = (usize, usize);
type Grid = Vec<Vec<Tile>>;

fn print_grid(grid: &Grid) {
    for row in grid {
        for col in row {
            let char = match col {
                Tile::Empty => '.',
                Tile::Blocked => '#',
            };
            print!("{}", char);
        }
        println!();
    }
}

fn parse_input(input_str: &str) -> (Coord, Grid) {
    let mut grid: Grid = vec![];
    let mut starting_pos = (0_usize, 0_usize);
    for (i, line_str) in input_str.lines().enumerate() {
        let mut line = vec![];
        for (j, c) in line_str.chars().enumerate() {
            line.push(match c {
                '.' => Tile::Empty,
                '#' => Tile::Blocked,
                'S' => {
                    starting_pos = (i, j);
                    Tile::Empty
                }
                _ => panic!("Invalid char"),
            })
        }
        grid.push(line);
    }
    (starting_pos, grid)
}

fn reachable_after_steps(
    cache: &mut HashMap<(Coord, usize), HashSet<Coord>>,
    grid: &Grid,
    from @ (from_r, from_c): Coord,
    steps: usize,
) -> HashSet<Coord> {
    if let Some(Tile::Empty) = grid.get(from_r).and_then(|r| r.get(from_c)) {
        // Valid tile, continue with algorithm
    } else {
        return HashSet::new();
    }

    if steps == 0 {
        return HashSet::from([(from_r, from_c)]);
    }

    if let Some(res) = cache.get(&(from, steps)) {
        return res.clone();
    }

    let north = (from_r + 1, from_c);
    let east = (from_r, from_c + 1);
    let south = (from_r - 1, from_c);
    let west = (from_r, from_c - 1);

    let mut res = HashSet::new();

    res.extend(reachable_after_steps(cache, grid, north, steps - 1));
    res.extend(reachable_after_steps(cache, grid, east, steps - 1));
    res.extend(reachable_after_steps(cache, grid, south, steps - 1));
    res.extend(reachable_after_steps(cache, grid, west, steps - 1));

    cache.insert((from, steps), res.clone());

    res
}

fn part_1(grid: &Grid, from: Coord, steps: usize) -> usize {
    let mut cache = HashMap::new();
    reachable_after_steps(&mut cache, grid, from, steps).len()
}

fn main() {
    let input_str = include_str!("../input.txt");
    let (from, grid) = parse_input(input_str);

    println!("Part 1: {}", part_1(&grid, from, 64));

    // print_grid(&grid);
}
