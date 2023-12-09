fn sequence_differences(sequence: &[i32]) -> impl Iterator<Item = i32> + '_ {
    sequence.windows(2).map(|window| window[1] - window[0])
}

fn is_sequence_all_zeros(sequence: &[i32]) -> bool {
    sequence.iter().all(|&n| n == 0)
}

fn reduce_sequence_to_zeros(sequence: &[i32]) -> Vec<Vec<i32>> {
    let mut current_sequence = sequence.to_vec();
    let mut sequences = vec![current_sequence.clone()];

    while !is_sequence_all_zeros(&current_sequence) {
        current_sequence = sequence_differences(&current_sequence)
            .collect::<Vec<i32>>()
            .clone();
        sequences.push(current_sequence.clone());
    }
    sequences
}

fn extrapolate_sequence(sequence: &[i32]) -> i32 {
    let mut next_value = 0;
    for seq in reduce_sequence_to_zeros(sequence).iter().rev() {
        next_value += seq.last().unwrap();
    }
    next_value
}

fn main() {
    let input = include_str!("../input.txt");
    let input_sequences = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let sum_of_extrapolations = input_sequences
        .iter()
        .map(|sequence| extrapolate_sequence(sequence))
        .sum::<i32>();

    println!("{}", sum_of_extrapolations);
}

#[test]
fn test_sequence_differences() {
    let sequence = vec![0, 3, 6, 9, 12, 15];
    let reduced = reduce_sequence_to_zeros(&sequence);
    assert_eq!(&reduced.last(), &Some(&vec![0, 0, 0, 0]));
}

#[test]
fn test_next_element_in_sequence() {
    let sequence = vec![0, 3, 6, 9, 12, 15];
    assert_eq!(extrapolate_sequence(&sequence), 18);
}
