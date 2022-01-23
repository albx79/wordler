use wordler_core::{Cell, Wordler, Word};
use std::fmt::{Display, Formatter};

fn main() {
    let mut game = Wordler::default();

    loop {
        for (i, word) in game.attempts.iter().enumerate() {
            let pad = if i != game.attempts.len() - 1 {
                "    "
            } else {
                "Try "
            };
            println!("{pad}{word}", pad = pad, word = Color(word));
        }
        match read_user_input(&mut game) {
            Err(_) => {
                println!("Bye!");
                return;
            }
            Ok(filters) => {
                game.add_filters(filters);
                match game.suggest_word().map(Word::new) {
                    Some(word) => {
                        game.attempts.push(word);
                    },
                    None => {
                        println!("I'm out of ideas");
                        return;
                    }
                }
            },
        }
    }
}

fn read_user_input(game: &mut Wordler) -> Result<Vec<Cell>, ()> {
    loop {
        println!(
            r#"
Enter the game response using:
"G" or "g" for a Green letter (you got it in the right position)
"Y" or "y" for a Yellow letter (you got it, but in the wrong position)
"." for a grey letter (you didn't get it)"#
        );
        let response = match promptly::prompt::<String, _>("Response") {
            Ok(r) => r,
            _ => {
                return Err(());
            }
        };
        match str_to_filter(&response, &game.attempts.last().unwrap().to_string()) {
            Some(filters) => {
                return Ok(filters);
            }
            None => println!("Invalid response"),
        }
    }

}

struct Color<'a>(&'a Word);

impl Display for Color<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        for cell in self.0.0.iter() {
            let letter = cell.letter().to_string();
            write!(f, "{}", match cell {
                Cell::Yellow {..} => letter.bright_yellow().on_black(),
                Cell::Green {..} => letter.bright_green().on_black(),
                Cell::Grey(_) => letter.white().on_bright_black(),
            })?;
        }
        std::fmt::Result::Ok(())
    }
}

fn str_to_filter(input: &str, word: &str) -> Option<Vec<Cell>> {
    let mut filters = vec![];
    for (position, code) in input.to_ascii_uppercase().as_bytes().iter().enumerate() {
        let letter = word.as_bytes().get(position).map(|l| *l);
        let filter = match (*code, letter) {
            (b'.', Some(letter)) => Cell::Grey(letter),
            (b'Y', Some(letter)) => Cell::Yellow { letter, position },
            (b'G', Some(letter)) => Cell::Green { letter, position },
            _ => return None,
        };
        filters.push(filter);
    }
    Some(filters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_filter() {
        assert_eq!(str_to_filter("invalid response", "WORD"), None);
        assert_eq!(str_to_filter(".", "WORD").unwrap()[0], Cell::Grey(b'W'));
        assert_eq!(
            str_to_filter("yG", "WORD").unwrap(),
            vec![
                Cell::Yellow {
                    letter: b'W',
                    position: 0
                },
                Cell::Green {
                    letter: b'O',
                    position: 1
                }
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn guess_the_word_PROXY() {
        let mut game = Wordler::default();

        for (resp, word) in [
            ("..g..", "ATONE"),
            ("..g.y", "CHOIR"),
            ("..gy.", "SWORD"),
            (".gg.y", "GROUP"),
            ("ggg..", "PROMO"),
        ] {
            game.add_filters(str_to_filter(resp, word).unwrap());
        }

        assert_eq!(game.suggest_word(), Some("PROXY"));
    }
}
