use std::fmt;

static mut counter : i32 = 0;

#[derive(Clone, Copy)]
struct Card {
    value : u8,
    cardTag : u8,
}

impl Card {
    // constructor/destructor
    fn new() -> Card;
    pub fn isAce(&self) -> bool;
    pub fn getValue(&self) -> u8;
    pub fn getTag(&self) -> u8;
    pub fn setTag(&mut self);
    // friend bool operator==
    // friend bool operator !=
    // Card operator=
    // Card operator []
    // friend operator <<    
}

#[derive(Clone)]
struct Deck {
    cDeck : Vec<Card>,
    cardsLeft : u8,
}

impl Deck{
    fn new() -> Deck;
    fn shuffle_deck(&mut self);
    fn fill_red(&mut self);
    fn fill_black(&mut self);
    fn get_card(&self) -> Card; // gpt suggests mut self
    fn deck_size(&self) -> u8;
    fn count_deck(&self) -> u8;
    fn push_front(&mut self, card: Card);
    fn card_swap(&mut self);
    fn clear_deck(&mut self);
}

