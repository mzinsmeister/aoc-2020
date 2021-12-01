use std::fs::read_to_string;
use std::collections::VecDeque;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let mut game = CombatGame::parse(&input_string);
    game.play();
    let winner_score = game.get_winner_score();
    println!("{}", winner_score);
    let mut recursive_game = RecursiveCombatGame::from(CombatGame::parse(&input_string));
    recursive_game.play();
    let recursive_winner_score = recursive_game.get_winner_score();
    println!("{}", recursive_winner_score);
}

struct CombatGame {
    player1_deck: VecDeque<u32>,
    player2_deck: VecDeque<u32>
}

impl CombatGame {
    fn parse(input: &str) -> CombatGame {
        let player1_deck = input.split("\n\nPlayer 2:\n")
            .next().unwrap()
            .split("\n")
            .skip(1)
            .map(|e| e.parse().unwrap())
            .collect();
        let player2_deck = input.split("\n\n").skip(1)
            .next().unwrap()
            .split("\n")
            .skip(1)
            .filter(|e| !e.is_empty())
            .map(|e| e.parse().unwrap())
            .collect();
        CombatGame { player1_deck, player2_deck }
    }

    fn play(&mut self) {
        while !self.player2_deck.is_empty() && !self.player1_deck.is_empty() {
            let player1_card = self.player1_deck.pop_front().unwrap();
            let player2_card = self.player2_deck.pop_front().unwrap();
            if player1_card > player2_card {
                self.player1_deck.push_back(player1_card);
                self.player1_deck.push_back(player2_card);
            } else {
                self.player2_deck.push_back(player2_card);
                self.player2_deck.push_back(player1_card);
            }
        }
    }

    fn get_winner_score(&self) -> u32 {
        let winner_deck = if self.player1_deck.len() > 0 {
            &self.player1_deck
        } else {
            &self.player2_deck
        };
        winner_deck.iter()
            .rev()
            .enumerate()
            .fold(0u32, |acc, (i, &e)| acc + (i as u32 + 1) * e)
    }
}

struct RecursiveCombatGame {
    player1_deck: VecDeque<u32>,
    player2_deck: VecDeque<u32>,
    deck_history: Vec<(VecDeque<u32>, VecDeque<u32>)>
}

impl RecursiveCombatGame {
    fn from(game: CombatGame) -> RecursiveCombatGame {
        RecursiveCombatGame{
            player1_deck: game.player1_deck,
            player2_deck: game.player2_deck,
            deck_history: Vec::new()
        }
    }

    fn play(&mut self) -> bool {
        while !self.player2_deck.is_empty() && !self.player1_deck.is_empty() {
            //println!("--- next round ----");
            if self.deck_history.contains(&(self.player1_deck.to_owned(), self.player2_deck.to_owned())) {
                //println!("player 1 wins (infinite loop)");
                return true;
            }
            self.deck_history.push((self.player1_deck.to_owned(), self.player2_deck.to_owned()));
            //println!("player1_deck: {:?}", self.player1_deck);
            //println!("player2_deck: {:?}", self.player2_deck);
            let player1_card = self.player1_deck.pop_front().unwrap();
            let player2_card = self.player2_deck.pop_front().unwrap();
            let mut player1_wins: Option<bool> = Option::None;
            if player1_card <= self.player1_deck.len() as u32
                && player2_card <= self.player2_deck.len() as u32 {
                //println!("playing subgame");
                let mut new_deck = self.copy_with_deck_sizes(player1_card as usize, player2_card as usize);
                player1_wins = Some(new_deck.play())
            }
            if player1_wins.is_none() {
                player1_wins = Some(player1_card > player2_card);
            }
            if player1_wins.unwrap() {
                self.player1_deck.push_back(player1_card);
                self.player1_deck.push_back(player2_card);
            } else {
                self.player2_deck.push_back(player2_card);
                self.player2_deck.push_back(player1_card);
            }
        }
        if self.player2_deck.is_empty() {
            //println!("player1 wins\n");
            true
        } else {
            //println!("player2 wins\n");
            false
        }
    }

    fn get_winner_score(&self) -> u32 {
        let winner_deck = if self.player1_deck.len() > 0 {
            &self.player1_deck
        } else {
            &self.player2_deck
        };
        winner_deck.iter()
            .rev()
            .enumerate()
            .fold(0u32, |acc, (i, &e)| acc + (i as u32 + 1) * e)
    }

    fn copy_with_deck_sizes(&self, player1_size: usize, player2_size: usize) -> RecursiveCombatGame {
        RecursiveCombatGame {
            player1_deck: self.player1_deck.iter().take(player1_size).map(|&e| e).collect(),
            player2_deck: self.player2_deck.iter().take(player2_size).map(|&e| e).collect(),
            deck_history: Vec::new()
        }
    }
}