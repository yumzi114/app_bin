use eframe::egui::{InnerResponse, Ui};

use crate::RasApp;







pub fn rf_content_view(app:&mut RasApp, ui: &mut Ui)->InnerResponse<()>{
    ui.horizontal_centered(|ui|{
        ui.heading("THIS RF");
    })
}