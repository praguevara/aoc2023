use std::collections::{hash_map::Entry, HashMap, VecDeque};

use ndarray::Array2;

fn parse_grid(input: &str) -> Array2<i32> {
    let lines_vec = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Array2::from_shape_vec((lines_vec.len(), lines_vec[0].len()), lines_vec.concat()).unwrap()
}

fn best_path(grid: &Array2<i32>) -> (Vec<(i32, i32)>, i32) {
    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();
    let mut queue: VecDeque<((i32, i32), Vec<(i32, i32)>, i32)> = VecDeque::new();

    queue.push_back(((0, 0), vec![(0, 0)], grid[[0, 0]]));
    visited.insert((0, 0), grid[[0, 0]]);

    let mut best_path = Vec::new();
    let mut best_cost = i32::MAX;

    while let Some(((row, col), path, cost)) = queue.pop_front() {
        if row == grid.shape()[0] as i32 - 1 && col == grid.shape()[1] as i32 - 1 {
            if cost < best_cost {
                best_path = path;
                best_cost = cost;
            }
            continue;
        }

        for (next_row, next_col) in [
            (row + 1, col),
            (row, col + 1),
            (row - 1, col),
            (row, col - 1),
        ] {
            if next_row >= grid.shape()[0] as i32
                || next_col >= grid.shape()[1] as i32
                || next_row < 0
                || next_col < 0
            {
                continue;
            }

            // dbg!(next_row, next_col);

            let mut next_path = path.clone();
            next_path.push((next_row, next_col));

            let next_cost = cost + grid[[next_row as usize, next_col as usize]];
            let visited_entry = visited.entry((next_row, next_col));
            match visited_entry {
                Entry::Occupied(mut entry) => {
                    if next_cost < *entry.get() {
                        entry.insert(next_cost);
                        queue.push_back(((next_row, next_col), next_path, next_cost));
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(next_cost);
                    queue.push_back(((next_row, next_col), next_path, next_cost));
                }
            }
        }
    }

    (best_path, best_cost)
}

fn main() {
    let input = include_str!("../sample.txt");
    let grid = parse_grid(input);
    dbg!(&grid);

    let (best_path, best_cost) = best_path(&grid);

    // print best path on grid
    for (y, row) in grid.rows().into_iter().enumerate() {
        for (x, col) in row.into_iter().enumerate() {
            if best_path.contains(&(y as i32, x as i32)) {
                print!("#");
            } else {
                print!("{}", col);
            }
        }
        println!();
    }

    dbg!(&best_cost);
}
