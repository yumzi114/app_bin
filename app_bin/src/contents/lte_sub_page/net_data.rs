use eframe::egui::{self, Color32, InnerResponse, RichText, Ui};

use crate::{components::custom_layouter, RasApp};





pub fn lte_sub_net_view(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    // let mut code=if let Some(csq)=&app.lte_reader_task.last_csq{
    //     format!(r#"RSSI    :  {}
    //     CSQ-BER :  {}
    //     UP_TIME :  {}"#
    //     ,csq.rssi,csq.ber,csq.time.format("%H:%M:%S"))
    // }else{
    //     "Not Conneted LTE".to_string()
    // };
    // let mut code1=if let Some(cesq)=&app.lte_reader_task.last_cesq{
    //     format!(r#"RXLEV(2G)   :  {}
    //     RSCP(3G) :  {}
    //     ECNO(3G) :  {}
    //     RSRQ(LTE) :  {}
    //     RSRP(LTE) :  {}
    //     BER :  {}
    //     UP_TIME :  {}"#
    //     ,cesq.rxlev,cesq.rscp,cesq.ecno,cesq.rsrq,cesq.rsrp,cesq.ber,cesq.time.format("%H:%M:%S"))
    // }else{
    //     "Not Conneted LTE".to_string()
    // };
    // let mut code2=if let (Some(cnum),Some(caddr))=(&app.lte_reader_task.last_cnum,&app.lte_reader_task.last_cgpaddr){
    //     format!(r#"NUMBER :  {}
    //     NUM_TYPE :  {}
    //     NUM_UPTIME :  {}
    //     IP4_ADDR :  {}
    //     IP_UPTIME :  {}"#
    //     ,cnum.number, cnum.n_type,cnum.time.format("%H:%M:%S"),
    //     caddr.ip_addr,caddr.time.format("%H:%M:%S")
    // )
    // }else{
    //     "Not Conneted LTE".to_string()
    // };
    
  
    // let highlight_values = if let Some(csq)=&app.lte_reader_task.last_csq{
    //     csq.parser()
    // }else{
    //     vec![String::new(), String::new(), String::new()]
    // };
    // let highlight_values1 = if let Some(cesq)=&app.lte_reader_task.last_cesq{
    //     cesq.parser()
    // }else{
    //     vec![String::new(),String::new(),String::new(),String::new(),String::new(), String::new(), String::new()]
    // };
    // let highlight_values2 = if let (Some(cnum),Some(caddr))=(&app.lte_reader_task.last_cnum,&app.lte_reader_task.last_cgpaddr){
    //     let mut list = cnum.parser();
    //     let mut caddr_l = caddr.parser();
    //     list.append(&mut caddr_l);
    //     list
    // }else{
    //     vec![String::new(),String::new(),String::new(),String::new(),String::new()]
    // };
    // // let highlight_color = egui::Color32::LIGHT_GREEN;  
    // let value_color = app.menu_ctl.value_color;
    // let value_size = app.menu_ctl.feild_font_size;;
    // let mut layouter_fn = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
    //     custom_layouter(ui, buf, wrap_width, &highlight_values, value_color,value_size)
    // };
    // let mut layouter_fn1 = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
    //     custom_layouter(ui, buf, wrap_width, &highlight_values1, value_color,value_size)
    // };
    // let mut layouter_fn2 = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
    //     custom_layouter(ui, buf, wrap_width, &highlight_values2, value_color,value_size)
    // };

    ui.vertical_centered(|ui|{
        
        ui.columns(2, |colums|{
            colums[0].vertical_centered(|ui|{
                ui.label(RichText::new("Module INFO").size(35.).color(Color32::from_rgb(232, 58, 58)).underline());
                // ui.label("CSQ");
                // ui.add(
                //     egui::TextEdit::multiline(&mut code2)
                //         .font(egui::TextStyle::Monospace) // for cursor height
                //         .interactive(false)
                //         // .code_editor()
                //         .desired_rows(1)
                //         .lock_focus(false)
                //         // .desired_width(1.==)
                //         .desired_width(500.)
                //         .layouter(&mut layouter_fn2),
                //     );
                // ui.label(RichText::new("CSQ").size(35.).color(Color32::from_rgb(232, 58, 58)).underline());
                // // ui.label("CSQ");
                // ui.add(
                //     egui::TextEdit::multiline(&mut code)
                //         .font(egui::TextStyle::Monospace) // for cursor height
                //         .interactive(false)
                //         // .code_editor()
                //         .desired_rows(1)
                //         .lock_focus(false)
                //         // .desired_width(1.==)
                //         .desired_width(500.)
                //         .layouter(&mut layouter_fn),
                //     );
            });
            colums[1].vertical_centered(|ui|{
                ui.label(RichText::new("CESQ").size(35.).color(Color32::from_rgb(232, 58, 58)).underline());
                // ui.add(
                //     egui::TextEdit::multiline(&mut code1)
                //         .font(egui::TextStyle::Monospace) // for cursor height
                //         .interactive(false)
                //         .code_editor()
                //         .desired_rows(5)
                //         .lock_focus(false)
                //         // .desired_width(1.==)
                //         .desired_width(500.)
                //         .layouter(&mut layouter_fn1),
                //     );
                // ui.label("CESQ");
            });
            
        });
        
    })
}
