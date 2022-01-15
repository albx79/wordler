use std::collections::BTreeMap;

mod words;
mod frequency;


fn main() {
    println!("Hello, world!");
    let word_map: BTreeMap<&str, f64> = words::all()
        .map(|word| (*word, frequency::score(word)))
        .collect();

}
