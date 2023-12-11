use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Tile {
    NorthSouth, // |
    EastWest,   // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Ground,     // .
    Start,      // S
}

impl core::fmt::Debug for Tile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Tile::NorthSouth => write!(f, "|"),
            Tile::EastWest => write!(f, "-"),
            Tile::NorthEast => write!(f, "L"),
            Tile::NorthWest => write!(f, "J"),
            Tile::SouthWest => write!(f, "7"),
            Tile::SouthEast => write!(f, "F"),
            Tile::Ground => write!(f, "."),
            Tile::Start => write!(f, "S"),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::NorthSouth),
            '-' => Ok(Tile::EastWest),
            'L' => Ok(Tile::NorthEast),
            'J' => Ok(Tile::NorthWest),
            '7' => Ok(Tile::SouthWest),
            'F' => Ok(Tile::SouthEast),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            _ => Err("Invalid tile"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord(i32, i32);

impl From<(i32, i32)> for Coord {
    fn from((row, col): (i32, i32)) -> Self {
        Coord(row, col)
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Map {
    fn get_tile(&self, coord: impl Into<Coord>) -> Option<&Tile> {
        let Coord(row, col) = coord.into();
        if row < 0 || col < 0 {
            return None;
        }
        self.tiles
            .get(row as usize)
            .and_then(|row| row.get(col as usize))
    }

    fn find_start(&self) -> Option<Coord> {
        for (row, line) in self.tiles.iter().enumerate() {
            for (col, tile) in line.iter().enumerate() {
                if *tile == Tile::Start {
                    return Some(Coord(row as i32, col as i32));
                }
            }
        }
        None
    }
}

fn parse_input(input: &str) -> Result<Map, &'static str> {
    let lines = input.lines().collect::<Vec<&str>>();

    let height = lines.len();
    let width = lines[0].len();

    let mut tiles = vec![vec![Tile::Ground; width]; height];
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            tiles[row][col] = Tile::try_from(c)?;
        }
    }

    Ok(Map {
        tiles,
        width,
        height,
    })
}

fn compute_edges(map: &Map, coord: Coord) -> Vec<Coord> {
    let mut edges: Vec<Coord> = vec![];
    let tile = map.get_tile(coord).unwrap();

    let Coord(y, x) = coord;

    let northern_coord = Coord(y - 1, x);
    let northern: Option<(Tile, Coord)> =
        map.get_tile(northern_coord).map(|t| (*t, northern_coord));

    let southern_coord = Coord(y + 1, x);
    let southern: Option<(Tile, Coord)> =
        map.get_tile(southern_coord).map(|t| (*t, southern_coord));

    let eastern_coord = Coord(y, x + 1);
    let eastern: Option<(Tile, Coord)> = map.get_tile(eastern_coord).map(|t| (*t, eastern_coord));

    let western_coord = Coord(y, x - 1);
    let western: Option<(Tile, Coord)> = map.get_tile(western_coord).map(|t| (*t, western_coord));

    match tile {
        Tile::NorthSouth => {
            [northern, southern]
                .iter()
                .flatten()
                .for_each(|t| edges.push(t.1));
        }
        Tile::EastWest => {
            [eastern, western]
                .iter()
                .flatten()
                .for_each(|t| edges.push(t.1));
        }
        Tile::NorthEast => {
            [northern, eastern]
                .iter()
                .flatten()
                .for_each(|t| edges.push(t.1));
        }
        Tile::NorthWest => {
            [northern, western]
                .iter()
                .flatten()
                .for_each(|t| edges.push(t.1));
        }
        Tile::SouthWest => {
            [southern, western]
                .iter()
                .flatten()
                .for_each(|t| edges.push(t.1));
        }
        Tile::SouthEast => {
            [southern, eastern]
                .iter()
                .flatten()
                .for_each(|t| edges.push(t.1));
        }
        Tile::Start => {
            [
                match northern {
                    Some((Tile::NorthSouth | Tile::SouthWest | Tile::SouthEast, _)) => northern,
                    _ => None,
                },
                match southern {
                    Some((Tile::NorthSouth | Tile::NorthWest | Tile::NorthEast, _)) => southern,
                    _ => None,
                },
                match eastern {
                    Some((Tile::EastWest | Tile::NorthWest | Tile::SouthWest, _)) => eastern,
                    _ => None,
                },
                match western {
                    Some((Tile::EastWest | Tile::NorthEast | Tile::SouthEast, _)) => western,
                    _ => None,
                },
            ]
            .iter()
            .flatten()
            .for_each(|t| edges.push(t.1));
        }
        Tile::Ground => {}
    }

    edges
}

