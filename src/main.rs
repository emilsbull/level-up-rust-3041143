#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut havs_ace = 0;
        let mut total = 0;

        for card in &self.cards {
            match card {
                Card::Ace => havs_ace += 1,
                Card::Two => total += 2,
                Card::Three => total += 3,
                Card::Four => total += 4,
                Card::Five => total += 5,
                Card::Six => total += 6,
                Card::Seven => total += 7,
                Card::Eight => total += 8,
                Card::Nine => total += 9,
                Card::Jack | Card::Queen | Card::King => total += 10,
            }
        }

        while havs_ace > 0 {
            if total + 11 <= 21 {
                total += 11;
            } else {
                total += 1;
            }
            havs_ace -= 1;
        }

        total
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);
}

#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);

    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 22);
}
