static FREQUENCIES: [(char, f64); 26] = [
    ('A', 8.34),
    ('B', 1.54),
    ('C', 2.73),
    ('D', 4.14),
    ('E', 12.60),
    ('F', 2.03),
    ('G', 1.92),
    ('H', 6.11),
    ('I', 6.71),
    ('J', 0.23),
    ('K', 0.87),
    ('L', 4.24),
    ('M', 2.53),
    ('N', 6.80),
    ('O', 7.70),
    ('P', 1.66),
    ('Q', 0.09),
    ('R', 5.68),
    ('S', 6.11),
    ('T', 9.37),
    ('U', 2.85),
    ('V', 1.06),
    ('W', 2.34),
    ('X', 0.20),
    ('Y', 2.04),
    ('Z', 0.06),
];

fn of(letter: char) -> f64 {
    FREQUENCIES.iter().find(|x| x.0 == letter).unwrap().1
}

pub fn score(word: &str) -> f64 {
    use itertools::Itertools;
    word.chars()
        .unique()
        .map(of)
        .fold(0.0, |a, b| a + b)
}