fn compute_adjacency_matrix(map: &Map) -> Vec<Vec<Vec<Coord>>> {
    let mut adjacency_matrix = vec![vec![vec![]; map.width]; map.height];

    for (y, row) in adjacency_matrix.iter_mut().enumerate() {
        for (x, edges) in row.iter_mut().enumerate() {
            *edges = compute_edges(map, Coord(y as i32, x as i32));
        }
    }

    adjacency_matrix
}

fn compute_longest_distance(map: &Map, adjacency_matrix: &[Vec<Vec<Coord>>]) -> usize {
    let start = map.find_start().unwrap();
    let mut visited_distances: HashMap<Coord, usize> = HashMap::from([(start, 0)]);

    let mut queue: VecDeque<(Coord, usize)> = VecDeque::from([(start, 0)]);
    while let Some((coord, distance)) = queue.pop_front() {
        for edge in &adjacency_matrix[coord.0 as usize][coord.1 as usize] {
            if !visited_distances.contains_key(edge) {
                visited_distances.insert(*edge, distance + 1);
                queue.push_back((*edge, distance + 1));
            }
        }
    }

    *visited_distances.values().max().unwrap()
}

type MainLoop = HashMap<Coord, i32>;

fn compute_main_loop(map: &Map, adjacency_matrix: &[Vec<Vec<Coord>>]) -> MainLoop {
    let start = map.find_start().unwrap();

    let mut main_loop = vec![start];
    let mut next_node = &adjacency_matrix[start.0 as usize][start.1 as usize][0];
    while *next_node != start {
        main_loop.push(*next_node);
        let current_node = next_node;
        let previous_node = main_loop[main_loop.len() - 2];
        next_node = adjacency_matrix[current_node.0 as usize][current_node.1 as usize]
            .iter()
            .find(|&&node| node != previous_node)
            .unwrap();
    }

    let mut main_loop_map = HashMap::new();

    // Compute direction (+1 if south, -1 if north)
    for i in 0..main_loop.len() {
        let previous_node = main_loop[(i + main_loop.len() - 1) % main_loop.len()];
        let current_node = main_loop[i];
        let next_node = main_loop[(i + 1) % main_loop.len()];

        let direction = next_node.0 - previous_node.0;

        main_loop_map.insert(current_node, direction);
    }

    main_loop_map
}

fn compute_tiles_contained_by_main_loop(map: &Map, main_loop: &MainLoop) -> Vec<Coord> {
    let mut tiles_contained_by_main_loop = vec![];

    for y in 0..map.height {
        let mut crosses = 0;
        for x in 0..map.width {
            if let Some(dy) = main_loop.get(&Coord(y as i32, x as i32)).copied() {
                crosses += dy;
            } else if crosses != 0 {
                tiles_contained_by_main_loop.push(Coord(y as i32, x as i32));
            }
        }
    }

    tiles_contained_by_main_loop
}

