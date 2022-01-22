use itertools::Itertools;
use std::collections::btree_map::BTreeMap;
use crate::Score;

static ENGLISH_WORDS: &str = include_str!("words.txt");

pub fn english_words() -> Vec<String> {
    ENGLISH_WORDS.split('\n').map(|s| s.to_owned()).collect()
}

pub type FreqTable = BTreeMap<u8, f64>;

#[derive(Clone)]
pub struct SumUnique(pub FreqTable);

#[derive(Clone)]
pub struct MulUnique(pub FreqTable);

impl Score for SumUnique {
    fn name(&self) -> &str {
        "Sum freq'cies of unique letters"
    }

    fn score(&self, word: &str) -> f64 {
        word.bytes()
            .unique()
            .map(|c| self.0[&c])
            .sum()
    }

    fn duplicate(&self) -> Box<dyn Score> {
        Box::new(self.clone())
    }
}

impl Score for MulUnique {
    fn name(&self) -> &str {
        "Multiply freq'cies of unique letters"
    }

    fn score(&self, word: &str) -> f64 {
        word.bytes()
            .unique()
            .map(|c| self.0[&c])
            .map(|freq| freq / 100.0 + 1.0)
            .fold(1.0, |a, b| a * b)
    }

    fn duplicate(&self) -> Box<dyn Score> {
        Box::new(self.clone())
    }
}

pub fn frequencies<'a>(words: impl Iterator<Item=&'a str>) -> BTreeMap<u8, f64> {
    let mut count: BTreeMap<u8, u32> = BTreeMap::new();
    let mut total = 0f64;
    for letter in words.flat_map(|w| w.bytes()) {
        *count.entry(letter).or_insert(0) += 1;
        total += 1.0;
    }
    count.iter()
        .map(|(&letter, &occurrencies)| (letter, occurrencies as f64 / total * 100.0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! dict {
        () => {
            {
                BTreeMap::new()
            }
        };
        ( $( $key:expr => $val:expr),* ) => {
            {
                let mut temp_set = BTreeMap::new();
                $(
                    temp_set.insert($key, $val); // Insert each item matched into the HashSet
                )*
                temp_set // Return the populated HashSet
            }
        };
    }

    #[test]
    fn test_frequencies() {
        assert_eq!(
            frequencies(["AA", "BB"].into_iter()),
            dict!(b'A' => 50.0, b'B' => 50.0)
        );

        assert_eq!(
            frequencies(["A", "BBB"].into_iter()),
            dict!(b'A' => 25.0, b'B' => 75.0)
        );
    }

    #[test]
    fn test_score() {
        let freq_table = frequencies(english_words().iter().map(|s| s.as_str()));

        let sum_unique = SumUnique(freq_table.clone());
        assert!(sum_unique.score("PROXY") > 10.0);
        assert!(sum_unique.score("SINCE") > sum_unique.score("WINCE"));

        let mul_unique = MulUnique(freq_table.clone());
        assert!(mul_unique.score("PROXY") > 1.0);
        assert!(mul_unique.score("SINCE") > mul_unique.score("WINCE"));
    }
}