use crate::card::*;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

#[derive(PartialEq)]
pub enum TieType {
    Tie,
    Win,
    Lose,
    AceWipe,
}

#[derive(PartialEq)]
pub enum CompareResult {
    AceWipe,
    KingTie,
    Invalid,
    Tie,
    Win,
}

pub struct Game {
    ad_stats: bool,
    game_over: bool,
    winner: bool,
    red_deck: Deck,
    black_deck: Deck,
    selected_red: Vec<u8>,
    selected_black: Vec<u8>,
    red_in_play: Vec<Option<Card>>,
    red_num_ip: u8,
    black_in_play: Vec<Option<Card>>,
    black_num_ip: u8,
    ca: u8,
    black_count: u8,
    red_count: u8,
    na: u8,
    score: u8,
    valid_move: bool,
    // init: bool,
    f_enc: bool,
    pub pa: bool,
}

impl Game {
    // TODO FIX EXPECT LOGIC
    // TODO FINISH THIS FUNCTION after compare
    pub fn tie(&mut self) -> TieType {
        if self.black_deck.deck_size() == 0 {
            return TieType::Win;
        }

        let red = self.red_deck.get_card().unwrap();
        let black = self.black_deck.get_card().unwrap();

        if !red.is_ace() {
            self.red_count -= red.get_value();
        }
        self.black_count -= black.get_value();

        println!("\nTie: Red {} vs Black {}", red, black);

        if red.is_ace() {
            return TieType::AceWipe;
        }
        if red.get_value() == black.get_value() {
            if red.get_tag() > black.get_tag() {
                return TieType::Win;
            } else if red.get_tag() < black.get_tag() {
                self.black_deck.push_front(black);
                return TieType::Lose;
            } else {
                return TieType::Tie;
            }
        } else if red.get_value() < black.get_value() {
            self.black_deck.push_front(black);
            return TieType::Lose;
        } else {
            return TieType::Win;
        }

        // TieType::Tie
    }
    pub fn ace_wipe(&mut self) {
        for i in 0..3 {
            self.black_count -= self.black_in_play[i].unwrap().get_value();
            self.black_in_play[i] = None;
        }
    }

    pub fn black_total(&self) -> u8 {
        let mut sum: u8 = 0;
        for i in &self.selected_black {
            if let Some(card) = self.black_in_play.get(*i as usize) {
                if let Some(value) = card.as_ref().map(|c| c.get_value()) {
                    sum += value;
                }
            }
        }
        sum
    }

    pub fn black_win(&self) -> bool {
        if self.red_deck.deck_size() > 0 {
            return false;
        }
        let mut rsum = 0;
        let mut bsum = 0;
        let mut rtag = 0;
        let mut btag = 0;
        let mut aceflag = false;

        for i in 0..3 {
            if self.red_in_play[i].unwrap().is_ace() {
                aceflag = true;
                break;
            }
            bsum += self.black_in_play[i].unwrap().get_value();
            btag += self.black_in_play[i].unwrap().get_tag();
            rsum += self.red_in_play[i].unwrap().get_value();
            rtag += self.red_in_play[i].unwrap().get_tag();
        }
        if aceflag {
            return false;
        } else if bsum > rsum {
            return true;
        } else if rsum > bsum {
            return false;
        } else {
            if btag > rtag {
                return true;
            } else {
                return false;
            }
        }
    }

    pub fn calculate_score(&mut self) {
        if self.red_count != 0 {
            self.score += self.red_deck.count_deck();
        }
        for i in 0..3 {
            if self.red_in_play[i].is_some() && !self.red_in_play[i].unwrap().is_ace() {
                self.score += self.red_in_play[i].unwrap_or(Card::new_empty()).get_value()
            }
        }
    }

    pub fn check_win(&mut self) {
        if self.black_in_play[0].is_none()
            && self.black_in_play[1].is_none()
            && self.black_in_play[2].is_none()
            && self.black_deck.deck_size() == 0
        {
            self.winner = true;
            self.game_over = true;
            return;
        } else if self.black_win() {
            self.winner = false;
            self.game_over = true;
            return;
        } else {
            return;
        }
    }

