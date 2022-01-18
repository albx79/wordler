use wordler_core::{Filter, Game};

fn main() {
    let mut game = Game::default();

    loop {
        let word = match game.suggest_word() {
            Some(w) => w,
            None => {
                println!("Sorry, I'm out of ideas.");
                return;
            }
        };
        println!("Try {}", word);
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
                    println!("Bye!");
                    return;
                }
            };
            match str_to_filter(&response, word) {
                Some(filters) => {
                    game.add_filters(filters);
                    break;
                }
                None => println!("Invalid response"),
            }
        }
    }
}

fn str_to_filter(input: &str, word: &str) -> Option<Vec<Filter>> {
    let mut filters = vec![];
    for (position, code) in input.to_ascii_uppercase().as_bytes().iter().enumerate() {
        let letter = word.as_bytes().get(position).map(|l| *l);
        let filter = match (*code, letter) {
            (b'.', Some(letter)) => Filter::Grey(letter),
            (b'Y', Some(letter)) => Filter::Yellow { letter, position },
            (b'G', Some(letter)) => Filter::Green { letter, position },
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
        assert_eq!(str_to_filter(".", "WORD").unwrap()[0], Filter::Grey(b'W'));
        assert_eq!(
            str_to_filter("yG", "WORD").unwrap(),
            vec![
                Filter::Yellow {
                    letter: b'W',
                    position: 0
                },
                Filter::Green {
                    letter: b'O',
                    position: 1
                }
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn guess_the_word_PROXY() {
        let mut game = Game::default();

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
