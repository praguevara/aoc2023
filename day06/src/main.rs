#[derive(Debug, PartialEq, Eq, Clone)]
struct Race {
    time: i32,            // milliseconds
    record_distance: i32, // millimeters
}

impl Race {
    const fn new(time: i32, record_distance: i32) -> Self {
        Self {
            time,
            record_distance,
        }
    }
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap());

    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap());

    times.zip(distances).map(|(t, r)| Race::new(t, r)).collect()
}

fn compute_race_distance(race_time: i32, held_button_time: i32) -> i32 {
    let boat_speed = held_button_time;
    let remaining_race_time = race_time - held_button_time;
    boat_speed * remaining_race_time
}

fn ways_to_beat_record(race: &Race) -> impl Iterator<Item = i32> + '_ {
    (0..=race.time)
        .map(|held_button_time| {
            (
                held_button_time,
                compute_race_distance(race.time, held_button_time),
            )
        })
        .filter(|(_, new_distance)| *new_distance > race.record_distance)
        .map(|(held_button_time, _)| held_button_time)
}

fn total_ways_to_beat_record_product(races: impl IntoIterator<Item = Race>) -> i32 {
    races
        .into_iter()
        .map(|race| ways_to_beat_record(&race).count() as i32)
        .product()
}

fn main() {
    let input = include_str!("../input.txt");
    let races = parse_input(input);
    println!("{}", total_ways_to_beat_record_product(races));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "Time:      7  15   30\r\nDistance:  9  40  200\r\n"
    }

    #[test]
    fn test_parse() {
        let input = sample_input();
        let races = parse_input(input);
        assert_eq!(races[0], Race::new(7, 9));
        assert_eq!(races[1], Race::new(15, 40));
        assert_eq!(races[2], Race::new(30, 200));
    }

    #[test]
    fn test_compute_race_time() {
        let race_time = 7;
        assert_eq!(compute_race_distance(race_time, 0), 0);
        assert_eq!(compute_race_distance(race_time, 1), 6);
        assert_eq!(compute_race_distance(race_time, 2), 10);
        assert_eq!(compute_race_distance(race_time, 3), 12);
        assert_eq!(compute_race_distance(race_time, 4), 12);
        assert_eq!(compute_race_distance(race_time, 5), 10);
        assert_eq!(compute_race_distance(race_time, 6), 6);
        assert_eq!(compute_race_distance(race_time, 7), 0);
    }

    #[test]
    fn test_ways_to_beat_record() {
        let race = Race::new(7, 9);
        let ways = ways_to_beat_record(&race).collect::<Vec<_>>();
        assert_eq!(ways, vec![2, 3, 4, 5]);
    }
}
