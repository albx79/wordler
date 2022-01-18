use std::fmt::{Debug, Formatter};
use std::collections::BTreeSet;

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

pub struct Game {
    words: BTreeSet<&'static str>,
    filters: BTreeSet<Filter>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            words: words::all().filter(|w| w.len() == 5).collect(),
            filters: BTreeSet::new(),
        }
    }
}

impl Game {
    pub fn suggest_word(&self) -> Option<&str> {
        self.words.iter()
            .filter(|word| self.filters.iter().all(|filter| filter.accept(word)))
            .map(|word| (word, frequency::score_sum_unique(word)))
            .max_by(|(_, score1), (_, score2)| score1.partial_cmp(score2).unwrap())
            .map(|(word, _)| *word)
    }

    pub fn add_filters(&mut self, filters: impl IntoIterator<Item=Filter>) {
        filters.into_iter().for_each(|filter| {
            self.filters.insert(filter);
        });
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
