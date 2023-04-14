use rand::{seq::SliceRandom, thread_rng};
use std::fmt::Display;

static mut counter: i32 = 0;

#[derive(Clone, Copy)]
pub struct Card {
    value: Option<u8>,
    cardTag: Option<u8>,
}

impl Card {
    // constructor/destructor
    // TODO investigate whether i need option type
    fn new_empty() -> Self {
        Self {
            cardTag: None,
            value: None,
        }
    }
    fn new_full(tag: u8) -> Self {
        let value = match tag {
            2..=10 => Some(tag),
            1 => Some(11),
            11..=13 => Some(10),
            _ => None,
        };

        Self {
            cardTag: Some(tag),
            value,
        }
    }
    pub fn isAce(&self) -> bool {
        match self.value {
            Some(11) => true,
            _ => false,
        };
        false
    }
    pub fn getValue(&self) -> Option<u8> {
        self.value
    }
    pub fn getTag(&self) -> Option<u8> {
        self.cardTag
    }
    pub fn setTag(&mut self, tag: Option<u8>) {
        self.cardTag = tag;
    }
    // friend bool operator==
    // friend bool operator !=
    // Card operator=
    // Card operator []
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.cardTag {
            Some(1) => write!(f, "A"),
            Some(x) if x >= 2 && x <= 10 => write!(f, "{:?}", Some(self.cardTag)),
            Some(11) => write!(f, "J"),
            Some(12) => write!(f, "Q"),
            Some(13) => write!(f, "K"),
            _ => write!(f, "X"),
        }
    }
}

#[derive(Clone)]
pub struct Deck {
    cDeck: Vec<Card>,
    cardsLeft: u8,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            cardsLeft: 0,
            cDeck: Vec::new(),
        }
    }
    pub fn shuffle_deck(&mut self) {
        self.cDeck.shuffle(&mut thread_rng());
    }
    pub fn fill_red(&mut self) {
        for i in 1..14 {
            let in_card = Card::new_full(i);
            self.cDeck.push(in_card);
            self.cDeck.push(in_card);
        }
        self.cardsLeft = 26;
        self.shuffle_deck();
    }
    pub fn fill_black(&mut self) {
        for i in 1..14 {
            let in_card = Card::new_full(i);
            self.cDeck.push(in_card);
            if i != 13 {
                self.cDeck.push(in_card);
            }
        }
        self.cardsLeft = 25;
        self.shuffle_deck();
    }
    // TODO fix
    pub fn get_card(&mut self) -> Option<Card> {
        if self.cardsLeft == 0 {
            println!("No cards left\n");
            return None;
        }
        let card = self.cDeck[0];
        let length = self.cDeck.len();
        self.cDeck.swap(0, length - 1);
        self.cDeck.pop();
        self.cardsLeft -= 1;
        return Some(card);
    }
    pub fn deck_size(&self) -> u8 {
        self.cardsLeft
    }
    pub fn count_deck(&self) -> u8 {
        let mut count: u8 = 0;
        for card in &self.cDeck {
            if card.getTag() != Some(1) {
                count += card.getValue().unwrap_or_default();
            }
        }
        count
    }
    pub fn push_front(&mut self, card: Card) {
        self.cDeck.insert(0, card)
    }
    // fn card_swap(&mut self);
    pub fn clear_deck(&mut self) {
        self.cardsLeft = 0;
        self.cDeck.clear();
    }
}
