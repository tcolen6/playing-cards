use crate::{Card, Suit};
use std::fmt;

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Hand { cards }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = String::new();
        for suit in Suit::iterator() {
            res = res + &format!("{} |", suit);
            for card in self.cards.iter().filter(|x| x.suit == *suit) {
                res = res + &format!(" {}", card);
            }
            res += "\n";
        }
        write!(f, "{}", res)
    }
}
