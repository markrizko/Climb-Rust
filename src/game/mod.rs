use crate::card::{self, *};
use std::io;
use std::env;
use std::path::Path;

#[derive(PartialEq)]
enum TieType {
    Tie,
    Win,
    Lose,
    AceWipe,
}

#[derive(PartialEq)]
enum CompareResult {
    AceWipe,
    KingTie,
    Invalid,
    Tie,
    Win,
}

pub struct Game {
    adStats: bool,
    gameOver: bool,
    winner: bool,
    redDeck: Deck,
    blackDeck: Deck,
    selectedRed: Vec<u8>,
    selectedBlack: Vec<u8>,
    redInPlay: Vec<Option<Card>>,
    redNumIP: u8,
    blackInPlay: Vec<Option<Card>>,
    blackNumIP: u8,
    ca: u8,
    blackCount: u8,
    redCount: u8,
    na: u8,
    score: u8,
    validMove: bool,
    initFlag: bool,
    fEnc: bool,
    pub pa: bool,
}

impl Game {
    pub fn new() -> Game {
        let mut red_deck = Deck::new();
        let mut black_deck = Deck::new();
        red_deck.fill_red();
        black_deck.fill_black();
        let black_count = black_deck.count_deck();
        let red_count = red_deck.count_deck();
        let mut game = Game {
            redDeck: red_deck,
            blackDeck: black_deck,
            pa: false,
            ca: 0,
            score: 0,
            adStats: false,
            redNumIP: 0,
            blackNumIP: 0,
            validMove: false,
            initFlag: true,
            gameOver: false,
            fEnc: false,
            blackCount: (black_count + 32),
            redCount: red_count,
            selectedBlack: Vec::new(),
            selectedRed: Vec::new(),
            redInPlay: Vec::new(),
            blackInPlay: Vec::new(),
            winner: false,
            na: 0,
        };
        game.draw();
        game
    }
    pub fn draw(&mut self) {
        if (self.redDeck.deck_size() != 0) {
            for i in 0..3 {
                if self.redInPlay[i].is_none() {
                    self.redInPlay[i] = self.redDeck.get_card();
                }
            }
        }

        if (self.blackDeck.deck_size() != 0) {
            for i in 0..3 {
                if self.blackInPlay[i].is_none() {
                    self.blackInPlay[i] = self.blackDeck.get_card();
                }
            }
        }
    }

    // TODO adstats
    pub fn runGame(&mut self) {
        loop {
            self.validMove = false;
            while !self.validMove && !self.gameOver {
                // if self.adStats {
                //     // self.display_ad_stats
                // }
                self.display_cards();
                self.turn();
                self.draw();
            }
            if self.gameOver {
                break;
            }
        }
        self.end_game();
        self.play_again();
    }

    pub fn play_again(&mut self){
        let mut input = String::new();
        let mut flag = false;
        while !flag {
            println!("\n\nDo you want to play again? (y/n)");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match input.to_lowercase().trim() {
                "y" => {
                    self.pa = true;
                    flag = true;
                }
                "n" => {
                    self.pa = false;
                    flag = true;
                }
                _ => println!("Invalid input, please try again..")
            }
        }
    }

    pub fn display_rules(){
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = Path::new(&manifest_dir).join("rules.txt");
    }

    pub fn final_encounter(&mut self) -> bool{
        self.fEnc = true;
        let king = Card::new_full(13);
        println!("\n\n\t\tTime to face the King!\t\t\n\n");
        self.blackInPlay[1] = Some(king);
        self.draw();
        self.display_cards();
        
        if self.blackWin() {
            return false;
        }
        self.select();

        let res = self.compare();
        match res {
            CompareResult::KingTie | CompareResult::AceWipe | CompareResult::Win => {
                if res == CompareResult::KingTie {
                    println!("\nKing Victory!\t\t+5 points\n");
                    self.score += 5;
                }
                for i in &self.selectedRed {
                    if let Some(mut card) = self.redInPlay.get(*i as usize) {
                        card = &None;
                    }
                }
                return true;
            }
            _ => return false,
        }
    }

    pub fn end_game(&mut self) {
        if self.winner {
            if self.final_encounter() {
                println!("You win!\n");
                self.calculateScore();
                println!("\nScore: {}\n", self.score);
            }
            else {
                println!("You lose!\n");
            }
        }
        else {
            println!("You lose!\n");
        }
    }

