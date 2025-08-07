use app_lib::ctr_mod::lte_ctr::lte_cmd::CMGF;
use eframe::egui::{self, vec2, Align, Align2, Area, Color32, CornerRadius, Frame, InnerResponse, Layout, Margin, Rect, RichText, Rounding, SidePanel, Stroke, Ui};
use egui::{StrokeKind};
use egui_extras::{Column, TableBuilder};
// use eframe::egui::Ui::child_ui;
use crate::{components::layout::{message_pop_button, message_pop_menu}, RasApp};
use egui::UiBuilder;






pub fn lte_sub_sms_view(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        // ui.label("SMS");
        ui.heading(RichText::new("MESSAGE LIST").size(35.).color(Color32::from_rgb(72, 245, 66)).underline());
        let d_str = if app.lte_reader_task.cmgf==CMGF::TEXT{"If you are not in the message list, refresh the message."}else {"LTE module is PDU MODE\nchecking the sensor, refresh the message to retrieve it."};
        
        message_pop_button(app,ui,ctx);
        
        
        // if ui.button(if app.menu_ctl.test_sms_nt { "창 닫기" } else { "창 열기" }).clicked() {
        //     app.menu_ctl.test_sms_nt = !app.menu_ctl.test_sms_nt;
        // }
        // let mode_str = if app.lte_reader_task.cmgf==CMGF::TEXT{"TEXT"}else {"PDU"};
        ui.label(RichText::new(d_str).strong().size(15.0));
        
        TableBuilder::new(ui)
            .striped(true) 
            // .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())   
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            // .column(Column::remainder()) // 두 번째 컬럼은 남는 공간 차지
            // .column(Column::exact(100.0)) // 세 번째 컬럼은 폭 100px 고정
            .header(50.0, |mut header| {
                header.col(|ui| { 
                    // let rect = ui.max_rect();
                    // ui.painter().rect_filled(rect, CornerRadius::same(8), Color32::from_rgba_unmultiplied(11, 12, 13,150));

                    ui.label(RichText::new("Num").size(25.).color(Color32::from_rgba_unmultiplied(255, 255, 255, 255)).strong().underline()); 
                });
                header.col(|ui| { 
                    ui.label(RichText::new("REC").size(25.).color(Color32::from_rgba_unmultiplied(255, 255, 255, 255)).strong().underline());
                    // ui.label("REC"); 
                });
                header.col(|ui| { 
                    ui.label(RichText::new("Phone number").size(25.).color(Color32::from_rgba_unmultiplied(255, 255, 255, 255)).strong().underline());
                });
                header.col(|ui| { 
                    ui.label(RichText::new("Date").size(25.).color(Color32::from_rgba_unmultiplied(255, 255, 255, 255)).strong().underline());
                });
                header.col(|ui| { 
                    ui.label(RichText::new("Time").size(25.).color(Color32::from_rgba_unmultiplied(255, 255, 255, 255)).strong().underline());
                });
                header.col(|ui| { 
                    ui.label(RichText::new("Message").size(25.).color(Color32::from_rgba_unmultiplied(255, 255, 255, 255)).strong().underline());
                });
            })
            .body(|mut body| {
                for (cmd,msg) in &app.sms_list{
                    let rest = cmd.trim().strip_prefix("+CMGL:").expect("Invalid CESQ format");
                    let sss=rest.replace("\"","").trim().to_string();
                    let parts: Vec<&str> = sss.split(',').collect();
                    body.row(25., |mut row|{
                        row.col(|ui| { 
                            ui.label(RichText::new(parts[0].trim()).size(app.menu_ctl.feild_font_size).color(app.menu_ctl.value_color));
                        });
                        row.col(|ui| { 
                            ui.label(RichText::new(parts[1].trim()).size(app.menu_ctl.feild_font_size).color(app.menu_ctl.value_color));
                        });
                        row.col(|ui| {
                            ui.label(RichText::new(parts[2].trim()).size(app.menu_ctl.feild_font_size).color(app.menu_ctl.value_color));
                            });
                        row.col(|ui| {
                            ui.label(RichText::new(parts[4].trim()).size(app.menu_ctl.feild_font_size).color(app.menu_ctl.value_color));
                            });
                        row.col(|ui| {
                            ui.label(RichText::new(parts[5].trim()).size(app.menu_ctl.feild_font_size).color(app.menu_ctl.value_color));
                            });
                        row.col(|ui| {
                            ui.label(RichText::new(msg).size(app.menu_ctl.feild_font_size).color(app.menu_ctl.value_color));
                            });
            
                    });
                }
            });
        
        ui.add_space(10.0);
        
                // for (name, age, job) in [("nan", "30", "stu"), ("kim", "25", "de"), ("lee", "28", "plan")] {
                //     body.row(18.0, |mut row| {
                //         row.col(|ui| { ui.label(name); });
                //         row.col(|ui| { ui.label(age); });
                //         row.col(|ui| { ui.label(job); });
                //     });
                // }
         
            // for (cmd,msg) in &app.sms_list{
            //     ui.label(cmd);
            //     ui.label(msg);
            // }
        message_pop_menu(app,ui,ctx);
 
    })
    
}

