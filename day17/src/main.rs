use std::{
    cell,
    collections::{HashMap, VecDeque},
};

fn parse_grid(input: &str) -> Box<[Box<[i32]>]> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .collect::<_>()
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct PathInfo {
    direction: Direction,
    sequential: i32,
    current_position: (i32, i32),
}

fn solve(grid: &[Box<[i32]>]) -> (Vec<(i32, i32)>, i32) {
    let mut paths: HashMap<PathInfo, i32> = HashMap::new();
    let mut best_path: Option<(PathInfo, Vec<(i32, i32)>, i32)> = None;

    let next_path_cells = |path_info: &PathInfo, cells: &[(i32, i32)]| {
        // can't have more than 3 sequential cells
        // can't get out of bounds
        // can't backtrack

        let (x, y) = cells.last().unwrap();
        let mut cells = [Option::None; 4];

        if !(*y == 0
            || (path_info.sequential == 3 && path_info.direction == Direction::Up)
            || path_info.direction == Direction::Down)
        {
            cells[0] = Some(((*x, y - 1), Direction::Up));
        }

        if !(*y == grid.len() as i32 - 1
            || (path_info.sequential == 3 && path_info.direction == Direction::Down)
            || path_info.direction == Direction::Up)
        {
            cells[1] = Some(((*x, y + 1), Direction::Down));
        }

        if !(*x == 0
            || (path_info.sequential == 3 && path_info.direction == Direction::Left)
            || path_info.direction == Direction::Right)
        {
            cells[2] = Some(((*x - 1, *y), Direction::Left));
        }

        if !(*x == grid[0].len() as i32 - 1
            || (path_info.sequential == 3 && path_info.direction == Direction::Right)
            || path_info.direction == Direction::Left)
        {
            cells[3] = Some(((*x + 1, *y), Direction::Right));
        }

        cells
    };

    let mut stack: Vec<(PathInfo, Vec<(i32, i32)>, i32)> = vec![(
        PathInfo {
            direction: Direction::Right,
            sequential: 0,
            current_position: (0, 0),
        },
        vec![(0, 0)],
        0,
    )];

    while let Some((path_info, cells, cost)) = stack.pop() {
        if let Some((ref _best_info, ref _best_path, best_cost)) = best_path {
            if cost > best_cost {
                continue;
            }
        }

        if let Some(existing_cost) = paths.get(&path_info) {
            if cost >= *existing_cost {
                continue;
            }
        }

        paths.insert(path_info.clone(), cost);

        if cells.last() == Some(&(grid[0].len() as i32 - 1, grid.len() as i32 - 1)) {
            if let Some((ref _best_info, ref _best_cells, best_cost)) = best_path {
                if cost < best_cost {
                    best_path = Some((path_info.clone(), cells, cost));
                    dbg!(cost);
                }
            } else {
                best_path = Some((path_info.clone(), cells, cost));
                dbg!(cost);
            }
            continue;
        }

        for (next_cell, direction) in next_path_cells(&path_info, &cells).into_iter().flatten() {
            let mut next_path = path_info.clone();
            let mut next_cells = cells.clone();
            next_cells.push(next_cell);
            next_path.direction = direction;
            next_path.current_position = next_cell;

            if path_info.direction == direction {
                next_path.sequential += 1;
            } else {
                next_path.sequential = 1;
            }

            stack.push((
                next_path,
                next_cells,
                cost + grid[next_cell.1 as usize][next_cell.0 as usize],
            ));
        }
    }

    let (_, path, cost) = best_path.unwrap();
    (path, cost)
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = parse_grid(input);

    let (cells, cost) = solve(&grid);

    // print best path on grid
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cells.contains(&(x as i32, y as i32)) {
                print!("O");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }

    dbg!(&cost);
}
