use core::panic;
use std::collections::HashSet;

use ndarray::Array2;

fn parse_input(input_str: &str) -> Array2<char> {
    let input_lines = input_str
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Array2::from_shape_vec(
        (input_lines.len(), input_lines[0].len()),
        input_lines.into_iter().flatten().collect(),
    )
    .unwrap()
}

fn next_beam(
    grid: &Array2<char>,
    previous_path: &mut HashSet<((i32, i32), (i32, i32))>,
    pos: (i32, i32),
    dir: (i32, i32),
) {
    debug_assert!(dir.0.abs() + dir.1.abs() == 1);

    let (pos_y, pos_x) = pos;
    let (dir_y, dir_x) = dir;

    if pos_y < 0 || pos_x < 0 {
        return;
    }

    if previous_path.contains(&(pos, dir)) {
        // loop
        return;
    }

    let current_cell = grid.get([pos_y as usize, pos_x as usize]);
    if let Some(current_cell) = current_cell {
        previous_path.insert((pos, dir));
        match current_cell {
            '.' => {
                next_beam(grid, previous_path, (pos_y + dir_y, pos_x + dir_x), dir);
            }
            '/' => {
                let (dir_y, dir_x) = (-dir_x, -dir_y);
                next_beam(
                    grid,
                    previous_path,
                    (pos_y + dir_y, pos_x + dir_x),
                    (dir_y, dir_x),
                );
            }
            '\\' => {
                let (dir_y, dir_x) = (dir_x, dir_y);
                next_beam(
                    grid,
                    previous_path,
                    (pos_y + dir_y, pos_x + dir_x),
                    (dir_y, dir_x),
                );
            }
            '-' => {
                if dir_x != 0 {
                    next_beam(grid, previous_path, (pos_y + dir_y, pos_x + dir_x), dir);
                } else {
                    let (left_dir_y, left_dir_x) = (0, -1);
                    next_beam(
                        grid,
                        previous_path,
                        (pos_y, (pos_x + left_dir_x)),
                        (left_dir_y, left_dir_x),
                    );
                    let (right_dir_y, right_dir_x) = (0, 1);
                    next_beam(
                        grid,
                        previous_path,
                        (pos_y, (pos_x + right_dir_x)),
                        (right_dir_y, right_dir_x),
                    );
                }
            }
            '|' => {
                if dir_y != 0 {
                    next_beam(grid, previous_path, (pos_y + dir_y, pos_x + dir_x), dir)
                } else {
                    let (up_dir_y, up_dir_x) = (1, 0);
                    next_beam(
                        grid,
                        previous_path,
                        (pos_y + up_dir_y, pos_x),
                        (up_dir_y, up_dir_x),
                    );
                    let (down_dir_y, down_dir_x) = (-1, 0);
                    next_beam(
                        grid,
                        previous_path,
                        (pos_y + down_dir_y, pos_x),
                        (down_dir_y, down_dir_x),
                    );
                }
            }
            _ => panic!("invalid cell, got {}", current_cell),
        };
    }
}

fn count_energized_tiles(
    grid: &Array2<char>,
    starting_position: (i32, i32),
    starting_direction: (i32, i32),
) -> usize {
    let mut path = HashSet::new();
    next_beam(grid, &mut path, starting_position, starting_direction);
    path.into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len()
}

fn main() {
    let input_str = include_str!("../input.txt");
    let grid = parse_input(input_str);
    let mut path = HashSet::new();
    next_beam(&grid, &mut path, (0, 0), (0, 1));
    let energized_tiles = path.into_iter().map(|(pos, _)| pos).collect::<HashSet<_>>();
    println!("{}\n", energized_tiles.len());

    let mut max_tiles = 0;
    for i in 0..grid.nrows() {
        let tiles = count_energized_tiles(&grid, (i as i32, 0), (0, 1));
        max_tiles = max_tiles.max(tiles);

        let tiles = count_energized_tiles(&grid, (i as i32, grid.ncols() as i32 - 1), (0, -1));
        max_tiles = max_tiles.max(tiles);
    }

    for i in 0..grid.ncols() {
        let tiles = count_energized_tiles(&grid, (0, i as i32), (1, 0));
        max_tiles = max_tiles.max(tiles);

        let tiles = count_energized_tiles(&grid, (grid.nrows() as i32 - 1, i as i32), (-1, 0));
        max_tiles = max_tiles.max(tiles);
    }

    println!("{}", max_tiles);
}
