use crate::word_bank::WordBank;
use std::collections::HashSet;

pub struct Game {
    wordbank: WordBank,
    exclussions: HashSet<char>,
    chars_exact: HashSet<(usize, char)>,
    chars_shift: HashSet<(usize, char)>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            wordbank: WordBank::from_asset_words(),
            exclussions: HashSet::with_capacity(26),
            chars_exact: HashSet::with_capacity(26),
            chars_shift: HashSet::with_capacity(26),
        }
    }

    pub fn reset(&mut self) {
        self.exclussions.clear();
        self.chars_shift.clear();
        self.chars_exact.clear();
    }

    pub fn add_exclussion(&mut self, exclussion: char) {
        self.exclussions.insert(exclussion);
    }

    pub fn char_exact(&mut self, loc: usize, char: char) {
        self.chars_exact.insert((loc, char));
    }

    pub fn char_shift(&mut self, loc: usize, char: char) {
        self.chars_shift.insert((loc, char));
    }

    pub fn best_guess(&self) -> &str {
        self.wordbank.best_guess()
    }

    pub fn words(&self) -> Vec<&str> {
        let mut words = vec![];

        'nextword: for word in self.wordbank.all_valid_words() {
            if word.chars().any(|c| self.exclussions.contains(&c)) {
                continue 'nextword;
            }

            for (pos, c) in &self.chars_exact {
                if word.chars().nth(*pos).unwrap() != *c {
                    continue 'nextword;
                }
            }

            for (pos, c) in &self.chars_shift {
                if word.chars().nth(*pos).unwrap() == *c {
                    continue 'nextword;
                }

                if !word.chars().any(|cc| cc == *c) {
                    continue 'nextword;
                }
            }

            words.push(*word)
        }

        words
    }
}
