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

    pub fn neighbours(&self, x: usize, y: usize) -> Vec<&char> {
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
            .iter()
            .filter(|(x, y)| *y < self.rows.len() && *x < self.rows[*y].len());

        valid_indices.filter_map(|(x, y)| self.at(*x, *y)).collect()
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
                    .iter()
                    .any(|&c| !c.is_ascii_digit() && *c != '.')
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

fn main() {
    let input = include_str!("../input.txt");
    let board = Board::new(input);
    let numbers = find_part_numbers(&board);
    let sum = numbers.iter().sum::<i32>();
    println!("{}", sum);
}
