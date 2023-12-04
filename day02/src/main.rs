#[derive(Debug, Clone)]
pub struct Game {
    id: i32,
    sets: Vec<Set>,
}

#[derive(Debug, Clone)]
pub struct Set {
    reds: i32,
    greens: i32,
    blues: i32,
}

fn parse_input(input: &str) -> Vec<Game> {
    let lines = input.lines();
    let games = lines.map(parse_game);
    games.collect()
}

fn parse_game(input: &str) -> Game {
    // First split at the colon
    let mut colon_split = input.split(':');
    let before_colon = colon_split.next().unwrap();

    // Second word before the colon is the id
    let id: i32 = before_colon.split(' ').nth(1).unwrap().parse().unwrap();

    let after_colon = colon_split.next().unwrap();
    // Each set is separated by a semicolon
    let sets = after_colon.split(';').map(parse_set).collect();

    Game { id, sets }
}

fn parse_set(input: &str) -> Set {
    let mut reds = 0;
    let mut greens = 0;
    let mut blues = 0;

    let comma_splits = input.split(',');

    for split in comma_splits {
        let trimmed = split.trim();
        let mut split = trimmed.split(' ');
        let count: i32 = split.next().unwrap().parse().unwrap();
        let color = split.next().unwrap();

        match color {
            "red" => reds = count,
            "green" => greens = count,
            "blue" => blues = count,
            _ => panic!("Unknown color: {}", color),
        }
    }

    Set {
        reds,
        greens,
        blues,
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let games = parse_input(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 1 green, 1 blue, 1 red; 3 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green";
        let game = parse_game(input);

        assert_eq!(game.id, 1);
        assert_eq!(game.sets.len(), 5);
        assert_eq!(game.sets[0].reds, 1);
        assert_eq!(game.sets[0].greens, 1);
        assert_eq!(game.sets[0].blues, 1);
    }
}
