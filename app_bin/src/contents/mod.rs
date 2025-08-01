pub mod gps_page;
pub mod lte_page;
pub mod car_page;
pub mod rf_page;
pub mod main_page;




use std::sync::atomic::Ordering;

use chrono::Local;
use eframe::egui::{self, include_image, Align, Image, InnerResponse, Layout, RichText, Ui};
use crate::RasApp;

pub fn connect_bt(app:&mut RasApp, ui: &mut Ui)->InnerResponse<()>{
    ui.vertical_centered_justified(|ui|{
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            match app.gps_reader_task.is_running.load(Ordering::Relaxed) {
                true=>{
                    if ui.add_sized([70.0, 70.0], egui::ImageButton::new(include_image!("../../../assets/gps_true.png")).frame(false)).clicked(){
                        app.gps_reader_task.is_running.store(false,Ordering::Release);
                    }
                },
                false=>{
                    if ui.add_sized([70.0, 70.0], egui::ImageButton::new(include_image!("../../../assets/gps_false.png")).frame(false)).clicked(){
                        app.gps_reader_task.is_running.store(true,Ordering::Release);
                    }
                }
            } 
            
            ui.add_sized([70.0, 70.0], Image::new(include_image!("../../../assets/wifi_true.png")));
            ui.add_sized([70.0, 70.0], Image::new(include_image!("../../../assets/wifi_false.png")));
        });
        
        
    })
    
}