fn main() {
    let input = include_str!("../input.txt");
    let map = parse_input(input).unwrap();
    let adjacency_matrix = compute_adjacency_matrix(&map);
    let longest_distance = compute_longest_distance(&map, &adjacency_matrix);
    println!("{longest_distance}");

    let main_loop = compute_main_loop(&map, &adjacency_matrix);
    let tiles_contained_by_main_loop = compute_tiles_contained_by_main_loop(&map, &main_loop);
    println!("{:?}", tiles_contained_by_main_loop.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_compute_adjacency_matrix() {
        let input = ".....\r\n.S-7.\r\n.|.|.\r\n.L-J.\r\n.....\r\n";
        let map = parse_input(input).unwrap();
        let adjacency_matrix = compute_adjacency_matrix(&map);

        assert_eq!(adjacency_matrix[0][0], vec![]);
        assert_eq!(
            adjacency_matrix[1][1]
                .iter()
                .copied()
                .collect::<HashSet<_>>(),
            HashSet::from([Coord(1, 2), Coord(2, 1)])
        );
        assert_eq!(
            adjacency_matrix[1][3]
                .iter()
                .copied()
                .collect::<HashSet<_>>(),
            HashSet::from([Coord(1, 2), Coord(2, 3)])
        );
    }

    #[test]
    fn test_compute_longest_distance() {
        let input = "..F7.\r\n.FJ|.\r\nSJ.L7\r\n|F--J\r\nLJ...\r\n";
        let map = parse_input(input).unwrap();
        let adjacency_matrix = compute_adjacency_matrix(&map);
        let longest_distance = compute_longest_distance(&map, &adjacency_matrix);
        assert_eq!(longest_distance, 8);
    }

    #[test]
    fn test_compute_main_loop() {
        let input = ".....\r\n\
        .S-7.\r\n\
        .|.|.\r\n\
        .L-J.\r\n\
        .....\r\n";
        let map = parse_input(input).unwrap();
        let adjacency_matrix = compute_adjacency_matrix(&map);
        let main_loop = compute_main_loop(&map, &adjacency_matrix);

        assert_eq!(main_loop.len(), 8);
    }

    #[test]
    fn test_compute_tiles_contained_by_main_loop() {
        let input = "FF7FSF7F7F7F7F7F---7\r\n\
        L|LJ||||||||||||F--J\r\n\
        FL-7LJLJ||||||LJL-77\r\n\
        F--JF--7||LJLJ7F7FJ-\r\n\
        L---JF-JLJ.||-FJLJJ7\r\n\
        |F|F-JF---7F7-L7L|7|\r\n\
        |FFJF7L7F-JF7|JL---7\r\n\
        7-L-JL7||F7|L7F-7F7|\r\n\
        L.L7LFJ|||||FJL7||LJ\r\n\
        L7JLJL-JLJLJL--JLJ.L\r\n";
        let map = parse_input(input).unwrap();
        let adjacency_matrix = compute_adjacency_matrix(&map);
        let main_loop = compute_main_loop(&map, &adjacency_matrix);
        let tiles_contained_by_main_loop = compute_tiles_contained_by_main_loop(&map, &main_loop);

        assert_eq!(tiles_contained_by_main_loop.len(), 10);
    }

    #[test]
    fn test_compute_tiles_contained_by_main_loop_2() {
        let input = ".F----7F7F7F7F-7....\r\n\
        .|F--7||||||||FJ....\r\n\
        .||.FJ||||||||L7....\r\n\
        FJL7L7LJLJ||LJ.L-7..\r\n\
        L--J.L7...LJS7F-7L7.\r\n\
        ....F-J..F7FJ|L7L7L7\r\n\
        ....L7.F7||L7|.L7L7|\r\n\
        .....|FJLJ|FJ|F7|.LJ\r\n\
        ....FJL-7.||.||||...\r\n\
        ....L---J.LJ.LJLJ...\r\n";
        let map = parse_input(input).unwrap();
        let adjacency_matrix = compute_adjacency_matrix(&map);
        let main_loop = compute_main_loop(&map, &adjacency_matrix);
        let tiles_contained_by_main_loop = compute_tiles_contained_by_main_loop(&map, &main_loop);

        assert_eq!(tiles_contained_by_main_loop.len(), 8);
    }

    #[test]
    fn test_compute_tiles_contained_by_main_loop_3() {
        let input = "FF7FSF7F7F7F7F7F---7\r\n\
        L|LJ||||||||||||F--J\r\n\
        FL-7LJLJ||||||LJL-77\r\n\
        F--JF--7||LJLJ7F7FJ-\r\n\
        L---JF-JLJ.||-FJLJJ7\r\n\
        |F|F-JF---7F7-L7L|7|\r\n\
        |FFJF7L7F-JF7|JL---7\r\n\
        7-L-JL7||F7|L7F-7F7|\r\n\
        L.L7LFJ|||||FJL7||LJ\r\n\
        L7JLJL-JLJLJL--JLJ.L\r\n";
        let map = parse_input(input).unwrap();
        let adjacency_matrix = compute_adjacency_matrix(&map);
        let main_loop = compute_main_loop(&map, &adjacency_matrix);
        let tiles_contained_by_main_loop = compute_tiles_contained_by_main_loop(&map, &main_loop);

        assert_eq!(tiles_contained_by_main_loop.len(), 10);
    }
}
