use wordler_core::{Cell, Wordler, Word};
use eframe::{egui, epi};
use eframe::egui::{CtxRef, RichText, Ui};
use eframe::epi::Frame;

struct Gui(Wordler);

fn word_ui(word: &mut Word, ui: &mut Ui) {
    for (position, filter) in word.enumerate() {
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

fn filter_color(filter: &Cell) -> egui::Color32 {
    match filter {
        Cell::Yellow { .. } => egui::Color32::from_rgb(0xc9, 0xb4, 0x58),
        Cell::Green { .. } => egui::Color32::from_rgb(0x6a, 0xaa, 0x64),
        Cell::Grey(_) => egui::Color32::from_rgb(0x78, 0x7c, 0x7e),
    }
}

impl epi::App for Gui {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        let w = &mut self.0;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui|{
                ui.add_enabled_ui(w.attempts.len() == 1, |ui| {
                    let update_scoring_fn = egui::ComboBox::from_label("Scoring function")
                        .selected_text(w.scoring_functions[w.scoring_idx].name())
                        .show_index(ui, &mut w.scoring_idx, w.scoring_functions.len(), |i| {
                            w.scoring_functions[i].name().to_string()
                        }).changed();
                    if update_scoring_fn {
                        w.reset();
                    };
                });
                ui.separator();

                egui::Grid::new("main grid")
                    .show(ui, |ui| {
                        let num_attempts = w.attempts.len();
                        let mut next_word = false;
                        let mut undo_word = false;
                        let mut not_a_word = false;
                        for (row, word) in w.attempts.iter_mut().enumerate() {
                            let is_last_row = row == num_attempts - 1;
                            let show_more_btns = is_last_row && row != 0;
                            ui.add_enabled_ui(is_last_row, |ui| {
                                word_ui(word, ui);
                                next_word = ui.button("Next").clicked();
                                undo_word =  show_more_btns && ui.button("Undo").clicked();
                                not_a_word = show_more_btns && ui.button("Not a word").clicked();
                            });
                            ui.end_row();
                        }
                        if next_word {
                            w.next();
                        } else if undo_word {
                            w.undo();
                        } else if not_a_word {
                            w.not_a_word();
                        }
                    });
                ui.separator();
                if ui.button("Reset").clicked() {
                    w.reset();
                }
            });
        });

        frame.set_window_title("Wordler");
    }

    fn name(&self) -> &str {
        "Wordler"
    }
}

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.resizable = false;
    options.initial_window_size = Some([400.0, 350.0].into());
    eframe::run_native(Box::new(Gui(Wordler::default())), options);
}