use wordler_core::{Filter, Game};
use eframe::{egui, epi};
use eframe::egui::{CtxRef, RichText, Ui};
use eframe::epi::Frame;

struct Wordler {
    attempts: Vec<Word>,
    game: Game,
}

impl Default for Wordler {
    fn default() -> Self {
        let game = Game::default();
        let first_word = game.suggest_word().unwrap();
        Wordler {
            attempts: vec![Word::new(first_word)],
            game,
        }
    }
}

impl Wordler {
    fn undo(&mut self) {
        if self.attempts.len() < 2 {
            return;
        }
        self.attempts.remove(self.attempts.len() - 1);
        self.game = Game::default();
        self.attempts.iter().for_each(|word| self.game.add_filters(word.letters.clone()))
    }

    fn next(&mut self) {
        let last_word = self.attempts.last().unwrap();
        self.game.add_filters(last_word.letters.clone());
        match self.game.suggest_word() {
            Some(word) => {
                let next_word = last_word.next(word);
                self.attempts.push(next_word);
            }
            _ => (),
        }
    }
}

struct Word {
    letters: Vec<Filter>,
}

impl Word {
    fn new(input: &str) -> Self {
        let letters = input.bytes().map(|b| Filter::Grey(b)).collect();
        Word { letters }
    }

    fn next(&self, input: &str) -> Self {
        let mut letters = Word::new(input);
        for (i, letter) in letters.enumerate() {
            let letter_in_prev_word = self.letters[i];
            match letter_in_prev_word {
                Filter::Green { .. } => *letter = letter_in_prev_word,
                _ => (),
            }
        }
        letters
    }

    fn enumerate(&mut self) -> impl Iterator<Item=(usize, &mut Filter)> {
        self.letters.iter_mut().enumerate()
    }

    fn ui(&mut self, ui: &mut Ui) {
        for (position, filter) in self.letters.iter_mut().enumerate() {
            let button = egui::Button::new(RichText::new(
                String::from(filter.letter()))
                .color(egui::Color32::WHITE)
                .monospace()
                .heading()
            )
                .fill(filter_color(filter));
            if ui.add(button).clicked() {
                *filter = filter.cycle(position);
            }
        }
    }
}

fn filter_color(filter: &Filter) -> egui::Color32 {
    match filter {
        Filter::Yellow { .. } => egui::Color32::from_rgb(0xc9, 0xb4, 0x58),
        Filter::Green { .. } => egui::Color32::from_rgb(0x6a, 0xaa, 0x64),
        Filter::Grey(_) => egui::Color32::from_rgb(0x78, 0x7c, 0x7e),
    }
}

impl epi::App for Wordler {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("main grid")
                .min_col_width(14.0)
                .max_col_width(14.0)
                .show(ui, |ui| {
                    let num_attempts = self.attempts.len();
                    let mut next_word = false;
                    let mut undo_word = false;
                    for (row, word) in self.attempts.iter_mut().enumerate() {
                        let is_last_row = row == num_attempts - 1;
                        ui.add_enabled_ui(is_last_row, |ui| {
                            word.ui(ui);
                            next_word = ui.button("Next").clicked();
                            undo_word = is_last_row && row != 0 && ui.button("Undo").clicked();
                        });
                        ui.end_row();
                    }
                    if next_word {
                        self.next();
                    } else if undo_word {
                        self.undo();
                    }

                    ui.separator();
                    ui.end_row();
                    if ui.button("Reset").clicked() {
                        let new_wordler = Wordler::default();
                        *self = new_wordler;
                    }
                    ui.end_row();
                });
        });

        frame.set_window_size(ctx.used_size());
        frame.set_window_title("Wordler");
    }

    fn name(&self) -> &str {
        "Wordler"
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(Wordler::default()), options);
}