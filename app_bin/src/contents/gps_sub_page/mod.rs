use eframe::egui::{self, Color32, InnerResponse, RichText, Ui};

use crate::{components::layout::sub_menu_window, RasApp};





pub fn gps_sub_raw_view(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.label(RichText::new("GPS RAW VIEW").size(35.).color(Color32::from_rgb(232, 58, 58)).underline());
        ui.label(&app.menu_ctl.test_gps_str);
    })
}