    pub fn compare(&mut self) -> CompareResult {
        if self.red_in_play[self.selected_red[0] as usize].unwrap().is_ace() {
            println!("ACE WIPE\n");
            return CompareResult::AceWipe;
        }
        if self.red_in_play[self.selected_red[0] as usize].unwrap().get_tag() == 13
            && self.black_in_play[self.selected_red[0] as usize].unwrap().get_tag() == 13
        {
            return CompareResult::KingTie;
        }

        if self.red_total() < self.black_total() {
            return CompareResult::Invalid;
        }

        if self.red_total() == self.black_total() {
            if self.red_deck.deck_size() == 0 {
                // cannot tie with no cards left
                return CompareResult::Invalid;
            }
            let mut r_tag = 0;
            let mut b_tag = 0;

            for i in &self.selected_red {
                if let Some(card) = self.red_in_play.get(*i as usize) {
                    if let Some(tag) = card.as_ref().map(|c| c.get_tag()) {
                        if tag > r_tag {
                            r_tag = tag;
                        }
                    }
                }
            }

            for i in &self.selected_black {
                if let Some(card) = self.black_in_play.get(*i as usize) {
                    if let Some(tag) = card.as_ref().map(|c| c.get_tag()) {
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
        } else if self.red_total() > self.black_total() {
            self.valid_move = true;
            return CompareResult::Win;
        } else {
            return CompareResult::Invalid;
        }

        return CompareResult::Invalid;
    }

    pub fn welcome_screen(&mut self) {
        let mut p = false;
        let mut input = String::new();
        println!(
            "Welcome to Climb! A card game created by Mark Rizko.\n
                Officially rewritten in Rust!\nPress 1 for the rules\t
                Press 2 to toggle advanced stats (unimplemented)\tPress 3 to play!\n"
        );

        while !p {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim() {
                "1" => self.display_rules(),
                "2" => {
                    self.ad_stats = !self.ad_stats;
                    if self.ad_stats {
                        println!("Advanced stats on\n");
                    } else {
                        println!("Advanced stats off\n");
                    }
                }
                "3" => {
                    p = true;
                }
                _ => println!("Invalid input! Try again\n"),
            }
        }
    }

    pub fn display_cards(&self) {
        if !self.f_enc {
            println!(
                "\t\tK\t\t\tBlack Cards Left: {}\n",
                self.black_deck.deck_size() + 1
            );
            println!("Black: \n");
            println!(
                "\t{}\t{}\t{}\n",
                self.black_in_play[0].unwrap_or(Card::new_empty()),
                self.black_in_play[1].unwrap_or(Card::new_empty()),
                self.black_in_play[2].unwrap_or(Card::new_empty())
            );
            println!("Red: \n");
            println!(
                "\t{}\t{}\t{}\n",
                self.red_in_play[0].unwrap_or(Card::new_empty()),
                self.red_in_play[1].unwrap_or(Card::new_empty()),
                self.red_in_play[2].unwrap_or(Card::new_empty())
            );
            println!("\nRed Cards Left: {}\n", self.red_deck.deck_size())
        } else {
            println!("\t\tK\n\n");
            println!("Red: \n");
            println!(
                "\t{}\t{}\t{}\n",
                self.red_in_play[0].unwrap_or(Card::new_empty()),
                self.red_in_play[1].unwrap_or(Card::new_empty()),
                self.red_in_play[2].unwrap_or(Card::new_empty())
            );
            println!("\nRed Cards Left: {}\n", self.red_deck.deck_size());
        }
    }

    pub fn display_rules(&self) {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = Path::new(&manifest_dir).join("rules.txt");
        let contents = fs::read_to_string(file_path).expect("Cannot read rules.txt");
        println!("{contents}");
    }

    pub fn draw(&mut self) {
        if self.red_deck.deck_size() != 0 {
            for i in 0..3 {
                if self.red_in_play[i].is_none() {
                    self.red_in_play[i] = self.red_deck.get_card();
                }
            }
        }

        if self.black_deck.deck_size() != 0 {
            for i in 0..3 {
                if self.black_in_play[i].is_none() {
                    self.black_in_play[i] = self.black_deck.get_card();
                }
            }
        }
    }

    pub fn end_game(&mut self) {
        if self.winner {
            if self.final_encounter() {
                println!("You win!\n");
                self.calculate_score();
                println!("\nScore: {}\n", self.score);
            } else {
                println!("You lose!\n");
            }
        } else {
            println!("You lose!\n");
        }
    }

    pub fn final_encounter(&mut self) -> bool {
        self.f_enc = true;
        let king = Card::new_full(13);
        println!("\n\n\t\tTime to face the King!\t\t\n\n");
        self.black_in_play[1] = Some(king);
        self.draw();
        self.display_cards();

        if self.black_win() {
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
                for i in &self.selected_red {
                    if let Some(mut _card) = self.red_in_play.get(*i as usize) {
                        _card = &None;
                    }
                }
                return true;
            }
            _ => return false,
        }
    }

    pub fn new() -> Game {
        let mut red_deck = Deck::new();
        let mut black_deck = Deck::new();
        red_deck.fill_red();
        black_deck.fill_black();
        let black_count = black_deck.count_deck();
        let red_count = red_deck.count_deck();
        let mut game = Game {
            red_deck,
            black_deck,
            pa: false,
            ca: 0,
            score: 0,
            ad_stats: false,
            red_num_ip: 0,
            black_num_ip: 0,
            valid_move: false,
            // init: true,
            game_over: false,
            f_enc: false,
            black_count: (black_count + 32),
            red_count,
            selected_black: Vec::new(),
            selected_red: Vec::new(),
            red_in_play: vec![None; 3],
            black_in_play: vec![None; 3],
            winner: false,
            na: 0,
        };
        game.draw();
        game
    }

    pub fn play_again(&mut self) {
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
                _ => println!("Invalid input, please try again.."),
            }
        }
    }

