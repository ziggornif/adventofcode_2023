use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{BufRead, Error},
};
use utils::read_input_file;

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Clone, Copy, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Strength {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn char_to_card(ch: char) -> Option<Card> {
    match ch.to_ascii_uppercase() {
        'J' => Some(Card::Joker),
        '2' => Some(Card::Two),
        '3' => Some(Card::Three),
        '4' => Some(Card::Four),
        '5' => Some(Card::Five),
        '6' => Some(Card::Six),
        '7' => Some(Card::Seven),
        '8' => Some(Card::Eight),
        '9' => Some(Card::Nine),
        'T' => Some(Card::Ten),
        'Q' => Some(Card::Queen),
        'K' => Some(Card::King),
        'A' => Some(Card::Ace),
        _ => None,
    }
}

fn card_to_val(ch: char) -> u8 {
    match ch {
        '0'..='9' => ch.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' => 0,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand: String,
    strength: Strength,
    bid: i32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand == other.hand {
            return Ordering::Equal;
        }
        let self_hand = self.strength;
        let other_hand = other.strength;
        let mut self_chars = self.hand.chars();
        let mut other_chars = other.hand.chars();

        let mut ordering = self_hand.cmp(&other_hand);
        while ordering == Ordering::Equal {
            ordering = card_to_val(self_chars.next().unwrap())
                .cmp(&card_to_val(other_chars.next().unwrap()))
        }
        ordering
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn count_occurrences(arr: &[Card]) -> Strength {
    let mut count_map = HashMap::new();

    for &num in arr {
        let count = count_map.entry(num).or_insert(0);
        *count += 1;
    }

    let jokers = count_map.remove(&Card::Joker).unwrap_or(0);

    let mut counts = count_map.clone().into_values().collect::<Vec<usize>>();

    counts.sort();
    counts.reverse();

    // add jokers to highest count
    let with_jokers = counts.first().unwrap_or(&0) + jokers;
    match with_jokers {
        5 => Strength::FiveKind,
        4 => Strength::FourKind,
        3 => {
            if counts[1] == 2 {
                Strength::FullHouse
            } else {
                Strength::ThreeKind
            }
        }
        2 => {
            if counts[1] == 2 {
                Strength::TwoPair
            } else {
                Strength::OnePair
            }
        }
        1 => Strength::HighCard,
        _ => unreachable!(),
    }
}

fn process(input: String) -> Result<i32, Error> {
    let reader = read_input_file(input)?;

    let input_lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut hands = Vec::<Hand>::new();
    for line in input_lines {
        let mut slices = line.split_whitespace();
        let hand = slices.next().unwrap();
        let bid = slices.next().unwrap().parse::<i32>().unwrap();

        let mut cards_hand: [Card; 5] = [Card::Ace; 5];
        for (idx, ch) in hand.chars().enumerate() {
            cards_hand[idx] = char_to_card(ch).unwrap();
        }
        let strength = count_occurrences(&cards_hand);

        let hand = Hand {
            hand: hand.to_owned(),
            strength,
            bid,
        };
        hands.push(hand);
    }

    hands.sort();
    let mut res = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        res += (i + 1) as i32 * hand.bid;
    }
    Ok(res)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 7 !");

    let result = process("day7/src/resources/input.txt".to_owned())?;

    println!("The result is {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn shoud_get_score() -> Result<(), String> {
        let result = process("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 5905);
        Ok(())
    }
}
