use rand::{seq::SliceRandom, thread_rng};
use std::fmt::Display;

// static mut counter: i32 = 0;

#[derive(Clone, Copy)]
pub struct Card {
    value: u8,
    card_tag: u8,
}

impl Card {
    // constructor/destructor
    // // TODO investigate whether i need option type
    pub fn new_empty() -> Self {
        Self {
            card_tag: 0,
            value: 0,
        }
    }
    pub fn new_full(tag: u8) -> Self {
        let value = match tag {
            2..=10 => tag,
            1 => 11,
            11..=13 => 10,
            _ => 0,
        };

        Self {
            card_tag: tag,
            value,
        }
    }
    pub fn is_ace(&self) -> bool {
        match self.value {
            11 => true,
            _ => false,
        };
        false
    }
    pub fn get_value(&self) -> u8 {
        self.value
    }
    pub fn get_tag(&self) -> u8 {
        self.card_tag
    }
    // pub fn set_tag(&mut self, tag: u8) {
    //     self.card_tag = tag;
    // }
    // friend bool operator==
    // friend bool operator !=
    // Card operator=
    // Card operator []
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.card_tag {
            1 => write!(f, "A"),
            x if x >= 2 && x <= 10 => write!(f, "{:?}", Some(self.card_tag)),
            11 => write!(f, "J"),
            12 => write!(f, "Q"),
            13 => write!(f, "K"),
            _ => write!(f, "X"),
        }
    }
}

#[derive(Clone)]
pub struct Deck {
    c_deck: Vec<Card>,
    cards_left: u8,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            cards_left: 0,
            c_deck: Vec::new(),
        }
    }
    pub fn shuffle_deck(&mut self) {
        self.c_deck.shuffle(&mut thread_rng());
    }
    pub fn fill_red(&mut self) {
        for i in 1..14 {
            let in_card = Card::new_full(i);
            self.c_deck.push(in_card);
            self.c_deck.push(in_card);
        }
        self.cards_left = 26;
        self.shuffle_deck();
    }
    pub fn fill_black(&mut self) {
        for i in 1..14 {
            let in_card = Card::new_full(i);
            self.c_deck.push(in_card);
            if i != 13 {
                self.c_deck.push(in_card);
            }
        }
        self.cards_left = 25;
        self.shuffle_deck();
    }
    // TODO fix
    pub fn get_card(&mut self) -> Option<Card> {
        if self.cards_left == 0 {
            println!("No cards left\n");
            return None;
        }
        let card = self.c_deck[0];
        let length = self.c_deck.len();
        self.c_deck.swap(0, length - 1);
        self.c_deck.pop();
        self.cards_left -= 1;
        return Some(card);
    }
    pub fn deck_size(&self) -> u8 {
        self.cards_left
    }
    pub fn count_deck(&self) -> u8 {
        let mut count: u8 = 0;
        for card in &self.c_deck {
            if card.get_tag() != 1 {
                count += card.get_value();
            }
        }
        count
    }
    pub fn push_front(&mut self, card: Card) {
        self.c_deck.insert(0, card)
    }
    // fn card_swap(&mut self);
    pub fn clear_deck(&mut self) {
        self.cards_left = 0;
        self.c_deck.clear();
    }
}
