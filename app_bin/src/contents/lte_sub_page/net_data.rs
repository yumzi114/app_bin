use eframe::egui::{self, Color32, InnerResponse, RichText, Ui};

use crate::{components::custom_layouter, RasApp};





pub fn lte_sub_net_view(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    let rec_time = app.board_task.rec_time;
    let rssi = app.board_task.protocol.rssi;
    let rsrp = app.board_task.protocol.rsrp;
    let rsrq = app.board_task.protocol.rsrq;
    let ip = app.board_task.protocol.ip;
    let ip_pid = app.board_task.protocol.cpms;
    let cpms = app.board_task.protocol.cpms;
    let mut code=if (app.board_task.protocol.lte_state & (1 << 3)) != 0{
        format!(r#"LTE INFO
        REC-TIME    : {}
        RSSI    :  {}
        RSRP :  {}
        RSRQ :  {}
        IP :  {}
        IP-TYPE :  {}
        CMPS : {}"#
        ,rec_time.format("%H:%M:%S"),
        rssi,
        rsrp,
        rsrq,
        ip,
        ip_pid,
        cpms
    )
    }else{
        "Not Conneted LTE".to_string()
    };
    let mut code1=if (app.board_task.protocol.lte_state & (1 << 3)) != 0{
        format!(r#"TRACKING INFO
        LAST TRACKING TIME :  {}
        INTERVAL :  {}
        DATAS :  {}"#
        ,app.board_task.tracking_last_time.format("%H:%M:%S"),
        app.board_task.tracking_time,
        app.board_task.tracking_list.len()
    )
    }else{
        "Not Conneted LTE".to_string()
    };

 
    let highlight_values = vec![
        format!("{}",app.board_task.rec_time.format("%H:%M:%S")),
        rssi.to_string(),
        rsrp.to_string(),
        rsrq.to_string(),
        ip.to_string(),
        ip_pid.to_string(),
        cpms.to_string()
        ];
    let highlight_values1 = vec![
        format!("{}",app.board_task.tracking_last_time.format("%H:%M:%S")),
        app.board_task.tracking_time.to_string(),
        app.board_task.tracking_list.len().to_string()
         ];

    let value_color = app.menu_ctl.value_color;
    let value_size = app.menu_ctl.feild_font_size;;
    let mut layouter_fn = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
        custom_layouter(ui, buf, wrap_width, &highlight_values, value_color,value_size)
    };
    let mut layouter_fn1 = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
        custom_layouter(ui, buf, wrap_width, &highlight_values1, value_color,value_size)
    };


    ui.vertical_centered(|ui|{
        ui.heading(RichText::new("THIS LTE_VIEW").size(45.).color(Color32::from_rgb(72, 245, 66)).underline());
        ui.columns(2, |colums|{
            colums[0].vertical_centered(|ui|{
                ui.label(RichText::new("Module INFO").size(35.).color(Color32::from_rgb(232, 58, 58)).underline());
             
                ui.add(
                    egui::TextEdit::multiline(&mut code)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .interactive(false)
                        // .code_editor()
                        .desired_rows(1)
                        .lock_focus(false)
                        // .desired_width(1.==)
                        .desired_width(540.)
                        .layouter(&mut layouter_fn),
                    );
            });
            colums[1].vertical_centered(|ui|{
                ui.label(RichText::new("Tracking").size(35.).color(Color32::from_rgb(232, 58, 58)).underline());
                ui.add(
                    egui::TextEdit::multiline(&mut code1)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .interactive(false)
                        .code_editor()
                        .desired_rows(5)
                        .lock_focus(false)
                        // .desired_width(1.==)
                        .desired_width(500.)
                        .layouter(&mut layouter_fn1),
                    );
                // ui.label("CESQ");
            });
            
        });
        
    })
}
