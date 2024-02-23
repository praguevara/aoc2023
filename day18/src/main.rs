use std::ops::{Add, Div};

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{anychar, digit1, space1},
    combinator::{map, map_res},
    sequence::Tuple,
    IResult,
};

#[derive(Debug, Copy, Clone)]
struct Instruction {
    direction: char,
    distance: i32,
    color: (u8, u8, u8),
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    nom::Parser::parse(
        &mut map_res(take_while_m_n(2, 2, is_hex_digit), from_hex),
        input,
    )
}

fn hex_color(input: &str) -> IResult<&str, (u8, u8, u8)> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;

    Ok((input, (red, green, blue)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = anychar(input)?;
    let (input, _) = space1(input)?;
    let (input, distance) = digit1(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, (red, green, blue)) = hex_color(input)?;
    let (input, _) = tag(")")(input)?;

    let instruction = Instruction {
        direction,
        distance: distance.parse::<i32>().unwrap(),
        color: (red, green, blue),
    };

    Ok((input, instruction))
}

fn parse_hex_five_digits(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("#")(input)?;
    nom::Parser::parse(
        &mut map_res(take_while_m_n(5, 5, is_hex_digit), |s: &str| {
            usize::from_str_radix(s, 16)
        }),
        input,
    )
}

fn parse_direction_from_hex(input: &str) -> IResult<&str, char> {
    // 0 means R
    // 1 means D
    // 2 means L
    // 3 means U

    let (input, direction) = map(
        nom::branch::alt((tag("0"), tag("1"), tag("2"), tag("3"))),
        |s: &str| match s {
            "0" => 'R',
            "1" => 'D',
            "2" => 'L',
            "3" => 'U',
            _ => unreachable!(),
        },
    )(input)?;

    Ok((input, direction))
}

fn parse_instruction_second_part(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = anychar(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("(")(input)?;

    let (input, distance) = parse_hex_five_digits(input)?;
    let (input, direction) = parse_direction_from_hex(input)?;

    let (input, _) = tag(")")(input)?;

    let instruction = Instruction {
        direction,
        distance: distance as i32,
        color: (0, 0, 0),
    };

    Ok((input, instruction))
}

fn parse_input(input_str: &str) -> Result<Vec<Instruction>, nom::Err<nom::error::Error<&str>>> {
    let lines = input_str.lines();

    let instructions: Result<Vec<_>, _> = lines
        .map(|line| parse_instruction(line).map(|i| i.1))
        .collect();
    instructions
}

fn parse_input_second_part(
    input_str: &str,
) -> Result<Vec<Instruction>, nom::Err<nom::error::Error<&str>>> {
    let lines = input_str.lines();

    let instructions: Result<Vec<_>, _> = lines
        .map(|line| parse_instruction_second_part(line).map(|i| i.1))
        .collect();
    instructions
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

fn run_instructions(instructions: Vec<Instruction>) -> Vec<Coord> {
    let mut holes_dug = Vec::new();
    let mut current_position = Coord { x: 0, y: 0 };
    for Instruction {
        direction,
        distance,
        color: _color,
    } in instructions
    {
        for _ in 0..distance {
            match direction {
                'L' => current_position.x -= 1,
                'R' => current_position.x += 1,
                'U' => current_position.y += 1,
                'D' => current_position.y -= 1,
                _ => panic!("Unknown direction"),
            }
            holes_dug.push(current_position);
        }
    }
    holes_dug
}

fn shoelace_formula(points: &[Coord]) -> isize {
    let len = points.len();

    let (area, perimeter) =
        points
            .iter()
            .enumerate()
            .fold((0isize, 0isize), |(sum, perimeter), (i, p1)| {
                let l = (i + 1) % len;
                let p2 = points[l];

                let new_perimeter = perimeter + ((p2.x - p1.x).abs() + (p2.y - p1.y).abs());
                let new_area = sum + ((p1.y * p2.x) - (p1.x * p2.y));

                (new_area, new_perimeter)
            });

    area.abs().add(perimeter).div(2).add(1)
}

fn main() {
    let input_str = include_str!("../input.txt");
    let instructions = parse_input(input_str).unwrap();
    // dbg!(&instructions);

    let holes_dug = run_instructions(instructions);
    let mut points: Vec<Coord> = holes_dug.clone();
    points.push(holes_dug[0]);
    let holes = shoelace_formula(&points);
    println!("Number of holes dug: {}", holes);

    // part 2
    let instructions = parse_input_second_part(input_str).unwrap();

    let holes_dug = run_instructions(instructions);
    let mut points: Vec<Coord> = holes_dug.clone();
    points.push(holes_dug[0]);
    let holes = shoelace_formula(&points);
    println!("Number of holes dug: {}", holes);
}
