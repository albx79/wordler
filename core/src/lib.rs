use std::fmt::{Debug, Formatter};
use crate::frequency::{SumUnique, MulUnique};
use std::collections::btree_set::BTreeSet;
use itertools::Itertools;

pub mod frequency;

pub struct Wordler {
    pub attempts: Vec<Word>,
    pub suggestion: Word,
    pub filters: BTreeSet<Cell>,
    pub dictionary: Vec<String>,
    pub frequencies: frequency::FreqTable,
    pub scoring_functions: Vec<Box<dyn Score>>,
    pub scoring_idx: usize,
}

impl Default for Wordler {
    fn default() -> Self {
        let dictionary = frequency::english_words();
        let frequencies = frequency::frequencies(dictionary.iter().map(|s| s.as_str()));
        let scoring_functions: Vec<Box<dyn Score>> = vec![
            Box::new(SumUnique(frequencies.clone())),
            Box::new(MulUnique(frequencies.clone())),
        ];
        let suggestion = suggest_word(&dictionary, &BTreeSet::new(), scoring_functions[0].as_ref())
            .map(Word::new)
            .unwrap();
        Wordler {
            suggestion,
            dictionary,
            frequencies,
            scoring_functions,
            attempts: vec![],
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

    fn add_filters(&mut self, filters: impl IntoIterator<Item=Cell>) {
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
        self.suggestion = self.attempts.remove(self.attempts.len() - 1);
        self.filters = BTreeSet::new();
        let old_attempts = self.attempts.clone();
        old_attempts.iter().for_each(|word| self.add_filters(word.0.clone()));
    }

    pub fn not_a_word(&mut self) {
        if self.attempts.is_empty() {
            return;
        }
        let ceci_nest_pas_un_mot = self.suggestion.clone();
        self.undo();

        let idx_of_not_word = self.dictionary.iter().enumerate()
            .find(|(_, w)| w == &&ceci_nest_pas_un_mot.to_string())
            .map(|(i, _)| i)
            .unwrap();
        self.dictionary.swap_remove(idx_of_not_word);
        self.next();
    }

    /// Tries to guess another word, and returns `false` if it can't.
    pub fn next(&mut self) -> bool {
        self.add_filters(self.suggestion.0.clone());
        let new_suggestion = self.suggest_word().map(Word::new);
        match new_suggestion {
            Some(word)  => {
                self.attempts.push(self.suggestion.clone());
                self.suggestion = self.suggestion.next(word);
                return true;
            }
            _ => return false,
        }
    }

    pub fn reset(&mut self) {
        self.attempts = vec![];
        self.filters = BTreeSet::new();
        self.suggestion = Word::new(self.suggest_word().unwrap());
    }

    pub(crate) fn suggest_word(&self) -> Option<&str> {
        suggest_word(&self.dictionary, &self.filters, self.score())
    }

    fn score(&self) -> &dyn Score {
        self.scoring_functions[self.scoring_idx].as_ref()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
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

    pub fn next(&self, mut letters: Word) -> Self {
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

    pub fn byte(&self) -> u8 {
        use Cell::*;
        *match self {
            Grey(letter) => letter,
            Yellow { letter, .. } => letter,
            Green { letter, .. } => letter
        }
    }

    pub fn letter(&self) -> char {
        self.byte() as char
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
