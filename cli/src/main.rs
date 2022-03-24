use wordler_core::{Cell, Wordler, Word};
use std::fmt::{Display, Formatter};

fn main() {
    let mut game = Wordler::default();

    loop {
        for word in game.attempts.iter() {
            println!("    {word}", word = Color(word));
        }
        println!("Try {word}", word = Color(&game.suggestion));
        match read_user_input(&mut game) {
            Err(_) => {
                println!("Bye!");
                return;
            }
            Ok(_) => if !game.next() {
                println!("I'm out of ideas");
                return;
            }
        }
    }
}

fn read_user_input(game: &mut Wordler) -> Result<(), ()> {
    loop {
        println!(
            r#"
Enter the game response using:
"G" or "g" for a Green letter (you got it in the right position)
"Y" or "y" for a Yellow letter (you got it, but in the wrong position)
"." for a grey letter (you didn't get it)
"N" for "not a word"."#
        );
        let response = match promptly::prompt::<String, _>("Response") {
            Ok(r) => r,
            _ => {
                return Err(());
            }
        };

        let response = response.to_ascii_uppercase();
        if &response == "N" {
            game.not_a_word();
            return Ok(())
        } else {
            match mutate_cells(&response, &mut game.suggestion.0) {
                Ok(()) => return Ok(()),
                _ => (),
            }
        }
    }
}

pub fn mutate_cells(response: &str, cells: &mut [Cell]) -> Result<(), ()> {
    for (position, (user_input, cell)) in response.bytes().zip(cells.iter_mut()).enumerate() {
        *cell = match user_input {
            b'.' => Cell::Grey(cell.byte()),
            b'Y' => Cell::Yellow { letter: cell.byte(), position },
            b'G' => Cell::Green { letter: cell.byte(), position },
            _ => {
                println!("Invalid response");
                return Err(());
            }
        }
    }
    return Ok(());
}

struct Color<'a>(&'a Word);

impl Display for Color<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        for cell in self.0.0.iter() {
            let letter = cell.letter().to_string();
            write!(f, "{}", match cell {
                Cell::Yellow { .. } => letter.bright_yellow().on_black(),
                Cell::Green { .. } => letter.bright_green().on_black(),
                Cell::Grey(_) => letter.white().on_bright_black(),
            })?;
        }
        std::fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_filter() {
        let mut cells = vec![
            Cell::Yellow {
                letter: b'W',
                position: 0,
            },
            Cell::Green {
                letter: b'O',
                position: 1,
            },
            Cell::Grey(b'T'),
        ];

        mutate_cells("g.y", &mut cells).unwrap();

        assert_eq!(cells, vec![
            Cell::Green {
                letter: b'W',
                position: 0,
            },
            Cell::Grey(b'O'),
            Cell::Yellow{ letter: b'T', position: 2},
        ]);
    }

    #[test]
    #[allow(non_snake_case)]
    fn guess_a_word() {
        let mut game = Wordler::default();

        for resp in [
            "y.y.g",
            ".y.yg",
        ] {
            mutate_cells(resp, &mut game.suggestion.0).unwrap();
            game.next();
        }

        assert_eq!(game.suggestion.to_string(), Word::new("OVATE").to_string());
    }
}
