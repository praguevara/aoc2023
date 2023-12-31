#[derive(Debug, PartialEq, Eq, Clone)]
struct Race {
    time: i64,            // milliseconds
    record_distance: i64, // millimeters
}

impl Race {
    const fn new(time: i64, record_distance: i64) -> Self {
        Self {
            time,
            record_distance,
        }
    }
}

fn parse_input_first_part(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap());

    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap());

    times.zip(distances).map(|(t, r)| Race::new(t, r)).collect()
}

fn parse_input_second_part(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .fold(String::new(), |a, b| format!("{}{}", a, b))
        .parse::<i64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .fold(String::new(), |a, b| format!("{}{}", a, b))
        .parse::<i64>()
        .unwrap();

    Race::new(time, distance)
}

fn compute_race_distance(race_time: i64, held_button_time: i64) -> i64 {
    let boat_speed = held_button_time;
    let remaining_race_time = race_time - held_button_time;
    boat_speed * remaining_race_time
}

fn ways_to_beat_record(race: &Race) -> impl Iterator<Item = i64> + '_ {
    (0..=race.time)
        .map(|held_button_time| {
            (
                held_button_time,
                compute_race_distance(race.time, held_button_time),
            )
        })
        .filter(|(_, new_distance)| *new_distance > race.record_distance)
        .map(|(held_button_time, _)| (held_button_time))
}

fn ways_to_beat_record_math(race: &Race) -> std::ops::Range<i64> {
    // distance = speed * (time - speed)

    // Solving for speed
    let first_term = race.time as f64 / 2.0;
    let second_term = f64::sqrt((race.time * race.time - 4 * race.record_distance) as f64) / 2.0;

    // We take ceiling and floor since we only care about those speeds that beat the record
    let speed_l = f64::floor(first_term - second_term) as i64;
    let speed_r = f64::ceil(first_term + second_term) as i64;

    speed_l + 1..speed_r
}

fn total_ways_to_beat_record_product(races: impl IntoIterator<Item = Race>) -> i64 {
    races
        .into_iter()
        .map(|race| ways_to_beat_record(&race).count() as i64)
        .product()
}

fn main() {
    let input = include_str!("../input.txt");
    let race = parse_input_second_part(input);
    println!("{}", ways_to_beat_record_math(&race).count());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "Time:      7  15   30\r\nDistance:  9  40  200\r\n"
    }

    #[test]
    fn test_parse_first_part() {
        let input = sample_input();
        let races = parse_input_first_part(input);
        assert_eq!(races[0], Race::new(7, 9));
        assert_eq!(races[1], Race::new(15, 40));
        assert_eq!(races[2], Race::new(30, 200));
    }

    #[test]
    fn test_parse_second_part() {
        let input = sample_input();
        let race = parse_input_second_part(input);
        assert_eq!(race, Race::new(71530, 940200));
    }

    #[test]
    fn test_parse_second_part_math() {
        let input = sample_input();
        let race = parse_input_second_part(input);
        let ways_count = ways_to_beat_record_math(&race).count();
        assert_eq!(ways_count, 71503);
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

    #[test]
    fn test_ways_to_beat_record_math() {
        let race = Race::new(7, 9);
        let ways = ways_to_beat_record_math(&race).collect::<Vec<_>>();
        assert_eq!(ways, vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_total_ways_to_beat_record() {
        let race = Race::new(7, 9);
        let ways_count = ways_to_beat_record_math(&race).count();
        assert_eq!(ways_count, 4);
    }

    #[test]
    fn test_total_ways_to_beat_record_2() {
        let race = Race::new(71530, 940200);
        let ways_count = ways_to_beat_record_math(&race).count();
        assert_eq!(ways_count, 71503);
    }

    #[test]
    fn test_math() {
        // [0, 1, 2, 3, 4] -> [0, 3, 4, 3, 0]
        let r0 = Race::new(4, 0); // Can be beaten by [1, 2, 3] -> 3
        let r1 = Race::new(4, 1); // Can be beaten by [1, 2, 3] -> 3
        let r2 = Race::new(4, 2); // Can be beaten by [1, 2, 3] -> 3
        let r3 = Race::new(4, 3); // Can be beaten by [2] -> 1
        let r4 = Race::new(4, 4); // Can be beaten by [] -> 0
        let ways = [r0, r1, r2, r3, r4]
            .iter()
            .map(ways_to_beat_record_math)
            .map(Iterator::count)
            .collect::<Vec<_>>();
        assert_eq!(ways, vec![3, 3, 3, 1, 0]);
    }
}
