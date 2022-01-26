use std::fmt::{Debug, Formatter};
use crate::frequency::{SumUnique, MulUnique};
use std::collections::btree_set::BTreeSet;
use itertools::Itertools;

pub mod frequency;

pub struct Wordler {
    pub attempts: Vec<Word>,
    pub filters: BTreeSet<Cell>,
    pub words: Vec<String>,
    pub frequencies: frequency::FreqTable,
    pub scoring_functions: Vec<Box<dyn Score>>,
    pub scoring_idx: usize,
}

impl Default for Wordler {
    fn default() -> Self {
        let words = frequency::english_words();
        let frequencies = frequency::frequencies(words.iter().map(|s| s.as_str()));
        let scoring_functions: Vec<Box<dyn Score>> = vec![
            Box::new(SumUnique(frequencies.clone())),
            Box::new(MulUnique(frequencies.clone())),
        ];
        let first_word = suggest_word(&words, &BTreeSet::new(), scoring_functions[0].as_ref())
            .map(Word::new)
            .unwrap();
        Wordler {
            words,
            frequencies,
            scoring_functions,
            attempts: vec![first_word],
            filters: BTreeSet::new(),
            scoring_idx: 0,
        }
    }
}

fn suggest_word<'a, 'b>(words: &'a[String], filters: &BTreeSet<Cell>, score: &dyn Score) -> Option<&'a str> {
    words.iter()
        .filter(|word| filters.iter().all(|filter| filter.accept(word)))
        .map(|word| (word, score.score(word)))
        .max_by_key(|(_, score)| (score * 1000.0) as u64)
        .map(|(word, _)| word.as_str())
}


impl Wordler {

    pub fn add_filters(&mut self, filters: impl IntoIterator<Item=Cell>) {
        let new_filters = filters.into_iter()
            .filter(|filter| self.is_compatible(&filter))
            .collect::<Vec<_>>();
        new_filters.into_iter().for_each(|filter| {
            self.filters.insert(filter);
        });
    }

    fn is_compatible(&self, new_filter: &Cell) -> bool {
        self.filters.iter().all(|current_filter|{
            match (current_filter, new_filter) {
                (Cell::Green {letter: letter1, ..}, Cell::Grey(letter2)) |
                (Cell::Yellow {letter: letter1, ..}, Cell::Grey(letter2))
                if letter1 == letter2 => false,
                _ => true,
            }
        })
    }

    pub fn undo(&mut self) {
        self.attempts.remove(self.attempts.len() - 1);
        self.filters = BTreeSet::new();
        let old_attempts = std::mem::take(&mut self.attempts);
        old_attempts.iter().rev().skip(1).for_each(|word| self.add_filters(word.0.clone()));
        self.attempts = old_attempts;
    }

    pub fn not_a_word(&mut self) {
        let ceci_nest_pas_un_mot = self.attempts[self.attempts.len() - 1].clone();
        self.undo();
        let idx_of_not_word = self.words.iter().enumerate()
            .find(|(_, w)| w == &&ceci_nest_pas_un_mot.to_string())
            .map(|(i, _)| i)
            .unwrap();
        self.words.swap_remove(idx_of_not_word);
    }

    pub fn next(&mut self) {
        let last_word = self.attempts.last().unwrap().clone();
        self.add_filters(last_word.0.clone());
        match self.suggest_word() {
            Some(word)  => {
                let next_word = last_word.next(word);
                self.attempts.push(next_word);
            }
            _ => (),
        }
    }

    pub fn reset(&mut self) {
        self.attempts = vec![];
        self.filters = BTreeSet::new();
        self.attempts.push(Word::new(self.suggest_word().unwrap()));
    }

    pub fn suggest_word(&self) -> Option<&str> {
        suggest_word(&self.words, &self.filters, self.score())
    }

    fn score(&self) -> &dyn Score {
        self.scoring_functions[self.scoring_idx].as_ref()
    }
}

#[derive(Clone)]
pub struct Word(pub Vec<Cell>);

impl std::string::ToString for Word {
    fn to_string(&self) -> String {
        self.0.iter().map(|c| c.letter()).join("")
    }
}

impl Word {
    pub fn new(input: &str) -> Self {
        let letters = input.bytes().map(|b| Cell::Grey(b)).collect();
        Word(letters)
    }

    pub fn next(&self, input: &str) -> Self {
        let mut letters = Word::new(input);
        for (i, letter) in letters.enumerate() {
            let letter_in_prev_word = self.0[i];
            match letter_in_prev_word {
                Cell::Green { .. } => *letter = letter_in_prev_word,
                _ => (),
            }
        }
        letters
    }

    pub fn enumerate(&mut self) -> impl Iterator<Item=(usize, &mut Cell)> {
        self.0.iter_mut().enumerate()
    }
}

/// The possible states of a cell
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum Cell {
    /// This letter is present, but not at this position
    Yellow { letter: u8, position: usize },
    /// This letter is present at this position
    Green { letter: u8, position: usize },
    /// This letter is absent
    Grey(u8),
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        match self {
            Green { position, .. } => write!(f, "Green({}, {})", self.letter(), position),
            Yellow { position, .. } => write!(f, "Yellow({}, {})", self.letter(), position),
            Grey(_) => write!(f, "Grey({})", self.letter()),
        }
    }
}

impl Cell {
    pub fn accept(&self, input: &str) -> bool {
        use Cell::*;
        let input = input.as_bytes();
        match self {
            Green { letter, position } => input[*position] == *letter,
            Yellow { letter, position } => input[*position] != *letter && input.contains(letter),
            Grey(letter) => !input.contains(letter),
        }
    }

    pub fn letter(&self) -> char {
        use Cell::*;
        *match self {
            Grey(letter) => letter,
            Yellow { letter, .. } => letter,
            Green { letter, .. } => letter
        } as char
    }

    pub fn cycle(&self, position: usize) -> Cell {
        match *self {
            Cell::Grey(letter) => Cell::Yellow { position, letter },
            Cell::Yellow { letter, .. } => Cell::Green { position, letter },
            Cell::Green { letter, .. } => Cell::Grey(letter),
        }
    }
}

pub trait Score {
    fn name(&self) -> &str;
    fn score(&self, word: &str) -> f64;
    fn duplicate(&self) -> Box<dyn Score>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        assert!(!Cell::Grey(b'A').accept("BAB"));
        assert!(Cell::Grey(b'A').accept("BBB"));

        assert!(Cell::Green { letter: b'A', position: 1 }.accept("BAB"));
        assert!(!Cell::Green { letter: b'A', position: 0 }.accept("BAB"));

        assert!(Cell::Yellow { letter: b'A', position: 0 }.accept("BAB"));
        assert!(!Cell::Yellow { letter: b'A', position: 1 }.accept("BAB"));
        assert!(!Cell::Yellow { letter: b'A', position: 1 }.accept("BBB"));
    }

    #[test]
    fn test_game() {
        let mut game = Wordler::default();
        use Cell::*;

        game.add_filters(vec![
            Yellow { letter: b'A', position: 0 },
            Yellow { letter: b'R', position: 1 },
            Grey(b'E'),
            Grey(b'T'),
        ]);

        let word = game.suggest_word().unwrap();
        assert!(word.contains("A"));
        assert!(word.contains("O"));
    }
}
