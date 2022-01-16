static RAW_DATA: &str = include_str!("words.txt");

pub fn all<'a>() -> impl Iterator<Item=&'a str> {
    RAW_DATA.split('\n')
}