extern crate rand;

use std::fmt;
use rand::{thread_rng, Rng};
use std::io;

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

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Player {
    id: String,
    hand: Vec<Card>,
    // folded, cash, bet
}

impl Player {
    fn new(id: String) -> Player {
        Player {
            id: id,
            hand: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Game {
    players: Vec<Player>,
    deck: Deck,
    community_cards: Vec<Card>,
    // pot
}

impl Game {
    fn new(num_players: usize) -> Game {
        let mut players = Vec::with_capacity(num_players);
        for i in 0..num_players {
            players.push(Player::new(format!("Player {}", i)));
        }

        let mut deck = Deck::new();
        deck.shuffle();

        Game {
            players: players,
            deck: deck,
            community_cards: Vec::new(),
        }
    }
    
    fn deal(&mut self, num_cards: u8) {
        for _ in 0..num_cards {
            for player in self.players {
                player.hand.push(self.deck.pop());
            }
        }
    }

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

    fn pop(&mut self) -> Card {
        return self.cards.pop().unwrap();
    }
}


fn main() {
    let game = start_game();
    println!("{:#?}", game);
    
}

fn start_game() -> Game {
    println!("Welcome to Texas Hold 'em, please input the number of players (2-8):");
    let mut num_players = String::new();
    loop {
        io::stdin()
            .read_line(&mut num_players)
            .ok()
            .expect("Failed to read line");

        let num_players: usize = match num_players.trim().parse() {
            Ok(num) if num > 1 && num < 9 => num,
            Ok(_) => {
                println!("Please choose a number between 2 and 8");
                continue;
            },
            Err(_) => {
                println!("Please input a number");
                continue
            },
        };

        return Game::new(num_players);
    }
}
