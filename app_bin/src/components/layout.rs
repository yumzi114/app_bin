use eframe::egui::{self, RichText, Ui};

pub fn menu_button_layout(ui: &mut Ui) -> egui::Response {
    ui.add(egui::Button::new(RichText::new("Test").strong().size(15.)))
}
