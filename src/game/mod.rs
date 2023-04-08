use crate::card::*;

pub struct Game {
    adStats : bool,
    gameOver : bool,
    winner : bool,
    redDeck : Deck,
    blackDeck : Deck,
    selectedRed : Vec<u8>,
    selectedBlack : Vec<u8>,
    redInPlay : Vec<Card>,
    redNumIP : u8,
    blackInPlay : Vec<Card>,
    blackNumIP : u8,
    ca : u8,
    blackCount : u8,
    redCount : u8,
    na : u8,
    score : u8,
    validMove : bool,
    initFlag : bool,
    fEnc : bool,
    pub pa : bool,
}

// impl Game {
    
// }