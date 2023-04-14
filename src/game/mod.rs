use crate::card::*;

enum TieType {
    Tie,
    Win,
    Lose
}

enum CompareResult {
    AceWipe,
    KingTie,
    Invalid,
    Tie,
    Win
}

pub struct Game {
    adStats : bool,
    gameOver : bool,
    winner : bool,
    redDeck : Deck,
    blackDeck : Deck,
    selectedRed : Vec<u8>,
    selectedBlack : Vec<u8>,
    redInPlay : Vec<Option<Card>>,
    redNumIP : u8,
    blackInPlay : Vec<Option<Card>>,
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

impl Game {
    pub fn new() -> Game{
        let mut red_deck = Deck::new();
        let mut black_deck = Deck::new();
        red_deck.fill_red();
        black_deck.fill_black();
        let black_count = black_deck.count_deck();
        let red_count = red_deck.count_deck();
        let mut game = Game {
            redDeck : red_deck,
            blackDeck : black_deck,
            pa : false,
            ca : 0,
            score : 0,
            adStats : false,
            redNumIP : 0,
            blackNumIP : 0,
            validMove : false,
            initFlag : true, 
            gameOver : false,
            fEnc : false,
            blackCount : (black_count + 32),
            redCount : red_count,
            selectedBlack : Vec::new(),
            selectedRed : Vec::new(),
            redInPlay : Vec::new(),
            blackInPlay : Vec::new(),
            winner : false,
            na : 0   
        };
        game.draw();
        game

    }
    pub fn draw(&mut self){
        if (self.redDeck.deck_size() != 0){
            for i in 0..3{
                if self.redInPlay[i].is_none(){
                    self.redInPlay[i] = self.redDeck.get_card();
                }
            }
        }

        if (self.blackDeck.deck_size() != 0){
            for i in 0..3{
                if self.blackInPlay[i].is_none(){
                    self.blackInPlay[i] = self.blackDeck.get_card();
                }
            }
        }
    }
    
    pub fn runGame(&mut self){
        loop {
            self.validMove = false;
            while !self.validMove && !self.gameOver {
                if self.adStats {
                    // self.display_ad_stats
                }
                // self.display_cards();
                // self.turn();
                self.draw();
            }
            if self.gameOver {
                break;
            }
        }
        // self.end_game();
        // self.play_again();
    }

    // TODO FIX EXPECT LOGIC
    // TODO FINISH THIS FUNCTION after compare
    pub fn Tie(&mut self) -> TieType{
        if self.blackDeck.deck_size() == 0 {
            return TieType::Win;
        }

        let red = match self.redDeck.get_card(){
            Some(card) => card,
            None => {
                self.redCount
            }
        };
        let black = self.blackDeck.get_card();

        if !red.expect("No cards left").isAce(){
            self.redCount -= red.expect("invalid card").getValue();
        }

        TieType::Tie
    }

    // TODO FIX
    pub fn redTotal(&self) -> u8{
        let mut sum : u8 = 0;
        for i in &self.selectedRed {
            if let Some(card) = &self.redInPlay[*i] {
                sum += card.getValue().unwrap_or(0);
            }
        }
        sum
    }

    pub fn compare(&self) -> CompareResult{
        if self.redInPlay[0].unwrap().isAce(){
            println!("ACE WIPE\n");
            return CompareResult::AceWipe;
        }
        if (self.redInPlay[0].unwrap().getTag() == Some(13) && self.blackInPlay[0].unwrap().getTag() == Some(13)) {
             return CompareResult::KingTie;
        }

        if (self.r)

        return CompareResult::Tie
    }
}