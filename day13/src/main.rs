use std::ops::BitXor;

use nom::{
    branch::alt,
    character::{complete::char, complete::line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

use ndarray::prelude::*;

type Pattern = Array2<u8>;
fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    let (input, pattern_vec) = many1(terminated(
        many1(alt((map(char('.'), |_| 0), map(char('#'), |_| 1)))),
        line_ending,
    ))(input)?;

    let rows = pattern_vec.len();
    let cols = if rows > 0 { pattern_vec[0].len() } else { 0 };
    let pattern = Array2::from_shape_fn((rows, cols), |(i, j)| pattern_vec[i][j]);

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

fn vertical_reflection_errors(pattern: &Pattern, reflects_after: usize) -> usize {
    let max_reflection_len = usize::min(reflects_after + 1, pattern.ncols() - 1 - reflects_after);

    let left_cols = pattern.slice(s![
        ..,
        reflects_after + 1 - max_reflection_len..reflects_after + 1
    ]);
    let right_cols_mirrored = pattern.slice(s![
        ..,
        reflects_after + 1..reflects_after + 1 + max_reflection_len
    ; -1]);

    left_cols.bitxor(&right_cols_mirrored).sum() as usize
}

fn find_vertical_reflection(pattern: &Pattern, errors: usize) -> Option<usize> {
    (0..pattern.ncols() - 1)
        .find(|&i| vertical_reflection_errors(pattern, i) == errors)
        .map(|x| x + 1)
}

fn horizontal_reflection_errors(pattern: &Pattern, reflects_after: usize) -> usize {
    let max_reflection_len = usize::min(reflects_after + 1, pattern.nrows() - 1 - reflects_after);

    let top_rows = pattern.slice(s![
        reflects_after + 1 - max_reflection_len..reflects_after + 1,
        ..
    ]);
    let bottom_rows_mirrored = pattern.slice(s![
        reflects_after + 1..reflects_after + 1 + max_reflection_len ; -1,
        ..
    ]);

    top_rows.bitxor(&bottom_rows_mirrored).sum() as usize
}

fn find_horizontal_reflection(pattern: &Pattern, errors: usize) -> Option<usize> {
    (0..pattern.nrows() - 1)
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
