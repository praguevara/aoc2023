type SeedRange = std::ops::Range<i64>;

#[derive(Debug)]
pub struct Input<'a> {
    seeds: Vec<SeedRange>,
    maps: Vec<Map<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Map<'a> {
    name: &'a str,
    ranges: Vec<MapRange>,
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
            .unwrap_or(value);

        unmapped
    }

    fn map_range(&self, range: &SeedRange) -> Vec<SeedRange> {
        let mut ranges_to_map = vec![range.clone()];
        let mut mappings = vec![];

        while let Some(range) = ranges_to_map.pop() {
            let mut does_overlap = false;
            for map_range in self.ranges.iter() {
                let overlap_start = range.start.max(map_range.source_range_start);
                let overlap_end = range
                    .end
                    .min(map_range.source_range_start + map_range.range_length);
                if overlap_start < overlap_end {
                    // There is an overlap
                    mappings.push(
                        (overlap_start - map_range.source_range_start
                            + map_range.destination_range_start)
                            ..(overlap_end - map_range.source_range_start
                                + map_range.destination_range_start),
                    );

                    if overlap_start > range.start {
                        // There is a range before the overlap
                        // Since we could have other mappings, we need to check again
                        ranges_to_map.push(range.start..overlap_start);
                    }
                    if range.end > overlap_end {
                        // There is a range after the overlap
                        // Since we could have other mappings, we need to check again
                        ranges_to_map.push(overlap_end..range.end);
                    }

                    does_overlap = true;
                    break;
                }
            }

            if !does_overlap {
                mappings.push(range.clone());
            }
        }

        mappings
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MapRange {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl MapRange {
    fn is_contained_in_source_range(&self, value: i64) -> bool {
        self.source_range_start <= value && value < self.source_range_start + self.range_length
    }

    fn convert(&self, value: i64) -> Option<i64> {
        self.is_contained_in_source_range(value)
            .then_some(self.destination_range_start + (value - self.source_range_start))
    }

    fn unconvert(&self, value: i64) -> Option<i64> {
        let is_contained_in_destination_range = self.destination_range_start <= value
            && value < self.destination_range_start + self.range_length;

        is_contained_in_destination_range
            .then_some(self.source_range_start + (value - self.destination_range_start))
    }
}

pub fn parse_input(input: &str) -> Input {
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
                let size = s.parse::<i64>().unwrap();
                acc.push(start..start + size);
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
                    let range_length = parts.next().unwrap().parse::<i64>().unwrap();
                    MapRange {
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

// Very slow, DNF
pub fn lowest_location_bf(input: &Input) -> i64 {
    input
        .seeds
        .iter()
        .flat_map(|s| s.clone())
        .map(|s| input.maps.iter().fold(s, |acc, m| m.map(acc)))
        .min()
        .unwrap()
}

// OK but
pub fn lowest_location_backwards(input: &Input) -> i64 {
    let maps_in_reverse = input.maps.iter().rev().collect::<Vec<_>>();

    (0..)
        .flat_map(|location| {
            let mut current_value = location;

            for map in maps_in_reverse.iter() {
                current_value = map.unmap(current_value);
            }

            if input.seeds.iter().any(|s| s.contains(&current_value)) {
                Some(location)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

pub fn lowest_location_intervals(input: &Input) -> i64 {
    input
        .maps
        .iter()
        .fold(input.seeds.clone(), |seeds, map| {
            seeds
                .iter()
                .flat_map(|s| map.map_range(s))
                .collect::<Vec<_>>()
        })
        .iter()
        .map(|r| r.start)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_2() {
        let input_str = include_str!("../sample.txt");
        let input = parse_input(input_str);
        assert_eq!(input.seeds, vec![(79..79 + 14), (55..55 + 13)]);
        assert_eq!(input.maps.len(), 7);
        assert_eq!(input.maps[0].name, "seed-to-soil map");
        assert_eq!(input.maps[0].ranges.len(), 2);
        assert_eq!(
            input.maps[0].ranges[0],
            MapRange {
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2
            }
        );
    }

    #[test]
    fn test_convert() {
        let range = MapRange {
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
                MapRange {
                    destination_range_start: 50,
                    source_range_start: 98,
                    range_length: 2,
                },
                MapRange {
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
                MapRange {
                    destination_range_start: 50,
                    source_range_start: 98,
                    range_length: 2,
                },
                MapRange {
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

    #[test]
    fn test_convert_range() {
        let input_str = include_str!("../sample.txt");
        let input = parse_input(input_str);
        let map = &input.maps[0];
        let mappings = map.map_range(&input.seeds[0]);
        assert_eq!(mappings, vec![(81..95)]);
    }

    #[test]
    fn test_lowest_location_intervals() {
        let input_str = include_str!("../sample.txt");
        let input = parse_input(input_str);

        let res = lowest_location_intervals(&input);

        assert_eq!(res, 46);
    }
}
