fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.as_bytes().to_vec())
        .collect()
}

fn hash(input: &[u8]) -> u8 {
    let mut current: u16 = 0;

    for c in input {
        current += *c as u16;
        current *= 17;
        current %= 256;
    }

    current as u8
}

fn main() {
    let input_str = include_str!("../input.txt");
    let input = parse_input(input_str);
    let result = input.iter().map(|v| hash(v) as usize).sum::<usize>();
    println!("result: {}", result);
}
