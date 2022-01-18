use wordler_core::{Filter, Game, Scoring};
use eframe::{egui, epi};
use eframe::egui::{CtxRef, RichText, Ui};
use eframe::epi::Frame;
use wordler_core::frequency::{score_sum_unique, score_mul_unique};

struct Wordler {
    attempts: Vec<Word>,
    game: Game<'static>,
    scoring_functions: Vec<Scoring<'static>>,
    scoring_idx: usize,
}

impl Default for Wordler {
    fn default() -> Self {
        let game = Game::default();
        let first_word = game.suggest_word().unwrap();
        let sum_freqs = Scoring::default();
        let mul_freqs = Scoring {
            name: "Multiply freq'cies of unique letters".to_string(),
            func: &score_mul_unique,
        };
        Wordler {
            attempts: vec![Word::new(first_word)],
            game,
            scoring_functions: vec![sum_freqs, mul_freqs],
            scoring_idx: 0,
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

    fn reset(&mut self) {
        self.game = Game::with_scoring(self.scoring_functions[self.scoring_idx].clone());
        self.attempts = vec![self.game.suggest_word().map(Word::new).unwrap()];
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
            if ui.add_sized([24.0, 24.0], button).clicked() {
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
            ui.vertical(|ui|{
                ui.add_enabled_ui(self.attempts.len() == 1, |ui| {
                    let update_scoring_fn = egui::ComboBox::from_label("Scoring function")
                        .selected_text(&self.scoring_functions[self.scoring_idx].name)
                        .show_index(ui, &mut self.scoring_idx, self.scoring_functions.len(), |i| {
                            self.scoring_functions[i].name.clone()
                        }).changed();
                    if update_scoring_fn {
                        self.reset();
                    };
                });
                ui.separator();

                egui::Grid::new("main grid")
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
                    });
                ui.separator();
                if ui.button("Reset").clicked() {
                    self.reset();
                }
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