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

fn tilt_grid_north(grid: &mut Grid) {
    for i in 0..grid.ncols() {
        for j in 0..grid.nrows() {
            for k in (j + 1..grid.nrows()).rev() {
                if grid[[k - 1, i]] == '.' && grid[[k, i]] == 'O' {
                    grid[[k, i]] = '.';
                    grid[[k - 1, i]] = 'O';
                }
            }
        }
    }
}

#[test]
fn test_tilt_grid_north() {
    let grid_str = include_str!("../sample.txt");
    let mut grid = parse_grid(grid_str);

    let target_grid_str = include_str!("../sample_tilted.txt");
    let target_grid = parse_grid(target_grid_str);

    tilt_grid_north(&mut grid);
    assert_eq!(grid, target_grid);
}

fn compute_total_load(grid: &Grid) -> usize {
    let mut total_load = 0;
    for (j, row) in grid.slice(s![..;-1, ..]).rows().into_iter().enumerate() {
        total_load += row.iter().filter(|&&c| c == 'O').count() * (j + 1);
    }
    total_load
}

fn main() {
    let input = include_str!("../input.txt");
    let mut grid = parse_grid(input);

    tilt_grid_north(&mut grid);
    println!("Total load: {}", compute_total_load(&grid));
}
