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
    pub fn isAce() -> bool;
    pub fn getValue() -> u8;
    pub fn getTag() -> u8;
    pub fn setTag();
    // friend bool operator==
    // friend bool operator !=
    // Card operator=
    // Card operator []
    // friend operator <<    
}

struct Deck {
    cDeck : &'a Vec,
    cardsLeft : u8,
}

