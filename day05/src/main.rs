fn main() {
    let input_str = include_str!("../input.txt");
    let input = day05::parse_input(input_str);

    println!("{}", day05::lowest_location_intervals(&input));
}