    pub fn red_total(&self) -> u8 {
        let mut sum: u8 = 0;
        for i in &self.selected_red {
            if let Some(card) = self.red_in_play.get(*i as usize) {
                if let Some(value) = card.as_ref().map(|c| c.get_value()) {
                    sum += value;
                }
            }
        }
        sum
    }

    // TODO adstats
    pub fn run_game(&mut self) {
        loop {
            self.valid_move = false;
            while !self.valid_move && !self.game_over {
                // if self.adStats {
                //     // self.display_ad_stats
                // }
                self.display_cards();
                self.turn();
                self.draw();
            }
            if self.game_over {
                break;
            }
        }
        self.end_game();
        self.play_again();
    }

    pub fn select(&mut self) {
        let mut input = String::new();
        println!("\nSelect Red: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input.chars().for_each(|c| match c {
            '1' => {
                self.selected_red.push(0);
                self.red_num_ip += 1;
            }
            '2' => {
                self.selected_red.push(1);
                self.red_num_ip += 1;
            }
            '3' => {
                self.selected_red.push(2);
                self.red_num_ip += 1;
            }
            'x' => {
                self.black_deck.clear_deck();
            }
            _ => {}
        });

        input.clear();
        // flag = true;
        if !self.f_enc {
            println!("Select Black: ");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            input.chars().for_each(|c| match c {
                '1' => {
                    self.selected_black.push(0);
                    self.black_num_ip += 1;
                }
                '2' => {
                    self.selected_black.push(1);
                    self.black_num_ip += 1;
                }
                '3' => {
                    self.selected_black.push(2);
                    self.black_num_ip += 1;
                }
                'x' => {
                    self.red_deck.clear_deck();
                }
                _ => {}
            })
        } else {
            self.selected_black.push(1);
            self.black_num_ip += 1;
        }
    }

    pub fn turn(&mut self) {
        self.select();
        let res = self.compare();
        // let mut tres = TieType::Tie;
        match res {
            CompareResult::Tie => loop {
                let tres = self.tie();
                if tres != TieType::Tie {
                    if tres == TieType::AceWipe {
                        self.ace_wipe();
                    }
                    break;
                }
            },
            CompareResult::Invalid => {
                self.selected_red.clear();
                self.selected_black.clear();
                println!("********INVALID MOVE********\n");
                return;
            }
            CompareResult::AceWipe => self.ace_wipe(),
            _ => {}
        };

        for i in &self.selected_red {
            if let Some(mut _card) = self.red_in_play.get(*i as usize) {
                if let Some(tag) = _card.as_ref().map(|c| c.get_tag()) {
                    if tag != 1 {
                        self.red_count -= _card.unwrap().get_value();
                    }
                }
                self.red_in_play[*i as usize] = None;
            }
        }

        for i in &self.selected_black {
            if let Some(mut _card) = self.black_in_play.get(*i as usize) {
                self.black_count -= _card.unwrap().get_value();
                self.black_in_play[*i as usize] = None;
            }
        }

        self.selected_red.clear();
        self.selected_black.clear();
        self.check_win();
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::card;

//     use super::*;

//     #[test]
//     fn test_totals() {}
// }
