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

#[test]
fn test_parse_pattern() {
    let input = "#..#..##.#.\r\n#..#..##.#.\r\n\r\n";

    parse_pattern(input).unwrap();
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

// ##..## -> len(6)

fn has_vertical_reflection(pattern_line: &Vec<bool>, reflects_after: usize) -> bool {
    // dbg!(reflects_after);
    debug_assert!(reflects_after < pattern_line.len() - 1);

    let max_reflection_len =
        usize::min(reflects_after + 1, pattern_line.len() - 1 - reflects_after);
    // dbg!(max_reflection_len);

    for i in 0..max_reflection_len {
        let left_idx = reflects_after - i;
        let right_idx = reflects_after + i + 1;

        // dbg!(left_idx, right_idx);

        let (left, right) = (pattern_line[left_idx], pattern_line[right_idx]);
        if left != right {
            return false;
        }
    }
    true
}

#[test]
fn test_has_vertical_reflection() {
    let pattern = vec![vec![true, true, false, false, true, true]];
    assert!(has_vertical_reflection(&pattern[0], 0));
    assert!(!has_vertical_reflection(&pattern[0], 1));
    assert!(has_vertical_reflection(&pattern[0], 2));
    assert!(!has_vertical_reflection(&pattern[0], 3));
    assert!(has_vertical_reflection(&pattern[0], 4));
}

fn find_vertical_reflection(pattern: &Pattern) -> Option<usize> {
    (0..pattern[0].len() - 1).find(|&i| pattern.iter().all(|line| has_vertical_reflection(line, i)))
}

#[test]
fn test_find_vertical_reflection() {
    /*
       #.##..##.
       ..#.##.#.
       ##......#
       ##......#
       ..#.##.#.
       ..##..##.
       #.#.##.#.
    */

    let pattern = vec![
        vec![true, false, true, true, false, false, true, true, false],
        vec![false, false, true, false, true, true, false, true, false],
        vec![true, true, false, false, false, false, false, false, true],
        vec![true, true, false, false, false, false, false, false, true],
        vec![false, false, true, false, true, true, false, true, false],
        vec![false, false, true, true, false, false, true, true, false],
        vec![true, false, true, false, true, true, false, true, false],
    ];

    assert_eq!(find_vertical_reflection(&pattern), Some(4));
}

fn has_horizontal_reflection(pattern: &Pattern, reflects_after: usize) -> bool {
    debug_assert!(reflects_after < pattern.len() - 1);

    let max_reflection_len = usize::min(reflects_after + 1, pattern.len() - 1 - reflects_after);

    for i in 0..max_reflection_len {
        let top_idx = reflects_after - i;
        let bottom_idx = reflects_after + i + 1;

        if pattern[top_idx] != pattern[bottom_idx] {
            return false;
        }
    }
    true
}

#[test]
fn test_has_horizontal_reflection() {
    // #...##..#
    // #....#..#
    // ..##..###
    // #####.##.
    // #####.##.
    // ..##..###
    // #....#..#

    let pattern = vec![
        vec![true, false, false, false, true, true, false, false, true],
        vec![true, false, false, false, false, true, false, false, true],
        vec![false, false, true, true, false, false, true, true, true],
        vec![true, true, true, true, true, true, false, true, true],
        vec![true, true, true, true, true, true, false, true, true],
        vec![false, false, true, true, false, false, true, true, true],
        vec![true, false, false, false, false, true, false, false, true],
    ];

    assert!(!has_horizontal_reflection(&pattern, 0));
    assert!(!has_horizontal_reflection(&pattern, 1));
    assert!(!has_horizontal_reflection(&pattern, 2));
    assert!(has_horizontal_reflection(&pattern, 3));
    assert!(!has_horizontal_reflection(&pattern, 4));
    assert!(!has_horizontal_reflection(&pattern, 5));
}

fn find_horizontal_reflection(pattern: &Pattern) -> Option<usize> {
    (0..pattern.len() - 1).find(|&i| has_horizontal_reflection(pattern, i))
}

#[test]
fn test_find_horizontal_reflection() {
    // #...##..#
    // #....#..#
    // ..##..###
    // #####.##.
    // #####.##.
    // ..##..###
    // #....#..#

    let pattern = vec![
        vec![true, false, false, false, true, true, false, false, true],
        vec![true, false, false, false, false, true, false, false, true],
        vec![false, false, true, true, false, false, true, true, true],
        vec![true, true, true, true, true, true, false, true, true],
        vec![true, true, true, true, true, true, false, true, true],
        vec![false, false, true, true, false, false, true, true, true],
        vec![true, false, false, false, false, true, false, false, true],
    ];

    assert_eq!(find_horizontal_reflection(&pattern), Some(3));
}

fn main() {
    let input = include_str!("../input.txt");

    let patterns = parse_input(input).unwrap().1;

    dbg!(&patterns.len());

    let reflections = patterns.iter().map(|pattern| {
        (
            find_vertical_reflection(pattern).map(|x| x + 1),
            find_horizontal_reflection(pattern).map(|x| x + 1),
        )
    });

    let part_1 = reflections
        .map(|(x, y)| x.unwrap_or_default() + 100 * y.unwrap_or_default())
        .sum::<usize>();

    dbg!(part_1);
}
