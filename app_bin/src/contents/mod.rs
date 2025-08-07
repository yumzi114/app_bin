pub mod gps_adapter;
pub mod lte_adapter;
pub mod car_adapter;
pub mod rf_page;
pub mod main_page;
pub mod lte_sub_page;
pub mod gps_sub_page;



use std::sync::atomic::Ordering;

use chrono::{Duration, Local};
use eframe::egui::{self, include_image, Align, Image, InnerResponse, Layout, RichText, Ui};
use crate::RasApp;

pub fn connect_bt(app:&mut RasApp, ui: &mut Ui)->InnerResponse<()>{
    ui.vertical_centered_justified(|ui|{
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            
            match (app.board_task.protocol.gps_lat,app.board_task.protocol.gps_lat) {
                (lat, lon) if lat > 0.0 && lon > 0.0 =>{
                    if ui.add_sized([50.0, 50.0], egui::ImageButton::new(include_image!("../../../assets/gps_true.png")).frame(false)).clicked(){
                        // app.gps_reader_task.is_running.store(false,Ordering::Release);
                    }
                },
                _=>{
                    if ui.add_sized([50.0, 50.0], egui::ImageButton::new(include_image!("../../../assets/gps_false.png")).frame(false)).clicked(){
                        // app.gps_reader_task.is_running.store(true,Ordering::Release);
                    }
                }
            } 
            match app.board_task.protocol.rssi{
                rssi if rssi > 0=>{
                    ui.add_sized([50.0, 50.0], Image::new(include_image!("../../../assets/wifi_true.png")));
                }
                _=>{
                    ui.add_sized([50.0, 50.0], Image::new(include_image!("../../../assets/wifi_false.png")));
                }
            }
            
            
            
        });
        
        
    })
    
}