    pub fn calculateScore(&mut self) {
        if self.redCount != 0 {
            self.score += self.redDeck.count_deck();
        }
        for i in 0..3 {
            if self.redInPlay[i].is_some() && !self.redInPlay[i].unwrap().isAce() {
                self.score += self.redInPlay[i].unwrap_or(Card::new_empty()).getValue()
            }
        }
    }

    // TODO FIX EXPECT LOGIC
    // TODO FINISH THIS FUNCTION after compare
    pub fn Tie(&mut self) -> TieType {
        if self.blackDeck.deck_size() == 0 {
            return TieType::Win;
        }

        let red = self.redDeck.get_card().unwrap();
        let black = self.blackDeck.get_card().unwrap();

        if !red.isAce() {
            self.redCount -= red.getValue();
        }
        self.blackCount -= black.getValue();

        println!("\nTie: Red {} vs Black {}", red, black);

        if red.isAce() {
            return TieType::AceWipe;
        }
        if red.getValue() == black.getValue() {
            if red.getTag() > black.getTag() {
                return TieType::Win;
            } else if red.getTag() < black.getTag() {
                self.blackDeck.push_front(black);
                return TieType::Lose;
            } else {
                return TieType::Tie;
            }
        } else if red.getValue() < black.getValue() {
            self.blackDeck.push_front(black);
            return TieType::Lose;
        } else {
            return TieType::Win;
        }

        // TieType::Tie
    }

    pub fn select(&mut self) {
        let mut input = String::new();
        let mut flag = true;
        println!("\nSelect Red: ");
        while flag {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim() {
                "1" => {
                    self.selectedRed.push(0);
                    self.redNumIP += 1;
                }
                "2" => {
                    self.selectedRed.push(1);
                    self.redNumIP += 1;
                }
                "3" => {
                    self.selectedRed.push(2);
                    self.redNumIP += 1;
                }
                "." => {
                    flag = false;
                }
                "x" => {
                    self.blackDeck.clear_deck();
                }
                _ => {
                    println!("Invalid input")
                }
            }
            input.clear();
        }
        flag = true;
        if !self.fEnc {
            println!("Select Black: ");
            while flag {
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                match input.trim() {
                    "1" => {
                        self.selectedBlack.push(0);
                        self.blackNumIP += 1;
                    }
                    "2" => {
                        self.selectedBlack.push(1);
                        self.blackNumIP += 1;
                    }
                    "3" => {
                        self.selectedBlack.push(2);
                        self.blackNumIP += 1;
                    }
                    "." => {
                        flag = false;
                    }
                    "x" => {
                        self.redDeck.clear_deck();
                    }
                    _ => {
                        println!("Invalid input");
                    }
                }
            }
        } else {
            self.selectedBlack.push(1);
            self.blackNumIP += 1;
        }
    }

    pub fn ace_wipe(&mut self) {
        for i in 0..3 {
            self.blackCount -= self.blackInPlay[i].unwrap().getValue();
            self.blackInPlay[i] = None;
        }
    }

    pub fn display_cards(&self){
        if !self.fEnc {
            println!("\t\tK\t\t\tBlack Cards Left: {}\n", self.blackCount + 1);
            println!("Black: \n");
            println!("\t{}\t{}\t{}\n", self.blackInPlay[0].unwrap_or(Card::new_empty()), self.blackInPlay[1].unwrap_or(Card::new_empty()), self.blackInPlay[2].unwrap_or(Card::new_empty()));
            println!("Red: \n");
            println!("\t{}\t{}\t{}\n", self.redInPlay[0].unwrap_or(Card::new_empty()), self.redInPlay[1].unwrap_or(Card::new_empty()), self.redInPlay[2].unwrap_or(Card::new_empty()));
            println!("\nRed Cards Left: {}\n", self.redCount)
        }
        else {
            println!("\t\tK\n\n");
            println!("Red: \n");
            println!("\t{}\t{}\t{}\n", self.redInPlay[0].unwrap_or(Card::new_empty()), self.redInPlay[1].unwrap_or(Card::new_empty()), self.redInPlay[2].unwrap_or(Card::new_empty()));
            println!("\nRed Cards Left: {}\n", self.redCount);
        }
    }

