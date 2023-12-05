#[derive(Debug)]
struct Input<'a> {
    seeds: Vec<SeedRange>,
    maps: Vec<Map<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
struct Map<'a> {
    name: &'a str,
    ranges: Vec<Range>,
}

impl Map<'_> {
    fn map(&self, value: i64) -> i64 {
        self.ranges
            .iter()
            .find_map(|r| r.convert(value))
            .unwrap_or(value)
    }

    fn unmap(&self, value: i64) -> i64 {
        let unmapped = self
            .ranges
            .iter()
            .find_map(move |r| r.unconvert(value))
            .unwrap();

        unmapped
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Range {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: usize,
}

impl Range {
    fn is_contained_in_source_range(&self, value: i64) -> bool {
        self.source_range_start <= value
            && value < self.source_range_start + self.range_length as i64
    }

    fn convert(&self, value: i64) -> Option<i64> {
        self.is_contained_in_source_range(value)
            .then_some(self.destination_range_start + (value - self.source_range_start))
    }

    fn unconvert(&self, value: i64) -> Option<i64> {
        let is_contained_in_destination_range = self.destination_range_start <= value
            && value < self.destination_range_start + self.range_length as i64;

        is_contained_in_destination_range
            .then_some(self.source_range_start + (value - self.destination_range_start))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SeedRange {
    start: i64,
    length: usize,
}

impl SeedRange {
    fn iter(&self) -> impl Iterator<Item = i64> {
        self.start..self.start + self.length as i64
    }

    fn contains(&self, value: i64) -> bool {
        self.start <= value && value < self.start + self.length as i64
    }
}

fn parse_input(input: &str) -> Input {
    let mut parts = {
        let mut parts = input
            .lines()
            .fold((vec![], vec![]), |(mut acc, mut curr), l| {
                if l.is_empty() {
                    acc.push(curr);
                    (acc, vec![])
                } else {
                    curr.push(l);
                    (acc, curr)
                }
            });

        parts.0.push(parts.1);
        parts.0.into_iter()
    };

    let seeds: Vec<SeedRange> = parts
        .next()
        .unwrap()
        .first()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .fold((vec![], None), |(mut acc, start), s| match start {
            None => (acc, Some(s.parse::<i64>().unwrap())),
            Some(start) => {
                let size = s.parse::<usize>().unwrap();
                acc.push(SeedRange {
                    start,
                    length: size,
                });
                (acc, None)
            }
        })
        .0;

    let maps = parts
        .map(|ls| {
            let name = ls.first().unwrap().split(':').next().unwrap();
            let ranges = ls
                .iter()
                .skip(1)
                .map(|l| {
                    let mut parts = l.split_whitespace();
                    let destination_range_start = parts.next().unwrap().parse::<i64>().unwrap();
                    let source_range_start = parts.next().unwrap().parse::<i64>().unwrap();
                    let range_length = parts.next().unwrap().parse::<usize>().unwrap();
                    Range {
                        destination_range_start,
                        source_range_start,
                        range_length,
                    }
                })
                .collect::<Vec<_>>();

            Map { name, ranges }
        })
        .collect();

    Input { seeds, maps }
}

fn lowest_location_bf(input: &Input) -> i64 {
    input
        .seeds
        .iter()
        .flat_map(|s| s.iter())
        .map(|s| input.maps.iter().fold(s, |acc, m| m.map(acc)))
        .min()
        .unwrap()
}

fn lowest_location_backwards(input: &Input) -> i64 {
    let maps_in_reverse = input.maps.iter().rev().collect::<Vec<_>>();

    (0..)
        .flat_map(|location| {
            let mut current_value = location;

            for map in maps_in_reverse.iter() {
                current_value = map.unmap(current_value);
            }

            if input.seeds.iter().any(|s| s.contains(current_value)) {
                Some(location)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn main() {
    let input_str = include_str!("../input.txt");
    let input = parse_input(input_str);
    println!("{}", lowest_location_backwards(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_2() {
        let input_str = include_str!("../sample.txt");
        let input = parse_input(input_str);
        assert_eq!(
            input.seeds,
            vec![
                SeedRange {
                    start: 79,
                    length: 14
                },
                SeedRange {
                    start: 55,
                    length: 13
                }
            ]
        );
        assert_eq!(input.maps.len(), 7);
        assert_eq!(input.maps[0].name, "seed-to-soil map");
        assert_eq!(input.maps[0].ranges.len(), 2);
        assert_eq!(
            input.maps[0].ranges[0],
            Range {
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2
            }
        );
    }

    #[test]
    fn test_convert() {
        let range = Range {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };
        assert_eq!(range.convert(97), None);
        assert_eq!(range.convert(98), Some(50));
        assert_eq!(range.convert(99), Some(51));
        assert_eq!(range.convert(100), None);
    }

    #[test]
    fn test_map() {
        let map = Map {
            name: "seed-to-soil map",
            ranges: vec![
                Range {
                    destination_range_start: 50,
                    source_range_start: 98,
                    range_length: 2,
                },
                Range {
                    destination_range_start: 52,
                    source_range_start: 50,
                    range_length: 48,
                },
            ],
        };

        assert_eq!(map.map(79), 81);
        assert_eq!(map.map(14), 14);
        assert_eq!(map.map(55), 57);
        assert_eq!(map.map(13), 13);
        assert_eq!(map.map(98), 50);
        assert_eq!(map.map(99), 51);
        assert_eq!(map.map(53), 55);
        assert_eq!(map.map(10), 10);
    }

    #[test]
    fn test_lowest_location_bf() {
        let input_str = include_str!("../sample.txt");
        let input = parse_input(input_str);
        assert_eq!(lowest_location_bf(&input), 46);
    }

    #[test]
    fn test_unmap() {
        let map = Map {
            name: "seed-to-soil map",
            ranges: vec![
                Range {
                    destination_range_start: 50,
                    source_range_start: 98,
                    range_length: 2,
                },
                Range {
                    destination_range_start: 52,
                    source_range_start: 50,
                    range_length: 48,
                },
            ],
        };

        assert_eq!(map.unmap(81), 79);
        assert_eq!(map.unmap(14), 14);
        assert_eq!(map.unmap(57), 55);
        assert_eq!(map.unmap(13), 13);
        assert_eq!(map.unmap(50), 98);
        assert_eq!(map.unmap(51), 99);
        assert_eq!(map.unmap(55), 53);
        assert_eq!(map.unmap(10), 10);
    }

    #[test]
    fn test_lowest_location_backwards() {
        let input_str = include_str!("../sample.txt");
        let input = parse_input(input_str);
        assert_eq!(lowest_location_backwards(&input), 46);
    }
}
