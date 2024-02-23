use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    ops::{Add, Div},
};

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{anychar, digit1, space1},
    combinator::map_res,
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

fn parse_input(input_str: &str) -> Result<Vec<Instruction>, nom::Err<nom::error::Error<&str>>> {
    let lines = input_str.lines();

    let instructions: Result<Vec<_>, _> = lines
        .map(|line| parse_instruction(line).map(|i| i.1))
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
}
