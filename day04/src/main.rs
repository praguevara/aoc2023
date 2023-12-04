#[derive(Debug)]
pub struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    own_numbers: Vec<i32>,
}

fn parse_input(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.lines().map(parse_card)
}

fn parse_card(input: &str) -> Card {
    let mut split_at_colon = input.split(':');
    // Second word before colon is the id
    let id = split_at_colon
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let mut split_at_pipe = split_at_colon.next().unwrap().split('|');

    // Numbers before pipe are the winning numbers
    let winning_numbers = split_at_pipe
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // Numbers after pipe are the own numbers
    let own_numbers = split_at_pipe
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Card {
        id,
        winning_numbers,
        own_numbers,
    }
}

fn matching_numbers_count(card: &Card) -> usize {
    card.own_numbers
        .iter()
        .filter(|n| card.winning_numbers.contains(n))
        .count()
}

fn points_for_matching_numbers(count: usize) -> i32 {
    match count {
        0 => 0,
        n => 1 << (n - 1),
    }
}

fn process_and_count_cards(cards: impl Iterator<Item = Card>) -> usize {
    // (Card, Number of copies)
    let mut card_vec = cards.map(|c| (c, 1)).collect::<Vec<(Card, usize)>>();

    for i in 0..card_vec.len() {
        let (left, right) = card_vec.split_at_mut(i + 1);
        let (card, copies) = &left[i];

        (0..matching_numbers_count(card)).for_each(|j| {
            right[j].1 += copies;
        });
    }

    card_vec.iter().map(|(_, copies)| copies).sum::<usize>()
}

fn main() {
    let input = include_str!("../input.txt");
    let cards = parse_input(input);
    let total_cards = process_and_count_cards(cards);

    println!("{}", total_cards);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_card(input);
        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.own_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }
}
