use eframe::egui::{self, color_picker::{color_edit_button_srgba, color_picker_color32}, text::LayoutJob, Align, Color32, FontId, InnerResponse, Layout, RichText, TextEdit, TextFormat, Ui, Widget};
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
    let mut code1=if let Some(cesq)=&app.lte_reader_task.last_cesq{
        format!(r#"RXLEV(2G)   :  {}
        RSCP(3G) :  {}
        ECNO(3G) :  {}
        RSRQ(LTE) :  {}
        RSRP(LTE) :  {}
        BER :  {}
        UP_TIME :  {}
        "#
        ,cesq.rxlev,cesq.rscp,cesq.ecno,cesq.rsrq,cesq.rsrp,cesq.ber,cesq.time.format("%H:%M:%S"))
    }else{
        "Not Conneted LTE".to_string()
    };
    // let dd = app.lte_reader_task.
    let (rssi, csq_ber, uptime) = if let Some(csq) = &app.lte_reader_task.last_csq {
        (
            csq.rssi,
            csq.ber,
            csq.time.format("%H:%M:%S").to_string(),
        )
    } else {
        (0, 0, String::new())
    };
    let (rxlev,ber,rscp,ecno,rsrq,rsrp,uptime1) = if let Some(cesq) = &app.lte_reader_task.last_cesq {
        (
            cesq.rxlev,
            cesq.ber,
            cesq.rscp,
            cesq.ecno,
            cesq.rsrq,
            cesq.rsrp,
            cesq.time.format("%H:%M:%S").to_string(),
        )
    } else {
        (0, 0,0,0,0,0, String::new())
    };
  
    let highlight_values = vec![rssi.to_string(), csq_ber.to_string(), uptime.clone()];
    let highlight_values1 = vec![rxlev.to_string(),ber.to_string(),rscp.to_string(),ecno.to_string(),rsrq.to_string(),rsrp.to_string(), uptime1.clone()];
    // let highlight_color = egui::Color32::LIGHT_GREEN;  
    let value_color = app.menu_ctl.value_color;
    let value_size = app.menu_ctl.feild_font_size;;
    let mut layouter_fn = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
        custom_layouter(ui, buf, wrap_width, &highlight_values, value_color,value_size)
    };
    let mut layouter_fn1 = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
        custom_layouter(ui, buf, wrap_width, &highlight_values1, value_color,value_size)
    };

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
                ui.add(
                    egui::TextEdit::multiline(&mut code1)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .interactive(false)
                        // .code_editor()
                        .desired_rows(10)
                        .lock_focus(false)
                        // .desired_width(1.==)
                        .desired_width(400.)
                        .layouter(&mut layouter_fn1),
                    );
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
    font_size:f32
) -> std::sync::Arc<egui::Galley> {
    let text = buf.as_str();
    let mut job = egui::text::LayoutJob::default();
    let font_id = egui::FontId::new(font_size, egui::FontFamily::Proportional);
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