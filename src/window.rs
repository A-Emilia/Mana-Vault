use eframe::egui::{self, CentralPanel};
use super::Card;

pub fn create_window(){
    let options = eframe::NativeOptions::default();
    let _res = eframe::run_native(
    "Mana Vault",
    options,
    Box::new(|_a| Ok(Box::<Window>::default())),
    );
}
#[derive(Default)]
pub struct Window {
    card_counter: usize,
    cards: Vec<Card>,
    pending_card: Option<Card>,
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Welcome to Mana Vault");
            });
            ui.separator();
            if ui.button("test").clicked() {
                self.card_counter += 1;
                dbg!(self.cards.clone());
            }
            ui.label(format!("{}", self.card_counter));
            if ui.button("Add Card").clicked() {
                if self.pending_card.is_none() {
                    self.pending_card = Some(Card::default());
                }
            }
            for card in self.cards.iter() {
                ui.label(format!("Card: {}", card.name));
            }
        });
        if self.pending_card.is_some(){
            egui::Window::new("testest")
                .collapsible(false)
                .fade_in(true)
                .title_bar(true)
                .show(ctx, |ui| {
                    ui.label("Card name");
                    ui.text_edit_singleline(
                        //&mut self.pending_card.borrow_mut().as_mut().unwrap().name);
                        &mut self.pending_card_mut().name);
                    ui.horizontal(|ui|{
                        if ui.button("submit").clicked(){
                            if !self.pending_card_mut().name.is_empty() {
                                self.cards.push(self.pending_card.take().unwrap());
                            }
                        }
                        if ui.button("cancel").clicked(){
                            self.pending_card = None;
                        }
                    });
                });
        }
    }
}
impl Window{
    fn pending_card_mut(&mut self) -> &mut Card {
        self.pending_card.as_mut().unwrap()
    }
}