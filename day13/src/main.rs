use nom::{
    branch::alt,
    character::{complete::char, complete::line_ending},
    combinator::map,
    multi::{many1, separated_list0, separated_list1},
    sequence::terminated,
    IResult,
};

type Pattern = Vec<Vec<bool>>;

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    let (input, pattern) = many1(terminated(
        many1(alt((map(char('.'), |_| false), map(char('#'), |_| true)))),
        line_ending,
    ))(input)?;

    Ok((input, pattern))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Pattern>> {
    let (input, patterns) = separated_list1(line_ending, parse_pattern)(input)?;

    Ok((input, patterns))
}

#[test]
fn test_parse_input() {
    let input = include_str!("../sample.txt");
    let patterns = parse_input(input).unwrap().1;
    assert_eq!(patterns.len(), 2);
}

fn vertical_reflection_errors(pattern_line: &Vec<bool>, reflects_after: usize) -> usize {
    debug_assert!(reflects_after < pattern_line.len() - 1);

    let mut errors = 0;
    let max_reflection_len =
        usize::min(reflects_after + 1, pattern_line.len() - 1 - reflects_after);

    for i in 0..max_reflection_len {
        let left_idx = reflects_after - i;
        let right_idx = reflects_after + i + 1;

        let (left, right) = (pattern_line[left_idx], pattern_line[right_idx]);
        if left != right {
            errors += 1;
        }
    }
    errors
}

#[test]
fn test_has_vertical_reflection() {
    let pattern = vec![vec![true, true, false, false, true, true]];
    assert_eq!(vertical_reflection_errors(&pattern[0], 2), 0);
}

fn find_vertical_reflection(pattern: &Pattern, errors: usize) -> Option<usize> {
    (0..pattern[0].len() - 1)
        .find(|&i| {
            pattern
                .iter()
                .map(|line| vertical_reflection_errors(line, i))
                .sum::<usize>()
                == errors
        })
        .map(|x| x + 1)
}

fn horizontal_reflection_errors(pattern: &Pattern, reflects_after: usize) -> usize {
    debug_assert!(reflects_after < pattern.len() - 1);

    let mut errors = 0;
    let max_reflection_len = usize::min(reflects_after + 1, pattern.len() - 1 - reflects_after);

    for i in 0..max_reflection_len {
        let top_idx = reflects_after - i;
        let bottom_idx = reflects_after + i + 1;

        for (a, b) in pattern[top_idx].iter().zip(pattern[bottom_idx].iter()) {
            if a != b {
                errors += 1;
            }
        }
    }
    errors
}

fn find_horizontal_reflection(pattern: &Pattern, errors: usize) -> Option<usize> {
    (0..pattern.len() - 1)
        .find(|&i| horizontal_reflection_errors(pattern, i) == errors)
        .map(|x| x + 1)
}

fn find_reflection(pattern: &Pattern, errors: usize) -> (Option<usize>, Option<usize>) {
    let v = find_vertical_reflection(pattern, errors);
    if v.is_some() {
        (v, None)
    } else {
        (None, find_horizontal_reflection(pattern, errors))
    }
}

fn summarize_reflections(
    reflections: impl Iterator<Item = (Option<usize>, Option<usize>)>,
) -> usize {
    reflections
        .map(|(x, y)| match (x, y) {
            (_, Some(y)) => 100 * y,
            (Some(x), _) => x,
            _ => panic!("No reflection found"),
        })
        .sum::<usize>()
}

fn part_1(patterns: &[Pattern]) -> usize {
    let reflections = patterns.iter().map(|p| find_reflection(p, 0));
    summarize_reflections(reflections)
}

fn part_2(patterns: &[Pattern]) -> usize {
    let reflections = patterns.iter().map(|p| find_reflection(p, 1));
    summarize_reflections(reflections)
}

fn main() {
    let input = include_str!("../input.txt");

    let patterns = parse_input(input).unwrap().1;

    let part_1_answer = part_1(&patterns);
    println!("{}", part_1_answer);

    let part_2_answer = part_2(&patterns);
    println!("{}", part_2_answer);
}
