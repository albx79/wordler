use wordler_core::{Filter, Game};
use eframe::{egui, epi};
use eframe::egui::{CtxRef, Stroke, RichText};
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

struct Word {
    letters: Vec<Filter>,
}

impl Word {
    fn new(input: &str) -> Self {
        let letters = input.bytes().map(|b| Filter::Grey(b)).collect();
        Word { letters }
    }

    fn enumerate(&mut self) -> impl Iterator<Item=(usize, &mut Filter)> {
        self.letters.iter_mut().enumerate()
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
            ui.vertical(|ui| {
                let num_attempts = self.attempts.len();
                let mut next_word = false;
                for (row, word) in self.attempts.iter_mut().enumerate() {
                    ui.add_enabled_ui(row == num_attempts - 1, |ui| ui.horizontal(|ui| {
                        for (position, filter) in word.enumerate() {
                            let button = egui::Button::new(RichText::new(String::from(filter.letter()))
                                .color(egui::Color32::WHITE)
                                .monospace()
                                .heading())
                                .fill(filter_color(filter));
                            if ui.add(button).clicked() {
                                *filter = filter.cycle(position);
                            }
                        }
                        if ui.button("Next").clicked() {
                            next_word = true;
                        }
                    }));
                }
                if next_word {
                    self.game.add_filters(self.attempts.last().unwrap().letters.clone());
                    match self.game.suggest_word() {
                        Some(word) => self.attempts.push(Word::new(word)),
                        _ => (),
                    }
                }
                if ui.button("Reset").clicked() {
                    let new_wordler = Wordler::default();
                    *self = new_wordler;
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