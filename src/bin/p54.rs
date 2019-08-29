#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}
use Suit::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Value {
    Num(u8),
    Jack,
    Queen,
    King,
    Ace,
}
use Value::*;
impl Value {
    fn to_u8(&self) -> u8 {
        match self {
            Num(n) => *n,
            Jack => 11,
            Queen => 12,
            King => 13,
            Ace => 14,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Card {
    value: Value,
    suit: Suit,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum PokerRank {
    HighCard(Value),
    OnePair(Value),
    TwoPair(Value, Value),
    ThreeKind(Value),
    Straight(Value), // stores the first value in the straight
    Flush,
    FullHouse(Value, Value), // 3 of a kind then 2 of a kind
    FourKind(Value),
    StraightFlush(Value), // stores the first value in the straight
}
use PokerRank::*;

// these should always be sorted, so don't construct directly
// the derived ordering is the correct ordering when poker rankings are excluded
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn from_array(card_vec: &mut Vec<Card>) -> Hand {
        card_vec.sort();
        return Hand {
            cards: [
                card_vec[0],
                card_vec[1],
                card_vec[2],
                card_vec[3],
                card_vec[4],
            ],
        };
    }

    fn is_flush(&self) -> bool {
        let suit = self.cards[0].suit;
        self.cards.iter().all(|&card| card.suit == suit)
    }

    fn is_straight(&self) -> bool {
        let v = self.cards[0].value.to_u8();
        self.cards
            .iter()
            .enumerate()
            .all(|(e, &card)| card.value.to_u8() == v + e as u8)
    }

    fn count_values(&self) -> [Option<(Value, u8)>; 2] {
        let mut result = [None; 2];
        let mut result_index = 0;

        let mut count = 1;
        let mut last_value = self.cards[0].value;
        for i in 1..5 {
            let this_value = self.cards[i].value;
            if this_value == last_value {
                count += 1;
                last_value = this_value;
            } else if count > 1 {
                result[result_index] = Some((last_value, count));
                count = 1;
                last_value = this_value;
                result_index += 1;
            } else {
                last_value = this_value;
            }
        }
        if count > 1 {
            result[result_index] = Some((last_value, count));
        }
        return result;
    }

    fn to_rank(&self) -> PokerRank {
        match (self.is_flush(), self.is_straight()) {
            (true, true) => return StraightFlush(self.cards[0].value),
            (true, false) => return Flush,
            (false, true) => return Straight(self.cards[0].value),
            (false, false) => (),
        };
        let value_matches = self.count_values();
        match (value_matches[1], value_matches[0]) {
            (Some((val, 4)), _) | (_, Some((val, 4))) => return FourKind(val),
            (Some((val1, 3)), Some((val2, 2))) => return FullHouse(val1, val2),
            (Some((val2, 2)), Some((val1, 3))) => return FullHouse(val1, val2),
            (Some((val, 3)), _) | (_, Some((val, 3))) => return ThreeKind(val),
            (Some((val1, 2)), Some((val2, 2))) => return TwoPair(val1, val2),
            (Some((val, 2)), _) | (_, Some((val, 2))) => return OnePair(val),
            _ => (),
        };
        return HighCard(self.cards[4].value);
    }
}

fn is_winner(you: Hand, other: Hand) -> bool {
    let your_rank = you.to_rank();
    let their_rank = other.to_rank();
    if your_rank > their_rank {
        return true;
    } else if your_rank == their_rank {
        for i in 0..5 {
            if you.cards[4-i].value > other.cards[4-i].value {
                return true;
            } else if you.cards[4-i].value < other.cards[4-i].value {
                return false;
            }
        }
        panic!("No clear winner: {:?} --- {:?}", you, other);
    } else {
        return false;
    }
}

fn string_to_hand(hands: &str) -> Hand {
    let mut cards: Vec<Card> = hands
        .split(" ")
        .map(|c| (c.chars().nth(0).unwrap(), c.chars().nth(1).unwrap()))
        .map(|(v, s)| match s {
            'H' => (v, Heart),
            'D' => (v, Diamond),
            'C' => (v, Club),
            'S' => (v, Spade),
            _ => panic!("Bad suit {} in {}", s, hands.clone()),
        })
        .map(|(v, suit)| match v {
            'A' => (Ace, suit),
            'K' => (King, suit),
            'Q' => (Queen, suit),
            'J' => (Jack, suit),
            'T' => (Num(10), suit),
            _ => (Num(v.to_digit(10).unwrap() as u8), suit),
        })
        .map(|(value, suit)| Card { value, suit })
        .collect();
    return Hand::from_array(&mut cards);
}

fn solve(file: &str) -> u64 {
    read_input(file)
        .lines()
        .map(|line| (string_to_hand(&line[0..14]), string_to_hand(&line[15..29])))
        .filter(|&(h1, h2)| is_winner(h1, h2))
        .count() as u64
}

fn main() -> () {
    let result: u64 = solve(&get_arg_else(1, "p54".to_owned()));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn ordering_test() {
        use super::Value;
        assert!(Value::Num(10) > Value::Num(8));
        assert!(Value::Num(9) > Value::Num(7));
        assert!(!(Value::Num(6) > Value::Num(8)));
        assert!(Value::Jack > Value::Num(7));
        assert!(Value::Ace > Value::Queen);
        assert!(Value::Queen > Value::Jack);
        assert!(Value::King > Value::Num(5));
    }
    #[test]
    fn is_winner_test() {
        use super::string_to_hand;
        let hand1 = string_to_hand("4D QH 6S QC 9H");
        let hand2 = string_to_hand("3D 6D 7H QD QS");
        let hand3 = string_to_hand("2H 2D 4C 4D 4S");
        let hand4 = string_to_hand("2C 3S 8S 8D TD");

        assert_eq!(hand1.cards[4].value, Queen);

        use super::{PokerRank::*, Value::*};
        assert_eq!(hand1.to_rank(), OnePair(Queen));
        assert_eq!(hand2.to_rank(), OnePair(Queen));
        assert_eq!(hand3.to_rank(), FullHouse(Num(4), Num(2)));
        assert_eq!(hand4.to_rank(), OnePair(Num(8)));

        use super::is_winner;
        assert!(is_winner(hand1, hand2));
        assert!(is_winner(hand3, hand2));
        assert!(is_winner(hand2, hand4));
    }
}
