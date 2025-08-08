use eframe::egui::{Color32, InnerResponse, RichText, ScrollArea, Ui};

use crate::RasApp;







pub fn main_content_view(app:&mut RasApp, ui: &mut Ui)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.heading(RichText::new("THIS MAIN").size(45.).color(Color32::from_rgb(72, 245, 66)).underline());
        let rssi= app.board_task.protocol.rssi;
        let rsrp= app.board_task.protocol.rsrp;
        let rsrp= app.board_task.protocol.rsrp;
        let gps_lat= app.board_task.protocol.gps_lat;
        let gps_lon= app.board_task.protocol.gps_lon;
        let gps_speed_calc= app.board_task.protocol.gps_speed_calc;
        let gps_speed_direct= app.board_task.protocol.gps_speed_direct;
        let text = format!(
            "RSSI :{:?}, RSRQ :{:?}, RSRP :{:?}, 
            LAT :{:?}, LON :{:?}, 
            SP_C :{:?}, SP_D :{:?}"
            ,rssi,
            rsrp,
            rsrp,
            gps_lat,
            gps_lon,
            gps_speed_calc,
            gps_speed_direct,
        );
        ui.label(RichText::new(&text).size(app.menu_ctl.feild_font_size).color(app.menu_ctl.value_color));
        ScrollArea::vertical()
        .auto_shrink([false; 2])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            
            for i in &app.board_task.tracking_list{
                let text = format!("{:?}",i);
                ui.label(&text);
                
            }
        });
        
    })
}