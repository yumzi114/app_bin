use eframe::egui::{self, text::LayoutJob, Color32, FontId, InnerResponse, RichText, TextEdit, TextFormat, Ui, Widget};
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
    // let dd = app.lte_reader_task.
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
    // let mut theme =
    //         egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
    //     ui.collapsing("Theme", |ui| {
    //         ui.group(|ui| {
    //             theme.ui(ui);
    //             theme.clone().store_in_memory(ui.ctx());
    //         });
    //     });
    let highlight_values = vec![rssi.to_string(), csq_ber.to_string(), uptime.clone()];
    let highlight_color = egui::Color32::LIGHT_GREEN;  
    let mut layouter_fn = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
        custom_layouter(ui, buf, wrap_width, &highlight_values, highlight_color)
    };

    // let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
    //     let text = buf.as_str();
    //     let mut job = egui::text::LayoutJob::default();
    //     let font = egui::FontId::new(25.0, egui::FontFamily::Proportional);
    //     for line in text.lines() {
    //         let tokens: Vec<&str> = line.split_whitespace().collect();
    //         for (i, token) in tokens.iter().enumerate() {
    //             let color = if i == tokens.len() - 1 {
    //                 if *token == rssi.to_string() {
    //                     egui::Color32::LIGHT_GREEN
    //                 } else if *token == csq_ber.to_string() {
    //                     egui::Color32::LIGHT_BLUE
    //                 } else if *token == uptime {
    //                     egui::Color32::YELLOW
    //                 } else {
    //                     egui::Color32::WHITE
    //                 }
    //             } else {
    //                 egui::Color32::WHITE
    //             };
    //             job.append(
    //                 &format!("{} ", token),
    //                 0.0,
    //                 egui::text::TextFormat {
    //                     font_id: font.clone(),
    //                     color,
    //                     ..Default::default()
    //                 },
    //             );
    //         }
    //         job.append(
    //             "\n",
    //             0.0,
    //             egui::text::TextFormat {
    //                 font_id: font.clone(),
    //                 color: egui::Color32::WHITE,
    //                 ..Default::default()
    //             },
    //         );
    //     }
    
    //     job.wrap.max_width = wrap_width;
    //     ui.fonts(|f| f.layout_job(job))
    // };
    ui.vertical_centered(|ui|{
        ui.columns(2, |colums|{
            colums[0].vertical_centered(|ui|{
                ui.label(RichText::new("CSQ").size(35.).color(Color32::from_rgb(232, 58, 58)).underline());
                // ui.label("CSQ");
                ui.add(
                    egui::TextEdit::multiline(&mut code)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .interactive(false)
                        // .code_editor()
                        .desired_rows(10)
                        .lock_focus(false)
                        // .desired_width(1.==)
                        .desired_width(400.)
                        .layouter(&mut layouter_fn),
                    );
            });
            colums[1].vertical_centered(|ui|{
                ui.label(RichText::new("CESQ").size(35.).color(Color32::from_rgb(232, 58, 58)).underline());
                // ui.label("CESQ");
            });
            
        });
        
    })
}


fn custom_layouter(
    ui: &egui::Ui,
    buf: &dyn egui::TextBuffer,
    wrap_width: f32,
    highlight_values: &[String],
    highlight_color: egui::Color32,
) -> std::sync::Arc<egui::Galley> {
    let text = buf.as_str();
    let mut job = egui::text::LayoutJob::default();
    let font_id = egui::FontId::new(25.0, egui::FontFamily::Proportional);
    for line in text.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        for (i, token) in tokens.iter().enumerate() {
            let mut color = egui::Color32::WHITE;
            if i == tokens.len() - 1 && highlight_values.iter().any(|v| v == token) {
                color = highlight_color;
            }
            job.append(
                &format!("{} ", token),
                0.0,
                egui::text::TextFormat {
                    font_id: font_id.clone(),
                    color,
                    ..Default::default()
                },
            );
        }
        job.append(
            "\n",
            0.0,
            egui::text::TextFormat {
                font_id: font_id.clone(),
                color: egui::Color32::WHITE,
                ..Default::default()
            },
        );
    }
    job.wrap.max_width = wrap_width;
    ui.fonts(|f| f.layout_job(job))
}