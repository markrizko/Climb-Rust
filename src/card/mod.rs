use rand::{seq::SliceRandom, thread_rng};
use std::fmt::Display;

static mut counter: i32 = 0;

#[derive(Clone, Copy)]
pub struct Card {
    value: u8,
    cardTag: u8,
}

impl Card {
    // constructor/destructor
    // // TODO investigate whether i need option type
    pub fn new_empty() -> Self {
        Self {
            cardTag: 0,
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
            cardTag: tag,
            value,
        }
    }
    pub fn isAce(&self) -> bool {
        match self.value {
            11 => true,
            _ => false,
        };
        false
    }
    pub fn getValue(&self) -> u8 {
        self.value
    }
    pub fn getTag(&self) -> u8 {
        self.cardTag
    }
    pub fn setTag(&mut self, tag: u8) {
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
            1 => write!(f, "A"),
            x if x >= 2 && x <= 10 => write!(f, "{:?}", Some(self.cardTag)),
            11 => write!(f, "J"),
            12 => write!(f, "Q"),
            13 => write!(f, "K"),
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
            if card.getTag() != 1 {
                count += card.getValue();
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