    pub fn turn(&mut self) {
        self.select();
        let res = self.compare();
        let mut tres = TieType::Tie;
        match res {
            CompareResult::Tie => loop {
                tres = self.Tie();
                if tres != TieType::Tie {
                    if tres == TieType::AceWipe {
                        self.ace_wipe();
                    }
                    break;
                }
            },
            CompareResult::Invalid => {
                self.selectedRed.clear();
                self.selectedBlack.clear();
                println!("********INVALID MOVE********\n");
                return;
            }
            CompareResult::AceWipe => self.ace_wipe(),
            _ => {}
        };

        for i in &self.selectedRed {
            if let Some(mut card) = self.redInPlay.get(*i as usize) {
                if let Some(tag) = card.as_ref().map(|c| c.getTag()) {
                    if tag != 1 {
                        self.redCount -= card.unwrap().getValue();
                    }
                }
                card = &None;
            }
        }

        for i in &self.selectedBlack {
            if let Some(mut card) = self.blackInPlay.get(*i as usize) {
                self.blackCount -= card.unwrap().getValue();
                card = &None;
            }
        }

        self.selectedRed.clear();
        self.selectedBlack.clear();
        self.checkWin();
    }

    pub fn checkWin(&mut self) {
        if self.blackInPlay[0].is_none() && 
            self.blackInPlay[1].is_none() &&
            self.blackInPlay[2].is_none() &&
            self.blackDeck.deck_size() == 0{
                self.winner = true;
                self.gameOver = true;
                return;
        }
        else if self.blackWin(){
            self.winner = false;
            self.gameOver = true;
            return;
        }
        else {
            return;
        }
    }

    pub fn blackWin(&self) -> bool{
        if self.redDeck.deck_size() > 0 {
            return false;
        }
        let mut rsum = 0;
        let mut bsum = 0;
        let mut rtag = 0;
        let mut btag = 0;
        let mut aceflag = false;

        for i in 0..3 {
            if self.redInPlay[i].unwrap().isAce(){
                aceflag = true;
                break;
            }
            bsum += self.blackInPlay[i].unwrap().getValue();
            btag += self.blackInPlay[i].unwrap().getTag();
            rsum += self.redInPlay[i].unwrap().getValue();
            rtag += self.redInPlay[i].unwrap().getTag();
        }
        if aceflag {
            return false;
        }
        else if bsum > rsum {
            return true;
        }
        else if rsum > bsum {
            return false;
        }
        else {
            if btag > rtag {
                return true;
            }
            else {
                return false;
            }
        }
    }

    pub fn redTotal(&self) -> u8 {
        let mut sum: u8 = 0;
        for i in &self.selectedRed {
            if let Some(card) = self.redInPlay.get(*i as usize) {
                if let Some(value) = card.as_ref().map(|c| c.getValue()) {
                    sum += value;
                }
            }
        }
        sum
    }

    pub fn blackTotal(&self) -> u8 {
        let mut sum: u8 = 0;
        for i in &self.selectedBlack {
            if let Some(card) = self.blackInPlay.get(*i as usize) {
                if let Some(value) = card.as_ref().map(|c| c.getValue()) {
                    sum += value;
                }
            }
        }
        sum
    }

    pub fn compare(&mut self) -> CompareResult {
        if self.redInPlay[0].unwrap().isAce() {
            println!("ACE WIPE\n");
            return CompareResult::AceWipe;
        }
        if self.redInPlay[0].unwrap().getTag() == 13 && self.blackInPlay[0].unwrap().getTag() == 13
        {
            return CompareResult::KingTie;
        }

        if self.redTotal() < self.blackTotal() {
            return CompareResult::Invalid;
        }

        if self.redTotal() == self.blackTotal() {
            if self.redDeck.deck_size() == 0 {
                // cannot tie with no cards left
                return CompareResult::Invalid;
            }
            let mut r_tag = 0;
            let mut b_tag = 0;

            for i in &self.selectedRed {
                if let Some(card) = self.redInPlay.get(*i as usize) {
                    if let Some(tag) = card.as_ref().map(|c| c.getTag()) {
                        if tag > r_tag {
                            r_tag = tag;
                        }
                    }
                }
            }

            for i in &self.selectedBlack {
                if let Some(card) = self.blackInPlay.get(*i as usize) {
                    if let Some(tag) = card.as_ref().map(|c| c.getTag()) {
                        if tag > b_tag {
                            b_tag = tag;
                        }
                    }
                }
            }

            if r_tag > 10 || b_tag > 10 {
                if r_tag == b_tag {
                    return CompareResult::Tie;
                } else if r_tag > b_tag {
                    return CompareResult::Win;
                } else {
                    return CompareResult::Invalid;
                }
            }
        } else if self.redTotal() > self.blackTotal() {
            self.validMove = true;
            return CompareResult::Win;
        } else {
            return CompareResult::Invalid;
        }

        return CompareResult::Invalid;
    }

}

#[cfg(test)]
mod tests {
    use crate::card;

    use super::*;

    #[test]
    fn testTotals() {}
}
