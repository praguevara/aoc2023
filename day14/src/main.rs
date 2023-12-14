use ndarray::{s, Array2};

type Grid = Array2<char>;

fn parse_grid(input: &str) -> Grid {
    let vecs = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Array2::from_shape_vec(
        (vecs.len(), vecs[0].len()),
        vecs.into_iter().flatten().collect(),
    )
    .unwrap()
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn tilt_grid(grid: &mut Grid, direction: Direction) {
    match direction {
        Direction::North => {
            for mut column in grid.axis_iter_mut(ndarray::Axis(1)) {
                for j in 0..column.dim() {
                    for k in (j + 1..column.dim()).rev() {
                        if column[k - 1] == '.' && column[k] == 'O' {
                            column[k] = '.';
                            column[k - 1] = 'O';
                        }
                    }
                }
            }
        }
        Direction::South => {
            for mut column in grid.axis_iter_mut(ndarray::Axis(1)) {
                for j in (0..column.dim()).rev() {
                    for k in 0..j {
                        if column[k + 1] == '.' && column[k] == 'O' {
                            column[k] = '.';
                            column[k + 1] = 'O';
                        }
                    }
                }
            }
        }
        Direction::East => {
            for mut row in grid.axis_iter_mut(ndarray::Axis(0)) {
                for j in (0..row.dim()).rev() {
                    for k in 0..j {
                        if row[k + 1] == '.' && row[k] == 'O' {
                            row[k] = '.';
                            row[k + 1] = 'O';
                        }
                    }
                }
            }
        }
        Direction::West => {
            for mut row in grid.axis_iter_mut(ndarray::Axis(0)) {
                for j in 0..row.dim() {
                    for k in (j + 1..row.dim()).rev() {
                        if row[k - 1] == '.' && row[k] == 'O' {
                            row[k] = '.';
                            row[k - 1] = 'O';
                        }
                    }
                }
            }
        }
    }
}

fn tilt_grid_cycle(grid: &mut Grid) {
    tilt_grid(grid, Direction::North);
    tilt_grid(grid, Direction::West);
    tilt_grid(grid, Direction::South);
    tilt_grid(grid, Direction::East);
}

#[test]
fn test_tilt_grid() {
    let grid_str = include_str!("../sample.txt");
    let mut grid = parse_grid(grid_str);

    let target_grid_str = include_str!("../sample_tilted.txt");
    let target_grid = parse_grid(target_grid_str);

    tilt_grid(&mut grid, Direction::North);
    assert_eq!(grid, target_grid);
}

#[test]
fn test_tilt_grid_cycle() {
    let grid_str = include_str!("../sample.txt");
    let mut grid = parse_grid(grid_str);

    let target_grid_str = include_str!("../sample_cycle.txt");
    let target_grid = parse_grid(target_grid_str);

    tilt_grid_cycle(&mut grid);
    assert_eq!(grid, target_grid);
}

fn compute_total_load(grid: &Grid) -> usize {
    let mut total_load = 0;
    for (j, row) in grid.slice(s![..;-1, ..]).rows().into_iter().enumerate() {
        total_load += row.iter().filter(|&&c| c == 'O').count() * (j + 1);
    }
    total_load
}

#[test]
fn test_total_load() {
    let grid_str = include_str!("../input.txt");
    let mut grid = parse_grid(grid_str);
    tilt_grid(&mut grid, Direction::North);

    assert_eq!(compute_total_load(&grid), 109755);
}

fn main() {
    let input = include_str!("../input.txt");
    let mut grid = parse_grid(input);

    tilt_grid(&mut grid, Direction::North);
    println!("Total load: {}", compute_total_load(&grid));

    let mut grid = parse_grid(input);
    for _ in 0..1000 {
        tilt_grid_cycle(&mut grid);
    }
    println!(
        "Total load after 1000000000 cycles: {}",
        compute_total_load(&grid)
    );
}
