use regex::Regex;

fn first_digit(pattern: &Regex, line: &str) -> i32 {
    let res = pattern.captures(line).unwrap();
    match &res[0] {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => res[0].parse::<i32>().unwrap(),
    }
}

fn last_digit(pattern: &Regex, line: &str) -> i32 {
    let line_bacward = line.chars().rev().collect::<String>();
    let res = pattern.captures(&line_bacward).unwrap();
    match &res[0] {
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        _ => res[0].parse::<i32>().unwrap(),
    }
}

fn line_to_value(first_pattern: &Regex, last_pattern: &Regex, line: &str) -> i32 {
    let first_digit = first_digit(first_pattern, line);
    let last_digit = last_digit(last_pattern, line);
    10 * first_digit + last_digit
}

fn main() {
    let input = include_str!("../input.txt");
    let lines = input.lines();
    let first_pattern = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let last_pattern = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let values = lines.map(|line| line_to_value(&first_pattern, &last_pattern, line));
    let sum: i32 = values.sum();
    println!("{}", sum);
}
