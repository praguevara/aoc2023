use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Board {
    rows: Vec<Vec<char>>,
}

impl Board {
    pub fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        Self { rows }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&char> {
        self.rows.get(y).and_then(|row| row.get(x))
    }

    pub fn valid_neighbour_indices(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let indices = [
            (x.wrapping_sub(1), y.wrapping_sub(1)),
            (x, y.wrapping_sub(1)),
            (x.wrapping_add(1), y.wrapping_sub(1)),
            (x.wrapping_sub(1), y),
            (x.wrapping_add(1), y),
            (x.wrapping_sub(1), y.wrapping_add(1)),
            (x, y.wrapping_add(1)),
            (x.wrapping_add(1), y.wrapping_add(1)),
        ];

        let valid_indices = indices
            .into_iter()
            .filter(|(x, y)| *y < self.rows.len() && *x < self.rows[*y].len());

        valid_indices
    }

    pub fn neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = &char> + '_ {
        let valid_indices = self.valid_neighbour_indices(x, y);
        valid_indices.filter_map(|(x, y)| self.at(x, y))
    }
}

fn find_part_numbers(board: &Board) -> Vec<i32> {
    let mut part_numbers = Vec::new();
    let mut current_number = 0;
    let mut is_next_to_symbol = false;

    for (y, row) in board.rows.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                current_number = current_number * 10 + c.to_digit(10).unwrap() as i32;
                is_next_to_symbol |= board
                    .neighbours(x, y)
                    .any(|&c| !c.is_ascii_digit() && c != '.')
            } else {
                if is_next_to_symbol {
                    part_numbers.push(current_number);
                }

                current_number = 0;
                is_next_to_symbol = false;
            }
        }
    }
    part_numbers
}

fn find_gears(board: &Board) -> Vec<(usize, usize, i32)> {
    let mut possible_gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new(); // (x, y) -> Vec for possible gear ratio

    let mut current_number = 0;
    let mut all_neighbours: HashSet<(usize, usize)> = HashSet::new(); // Set of all neighbours of a full number

    for (y, row) in board.rows.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                current_number = current_number * 10 + c.to_digit(10).unwrap() as i32;
                all_neighbours.extend(board.valid_neighbour_indices(x, y));
            } else {
                let gears = all_neighbours
                    .iter()
                    .filter(|(x, y)| *board.at(*x, *y).unwrap() == '*');

                for (x, y) in gears {
                    possible_gears
                        .entry((*x, *y))
                        .and_modify(|v| v.push(current_number))
                        .or_insert_with(|| vec![current_number]);
                }

                current_number = 0;
                all_neighbours.clear();
            }
        }
    }

    possible_gears
        .into_iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|((x, y), v)| (x, y, v[0] * v[1]))
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let board = Board::new(input);
    let gears = find_gears(&board);
    println!("{}", gears.iter().map(|(_, _, v)| v).sum::<i32>());
}
