use std::fmt::{Debug, Formatter};
use std::collections::BTreeSet;
use crate::frequency::{score_mul_unique, score_sum_unique};

pub mod frequency;
pub mod words;

/// The possible colours of a cell
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum Filter {
    /// This letter is present, but not at this position
    Yellow { letter: u8, position: usize },
    /// This letter is present at this position
    Green { letter: u8, position: usize },
    /// This letter is absent
    Grey(u8),
}

impl Debug for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Filter::*;
        match self {
            Green { position, .. } => write!(f, "Green({}, {})", self.letter(), position),
            Yellow { position, .. } => write!(f, "Yellow({}, {})", self.letter(), position),
            Grey(_) => write!(f, "Grey({})", self.letter()),
        }
    }
}

impl Filter {
    pub fn accept(&self, input: &str) -> bool {
        use Filter::*;
        let input = input.as_bytes();
        match self {
            Green { letter, position } => input[*position] == *letter,
            Yellow { letter, position } => input[*position] != *letter && input.contains(letter),
            Grey(letter) => !input.contains(letter),
        }
    }

    pub fn letter(&self) -> char {
        use Filter::*;
        *match self {
            Grey(letter) => letter,
            Yellow { letter, .. } => letter,
            Green { letter, .. } => letter
        } as char
    }

    pub fn cycle(&self, position: usize) -> Filter {
        match *self {
            Filter::Grey(letter) => Filter::Yellow { position, letter },
            Filter::Yellow { letter, .. } => Filter::Green { position, letter },
            Filter::Green { letter, .. } => Filter::Grey(letter),
        }
    }
}

#[derive(Clone)]
pub struct Scoring<'a> {
    pub name: String,
    pub func: &'a dyn Fn(&str) -> f64,
}

impl Default for Scoring<'_> {
    fn default() -> Self {
        Scoring {
            name: "Sum freq'cies of unique letters".to_string(),
            func: &score_sum_unique
        }
    }
}

impl PartialEq<Self> for Scoring<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Scoring<'_> {}

pub struct Game<'a> {
    pub words: BTreeSet<&'static str>,
    pub filters: BTreeSet<Filter>,
    pub score: Scoring<'a>,
}

impl Default for Game<'_> {
    fn default() -> Self {
        Game {
            words: words::all().filter(|w| w.len() == 5).collect(),
            filters: BTreeSet::new(),
            score: Scoring::default(),
        }
    }
}

impl<'a> Game<'a> {
    pub fn with_scoring(s: Scoring<'a>) -> Self {
        Game {
            score: s,
            ..Default::default()
        }
    }
}

impl Game<'_> {
    pub fn suggest_word(&self) -> Option<&str> {
        self.words.iter()
            .filter(|word| self.filters.iter().all(|filter| filter.accept(word)))
            .map(|word| (word, (*self.score.func)(word)))
            .max_by_key(|(_, score)| (score * 1000.0) as u64)
            .map(|(word, _)| *word)
    }

    pub fn add_filters(&mut self, filters: impl IntoIterator<Item=Filter>) {
        let new_filters = filters.into_iter()
            .filter(|filter| self.is_compatible(&filter))
            .collect::<Vec<_>>();
        new_filters.into_iter().for_each(|filter| {
            self.filters.insert(filter);
        });
    }

    fn is_compatible(&self, new_filter: &Filter) -> bool {
        self.filters.iter().all(|current_filter|{
            match (current_filter, new_filter) {
                (Filter::Green {letter: letter1, ..}, Filter::Grey(letter2)) |
                (Filter::Yellow {letter: letter1, ..}, Filter::Grey(letter2))
                    if letter1 == letter2 => false,
                 _ => true,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        assert!(!Filter::Grey(b'A').accept("BAB"));
        assert!(Filter::Grey(b'A').accept("BBB"));

        assert!(Filter::Green { letter: b'A', position: 1 }.accept("BAB"));
        assert!(!Filter::Green { letter: b'A', position: 0 }.accept("BAB"));

        assert!(Filter::Yellow { letter: b'A', position: 0 }.accept("BAB"));
        assert!(!Filter::Yellow { letter: b'A', position: 1 }.accept("BAB"));
        assert!(!Filter::Yellow { letter: b'A', position: 1 }.accept("BBB"));
    }

    #[test]
    fn test_game() {
        let mut game = Game::default();
        use Filter::*;

        game.add_filters(vec![
            Yellow { letter: b'A', position: 0 },
            Yellow { letter: b'R', position: 1 },
            Grey(b'E'),
            Grey(b'T'),
        ]);

        let word = game.suggest_word().unwrap();
        assert_eq!(word, "ROANS");
    }
}
