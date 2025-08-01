pub mod gps_page;
pub mod lte_page;
pub mod car_page;
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
            match app.gps_reader_task.is_running.load(Ordering::Relaxed) {
                true=>{
                    if ui.add_sized([50.0, 50.0], egui::ImageButton::new(include_image!("../../../assets/gps_true.png")).frame(false)).clicked(){
                        app.gps_reader_task.is_running.store(false,Ordering::Release);
                    }
                },
                false=>{
                    if ui.add_sized([50.0, 50.0], egui::ImageButton::new(include_image!("../../../assets/gps_false.png")).frame(false)).clicked(){
                        app.gps_reader_task.is_running.store(true,Ordering::Release);
                    }
                }
            } 
            if let Some(csq)=&app.lte_reader_task.last_csq{
                if csq.time > Local::now() - Duration::seconds(1){
                    ui.add_sized([50.0, 50.0], Image::new(include_image!("../../../assets/wifi_true.png")));
                }else{
                    app.lte_reader_task.last_csq=None;
                }
                // if csq.time
                // let asd =Local::now()+Duration::seconds(10);
            }
            else{
                ui.add_sized([50.0, 50.0], Image::new(include_image!("../../../assets/wifi_false.png")));
            }
            
            
        });
        
        
    })
    
}