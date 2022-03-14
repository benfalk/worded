use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Debug;
use std::fs::read_to_string;

use crate::assets::Asset;

pub struct WordBank {
    data: String,
    words: BTreeSet<&'static str>,
}

impl Debug for WordBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WordBank(Storage: {}, Words: {}: BestGuess: {})",
            self.data.capacity(),
            self.words.len(),
            self.best_guess(),
        )
    }
}

impl WordBank {
    pub fn from_string(data: String) -> Self {
        let mut words = BTreeSet::new();
        let (ptr, len, cap) = data.into_raw_parts();

        let data = unsafe {
            let slice = std::slice::from_raw_parts(ptr, len);
            let pool: &'static str = std::str::from_utf8_unchecked(slice);
            'outer: for word in pool.lines() {
                let mut count = 0;
                for char in word.chars() {
                    if !(char.is_ascii_alphanumeric() && char.is_ascii_lowercase()) {
                        continue 'outer;
                    }
                    count += 1;
                    if count == 6 {
                        continue 'outer;
                    }
                }
                if count == 5 {
                    words.insert(word);
                }
            }
            String::from_raw_parts(ptr, len, cap)
        };

        Self { data, words }
    }

    pub fn from_std_dict() -> Self {
        let data = read_to_string("/usr/share/dict/words").expect("words dictionary");
        Self::from_string(data)
    }

    pub fn from_asset_words() -> Self {
        let file = Asset::get("words.txt").unwrap();
        let data = String::from_utf8(file.data.to_owned().into_owned()).unwrap();
        Self::from_string(data)
    }

    pub fn all_valid_words(&self) -> &BTreeSet<&'static str> {
        &self.words
    }

    pub fn most_used_chars(&self) -> Vec<(char, usize)> {
        let mut chars = HashMap::with_capacity(26);
        let mut charset = HashSet::new();

        for word in &self.words {
            charset.clear();
            for c in word.chars() {
                if !charset.contains(&c) {
                    *chars.entry(c).or_insert(0 as usize) += 1;
                }
                charset.insert(c);
            }
        }

        let mut data: Vec<(char, usize)> = chars.into_iter().collect();

        data.sort_by(|a, b| a.1.cmp(&b.1));
        data
    }

    pub fn best_guess(&self) -> &str {
        let vowels = ['a', 'e', 'i', 'o', 's'];
        let mut vowel_tracker = BTreeSet::new();
        let mut most_vowels = 0;
        let mut guess = "";

        for word in self.words.iter() {
            vowel_tracker.clear();
            word.chars().filter(|c| vowels.contains(c)).for_each(|c| {
                vowel_tracker.insert(c);
            });
            if vowel_tracker.len() > most_vowels {
                most_vowels = vowel_tracker.len();
                guess = word;
            }
        }

        guess
    }

    pub fn best_guesses(&self) -> Vec<&str> {
        let most_used: Vec<char> = self
            .most_used_chars()
            .iter()
            .rev()
            .map(|d| d.0)
            .take(5)
            .collect();
        let mut vowel_tracker = BTreeSet::new();
        let mut guesses = vec![];

        for word in self.words.iter() {
            vowel_tracker.clear();
            word.chars().filter(|c| most_used.contains(c)).for_each(|c| {
                vowel_tracker.insert(c);
            });
            if vowel_tracker.len() == 5 {
                guesses.push(*word);
            }
        }

        guesses
    }
}
