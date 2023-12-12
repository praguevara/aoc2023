#![feature(iter_intersperse)]
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Spring::Working),
            '.' => Ok(Spring::Broken),
            '?' => Ok(Spring::Unknown),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    contiguous_working_springs: Vec<usize>,
}

impl TryFrom<&str> for Row {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split_ws = value.split_whitespace();
        let springs = split_ws
            .next()
            .ok_or(())?
            .chars()
            .map(|c| c.try_into().unwrap())
            .collect();

        let contiguous_working_springs = split_ws
            .next()
            .ok_or(())?
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Row {
            springs,
            contiguous_working_springs,
        })
    }
}

fn parse_input(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| Row::try_from(line).unwrap())
        .collect()
}

type Cache = HashMap<(Vec<Spring>, Option<usize>, Vec<usize>), usize>;

fn explore_row(
    cache: &mut Cache,
    springs: &[Spring],
    current_group: Option<usize>,
    remaining_continuous: &[usize],
) -> usize {
    if springs.is_empty() {
        return match (current_group, remaining_continuous.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == remaining_continuous[0] => 1,
            _ => 0,
        };
    }

    if current_group.is_some() && remaining_continuous.is_empty() {
        // couldn't clone working group
        return 0;
    }

    if let Some(ans) = cache.get(&(
        springs.to_vec(),
        current_group,
        remaining_continuous.to_vec(),
    )) {
        return *ans;
    }

    let ans = match (springs[0], current_group) {
        (Spring::Broken, Some(x)) if x != remaining_continuous[0] => 0,
        (Spring::Broken, Some(_)) => {
            explore_row(cache, &springs[1..], None, &remaining_continuous[1..])
        }
        (Spring::Broken, None) => explore_row(cache, &springs[1..], None, remaining_continuous),
        (Spring::Working, Some(_)) => explore_row(
            cache,
            &springs[1..],
            current_group.map(|x| x + 1),
            remaining_continuous,
        ),
        (Spring::Working, None) => explore_row(cache, &springs[1..], Some(1), remaining_continuous),
        (Spring::Unknown, Some(x)) => {
            let mut ans = explore_row(
                cache,
                &springs[1..],
                current_group.map(|x| x + 1),
                remaining_continuous,
            );
            if x == remaining_continuous[0] {
                ans += explore_row(cache, &springs[1..], None, &remaining_continuous[1..])
            }
            ans
        }
        (Spring::Unknown, None) => {
            explore_row(cache, &springs[1..], Some(1), remaining_continuous)
                + explore_row(cache, &springs[1..], None, remaining_continuous)
        }
    };

    cache.insert(
        (
            springs.to_vec(),
            current_group,
            remaining_continuous.to_vec(),
        ),
        ans,
    );

    ans
}

fn possible_arrangements(row: &Row) -> usize {
    explore_row(
        &mut HashMap::new(),
        &row.springs,
        None,
        &row.contiguous_working_springs,
    )
}

fn main() {
    let input = include_str!("../input.txt");
    let rows = parse_input(input);
    let arrangements = rows.iter().map(possible_arrangements).sum::<usize>();

    println!("{:?}", arrangements);

    // part 2
    let rows = rows
        .iter()
        .map(|row| {
            let springs = (0..5)
                .map(|_| row.springs.clone())
                .intersperse(vec![Spring::Unknown])
                .flatten()
                .collect();

            let contiguous_working_springs = row.contiguous_working_springs.repeat(5);

            Row {
                springs,
                contiguous_working_springs,
            }
        })
        .collect::<Vec<_>>();

    let arrangements = rows.iter().map(possible_arrangements).sum::<usize>();

    println!("{:?}", arrangements);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explore_row() {
        let input = parse_input("#.#.### 1,1,3");
        let row = &input[0];
        assert_eq!(possible_arrangements(row), 0);
    }

    #[test]
    fn test_explore_row_2() {
        let input = parse_input("???.### 1,1,3");
        let row = &input[0];
        assert_eq!(possible_arrangements(row), 1);
    }

    #[test]
    fn test_explore_row_3() {
        let input = parse_input(".??..??...?##. 1,1,3");
        let row = &input[0];
        assert_eq!(possible_arrangements(row), 4);
    }
}
