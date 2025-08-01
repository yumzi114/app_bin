use eframe::egui::{self, text::LayoutJob, Color32, FontId, InnerResponse, TextEdit, TextFormat, Ui, Widget};
use egui_extras::syntax_highlighting;
use crate::{components::layout::sub_menu_window, RasApp};





pub fn lte_sub_raw_view(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    let mut code=if let Some(csq)=&app.lte_reader_task.last_csq{
        format!(r#"RSSI    :  {}
        CSQ-BER :  {}
        UP_TIME :  {}
        "#
        ,csq.rssi,csq.ber,csq.time.format("%H:%M:%S"))
    }else{
        "Not Conneted LTE".to_string()
    };
    
    let mut rssi = 0;
    let mut csq_ber = 0;
    let mut uptime = String::new();
    let mut dummy = String::new();
    if let Some(csq)=&app.lte_reader_task.last_csq{
        rssi=csq.rssi;
        csq_ber=csq.ber;
        uptime =csq.time.clone().format("%H:%M:%S").to_string();
        // uptime=format!("{}",asdd.format("%H:%M:%S")).as_str();
    }
   
    ui.horizontal_centered(|ui|{
        ui.columns(2, |colums|{
            colums[0].vertical_centered(|ui|{
                ui.label("CSQ");
                ui.columns(2, |columss|{
                    // let rect_left = columss[0].available_rect_before_wrap();
                    // columss[0].painter().rect_filled(
                    //     rect_left,
                    //     egui::Rounding::same(6.0),
                    //     egui::Color32::from_rgb(40, 40, 40), // 배경색
                    // );
                   
                    columss[0].with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                        ui.label("RSSI : ");
                        ui.label("DDD : ");
                    });
                    columss[1].with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        ui.label("SSSS");
                        ui.label("AAAA");
                    });
                    
                });
                
                ui.end_row();
                
            });
            colums[1].vertical_centered(|ui|{
                ui.label("RI");
            });
            
        });
        
    })
}