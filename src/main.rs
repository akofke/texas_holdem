extern crate rand;

use std::fmt;
use rand::{thread_rng, Rng};

const SUITS: [Suit; 4] = [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club];

const SPADE: char = '\u{2660}';
const HEART: char = '\u{2665}';
const DIAMOND: char = '\u{2666}';
const CLUB: char = '\u{2663}';

const RANKS: [Rank; 13] = [Rank::A,
                           Rank::K,
                           Rank::Q,
                           Rank::J,
                           Rank::Ten,
                           Rank::Nine,
                           Rank::Eight,
                           Rank::Seven,
                           Rank::Six,
                           Rank::Five,
                           Rank::Four,
                           Rank::Three,
                           Rank::Two];

#[derive(Debug, Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug, Clone, Copy)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Debug, Clone, Copy)]
enum Rank {
    A = 13,
    K = 12,
    Q = 11,
    J = 10,
    Ten = 9,
    Nine = 8,
    Eight = 7,
    Seven = 6,
    Six = 5,
    Five = 4,
    Four = 3,
    Three = 2,
    Two = 1,
}

impl Card {
    fn new(rank: &Rank, suit: &Suit) -> Card {
        Card {
            rank: *rank,
            suit: *suit,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{rank:?}{suit}",
               rank = self.rank,
               suit = match self.suit {
                   Suit::Spade => SPADE,
                   Suit::Heart => HEART,
                   Suit::Diamond => DIAMOND,
                   Suit::Club => CLUB,
               })
    }
}

impl Deck {
    fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();

        for suit in SUITS.iter() {
            for rank in RANKS.iter() {
                cards.push(Card::new(rank, suit));
            }
        }

        Deck { cards: cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(&mut self.cards[..]);
    }        
}


fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    for card in deck.cards {
        println!("{}", card);
    }

}
