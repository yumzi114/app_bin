use eframe::egui::{self, color_picker::{color_edit_button_srgba, color_picker_color32}, text::LayoutJob, Align, Color32, FontId, InnerResponse, Layout, RichText, TextEdit, TextFormat, Ui, Widget};
use egui_extras::syntax_highlighting;
use crate::{components::layout::sub_menu_window, RasApp};





pub fn lte_sub_raw_view(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    let mut code=if let Some(csq)=&app.lte_reader_task.last_csq{
        format!(r#"RSSI    :  {}
        CSQ-BER :  {}
        UP_TIME :  {}"#
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
        UP_TIME :  {}"#
        ,cesq.rxlev,cesq.rscp,cesq.ecno,cesq.rsrq,cesq.rsrp,cesq.ber,cesq.time.format("%H:%M:%S"))
    }else{
        "Not Conneted LTE".to_string()
    };
    let mut code2=if let (Some(cnum),Some(caddr))=(&app.lte_reader_task.last_cnum,&app.lte_reader_task.last_cgpaddr){
        format!(r#"NUMBER :  {}
        NUM_TYPE :  {}
        NUM_UPTIME :  {}
        I4_ADDR :  {}
        IP_UPTIME :  {}"#
        ,cnum.number, cnum.n_type,cnum.time.format("%H:%M:%S"),
        caddr.ip_addr,caddr.time.format("%H:%M:%S")
    )
    }else{
        "Not Conneted LTE".to_string()
    };
    
  
    let highlight_values = if let Some(csq)=&app.lte_reader_task.last_csq{
        csq.parser()
    }else{
        vec![String::new(), String::new(), String::new()]
    };
    let highlight_values1 = if let Some(cesq)=&app.lte_reader_task.last_cesq{
        cesq.parser()
    }else{
        vec![String::new(),String::new(),String::new(),String::new(),String::new(), String::new(), String::new()]
    };
    let highlight_values2 = if let (Some(cnum),Some(caddr))=(&app.lte_reader_task.last_cnum,&app.lte_reader_task.last_cgpaddr){
        let mut list = cnum.parser();
        let mut caddr_l = caddr.parser();
        list.append(&mut caddr_l);
        list
    }else{
        vec![String::new(),String::new(),String::new(),String::new(),String::new()]
    };
    // let highlight_color = egui::Color32::LIGHT_GREEN;  
    let value_color = app.menu_ctl.value_color;
    let value_size = app.menu_ctl.feild_font_size;;
    let mut layouter_fn = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
        custom_layouter(ui, buf, wrap_width, &highlight_values, value_color,value_size)
    };
    let mut layouter_fn1 = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
        custom_layouter(ui, buf, wrap_width, &highlight_values1, value_color,value_size)
    };
    let mut layouter_fn2 = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
        custom_layouter(ui, buf, wrap_width, &highlight_values2, value_color,value_size)
    };

    ui.vertical_centered(|ui|{
        
        ui.columns(2, |colums|{
            colums[0].vertical_centered(|ui|{
                ui.label(RichText::new("Module INFO").size(35.).color(Color32::from_rgb(232, 58, 58)).underline());
                // ui.label("CSQ");
                ui.add(
                    egui::TextEdit::multiline(&mut code2)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .interactive(false)
                        // .code_editor()
                        .desired_rows(1)
                        .lock_focus(false)
                        // .desired_width(1.==)
                        .desired_width(500.)
                        .layouter(&mut layouter_fn2),
                    );
                ui.label(RichText::new("CSQ").size(35.).color(Color32::from_rgb(232, 58, 58)).underline());
                // ui.label("CSQ");
                ui.add(
                    egui::TextEdit::multiline(&mut code)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .interactive(false)
                        // .code_editor()
                        .desired_rows(1)
                        .lock_focus(false)
                        // .desired_width(1.==)
                        .desired_width(500.)
                        .layouter(&mut layouter_fn),
                    );
            });
            colums[1].vertical_centered(|ui|{
                ui.label(RichText::new("CESQ").size(35.).color(Color32::from_rgb(232, 58, 58)).underline());
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