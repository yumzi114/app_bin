use eframe::egui::{self, Color32, InnerResponse, RichText, Ui};

use crate::{components::layout::sub_menu_window, contents::lte_sub_page::lte_sub_raw_view, App_Menu, RasApp};







pub fn lte_content_view(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    sub_menu_window(app,ui,ctx);
    
    match app.menu_ctl.state {
        App_Menu::LTE(0)=>{
            ui.vertical_centered(|ui|{
                ui.heading(RichText::new("THIS GPS_VIEW").size(45.).color(Color32::from_rgb(72, 245, 66)).underline());
            })
        }
        App_Menu::LTE(2)=>{
            lte_sub_raw_view(app,ui,ctx)
        }
        _=>{
            ui.vertical_centered(|ui|{
                // ui.heading(RichText::new("THIS GPS_VIEW").size(45.).color(Color32::from_rgb(72, 245, 66)).underline());
            })
        }
    }
}

