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
        ui.label(RichText::new(d_str).strong().size(20.0));
        
        ui.add_space(10.0);
        TableBuilder::new(ui)
            .striped(true) // 줄무늬 행
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())   // 첫 번째 컬럼 자동 폭
            .column(Column::remainder()) // 두 번째 컬럼은 남는 공간 차지
            .column(Column::exact(100.0)) // 세 번째 컬럼은 폭 100px 고정
            .header(20.0, |mut header| {
                header.col(|ui| { ui.label("이름"); });
                header.col(|ui| { ui.label("나이"); });
                header.col(|ui| { ui.label("직업"); });
            })
            .body(|mut body| {
                for (name, age, job) in [("홍길동", "30", "개발자"), ("김철수", "25", "디자이너"), ("이영희", "28", "기획자")] {
                    body.row(18.0, |mut row| {
                        row.col(|ui| { ui.label(name); });
                        row.col(|ui| { ui.label(age); });
                        row.col(|ui| { ui.label(job); });
                    });
                }
            });
            for (cmd,msg) in &app.sms_list{
                ui.label(cmd);
                ui.label(msg);
            }
        message_pop_menu(app,ui,ctx);
 
    })
    
}

