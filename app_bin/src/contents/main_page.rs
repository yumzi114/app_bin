use eframe::egui::{Color32, InnerResponse, RichText, ScrollArea, Ui};

use crate::RasApp;







pub fn main_content_view(app:&mut RasApp, ui: &mut Ui)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.heading(RichText::new("THIS MAIN").size(45.).color(Color32::from_rgb(72, 245, 66)).underline());
        ui.label("WHYYY");
        ScrollArea::vertical()
        .auto_shrink([false; 2])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for i in &app.test_list{
                ui.label(i);
                
            }
        });
        
    })
}