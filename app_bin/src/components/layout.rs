use app_lib::ctr_mod::lte_ctr::lte_cmd::CMGF;
use eframe::egui::{self, color_picker::color_edit_button_srgba, Align, Color32, CornerRadius, InnerResponse, Layout, Response, RichText, Stroke, StrokeKind, Ui, Window};
use core::mem::discriminant;
use crate::{App_Menu, RasApp};

pub fn menu_button_layout(ui: &mut Ui,  app:&mut RasApp,app_menu:App_Menu) -> Response {
    // ui.button("atoms")
    let text = match  app_menu{
        App_Menu::CAR(_)=>"🚗",
        App_Menu::LTE(_)=>"📞",
        App_Menu::RF(_)=>"📡",
        App_Menu::MAIN(_)=>"🏠",
        App_Menu::GPS(_)=>"🗺",
    };
    
    let color = if discriminant(&app_menu) == discriminant(&app.menu_ctl.state) {
        egui::Color32::from_rgb(200, 50, 50)
    } else {
        egui::Color32::from_rgb(255, 255, 255)
    };
    ui.add(egui::Button::new(
        RichText::new(text)
        .strong()
        .size(60.0)
        .color(color)
        ).frame(false))
}

pub fn sub_menu_open(ui: &mut Ui,  app:&mut RasApp)-> InnerResponse<()> {
    let button_text = if app.menu_ctl.side_open{"MENU_CLOSE"}else{"MENU_OPEN"};
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
        
        if ui.add(
            egui::Button::new(RichText::new(button_text).size(35.).strong().color(egui::Color32::from_rgb(72, 245, 66)))
                .fill(egui::Color32::from_rgb(0, 120, 215)) 
                .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
        ).clicked() {
            app.menu_ctl.side_open=!app.menu_ctl.side_open;
        }
        //     if ui.button().clicked(){
               
        // }
    })
    
}

pub fn feild_font_edit(ui: &mut Ui,  app:&mut RasApp)-> InnerResponse<()> {
    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
        ui.spacing_mut().interact_size = egui::vec2(60.0, 30.0);
        color_edit_button_srgba(ui, &mut app.menu_ctl.value_color, egui::color_picker::Alpha::BlendOrAdditive);
        
        ui.label(RichText::new("Feild Color : ").size(25.).color(Color32::from_rgb(255, 255, 255)));
        
        ui.add(egui::Slider::new(&mut app.menu_ctl.feild_font_size, 8.0..=35.0)
        .text(RichText::new("Font Size : ").size(25.).color(Color32::from_rgb(255, 255, 255))));
        // ui.label();
        // egui::widgets::color_picker::color_picker_color32(ui, srgba, alpha)
        // ui.label("text");
    })
    
}

pub fn sub_menu_window(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context){
    let target = if app.menu_ctl.side_open { 1.0 } else { 0.0 };
    // let target = 0;
    // app.menu_ctl.gps_anim += (target - app.menu_ctl.gps_anim) * 0.2; 
    //속도보정
    app.menu_ctl.anim += (target - app.menu_ctl.anim) * 0.2;  
    // let alpha = (app.menu_ctl.gps_anim * 255.0) as u8;
    // let eased = 0.5 - 0.5 * ((std::f32::consts::PI * (1.0 - app.menu_ctl.gps_anim)).cos());
    let eased = (std::f32::consts::PI * app.menu_ctl.anim * 0.5).sin().powf(45.5);
    // let alpha = ((app.menu_ctl.gps_anim.powf(1.5)) * 255.0) as u8; 
    let alpha = (eased * 255.0) as u8;
    if (app.menu_ctl.anim - target).abs() > 0.01 {
        ctx.request_repaint();
    }
    if app.menu_ctl.anim > 0.01 {
        let full_width = 220.0 * app.menu_ctl.anim;
            Window::new("SIDE MENU")
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .frame(egui::Frame {
                    shadow: egui::epaint::Shadow::NONE,
                    stroke: Stroke::NONE,
                    ..Default::default()
                })
                .fixed_pos([0.0, 250.0])
                .fixed_size([full_width, ctx.screen_rect().height() - 150.0])
                
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.spacing_mut().item_spacing.y = 50.0;
                        match app.menu_ctl.state{
                            App_Menu::CAR(_)=>{},
                            App_Menu::LTE(_)=>{
                                if ui.label(
                                    egui::RichText::new("All Info").size(35.)
                                        .underline()
                                        .color(egui::Color32::from_rgba_unmultiplied(72, 245, 66, alpha))
                                ).clicked(){
                                    app.menu_ctl.state=App_Menu::LTE(0);
                                    // app.menu_ctl.side_open=false;
                                };
                                if ui.label(
                                    RichText::new("Tracing").size(35.)
                                        .underline()
                                        .color(egui::Color32::from_rgba_unmultiplied(72, 245, 66, alpha))
                                ).clicked(){
                                    app.menu_ctl.state=App_Menu::LTE(1);
                                    // app.menu_ctl.side_open=false;
                                };
                                if ui.label(
                                    egui::RichText::new("NET Data").size(35.)
                                        .underline()
                                        .color(egui::Color32::from_rgba_unmultiplied(72, 245, 66, alpha))
                                ).clicked(){
                                    app.menu_ctl.state=App_Menu::LTE(2);
                                    // app.menu_ctl.side_open=false;
                                };
                                if ui.label(
                                    egui::RichText::new("SMS Data").size(35.)
                                        .underline()
                                        .color(egui::Color32::from_rgba_unmultiplied(72, 245, 66, alpha))
                                ).clicked(){
                                    app.menu_ctl.state=App_Menu::LTE(3);
                                    // app.menu_ctl.side_open=false;
                                };
                            },
                            App_Menu::RF(_)=>{},
                            App_Menu::GPS(_)=>{
                                if ui.label(
                                    egui::RichText::new("All Info").size(35.)
                                        .underline()
                                        .color(egui::Color32::from_rgba_unmultiplied(72, 245, 66, alpha))
                                ).clicked(){
                                    app.menu_ctl.state=App_Menu::GPS(0);
                                };
                                if ui.label(
                                    RichText::new("Map View").size(35.)
                                        .underline()
                                        .color(egui::Color32::from_rgba_unmultiplied(72, 245, 66, alpha))
                                ).clicked(){
                                    app.menu_ctl.state=App_Menu::GPS(1);
                                };
                                if ui.label(
                                    egui::RichText::new("Raw Data").size(35.)
                                        .underline()
                                        .color(egui::Color32::from_rgba_unmultiplied(72, 245, 66, alpha))
                                ).clicked(){
                                    app.menu_ctl.state=App_Menu::GPS(2);
                                };
                            },
                            App_Menu::MAIN(_)=>{},

                        }
                        
                    });
                    
                });
    }
}






// pub fn message_pop_menu(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context){
//     let anim = ctx.animate_bool_with_time("window_anim".into(), app.menu_ctl.test_sms_nt, 0.1);
//         // 화면 전체 rect
//         let screen_rect = ctx.screen_rect();
//         // 애니메이션 패널 크기 (중앙에서 확대/축소)
//         let target_width = 300.0;
//         let target_height = 450.0;
//         let width = target_width * anim;
//         let height = target_height * anim;
//         let center = screen_rect.center();
//         let rect = egui::Rect::from_center_size(center, egui::vec2(width, height))
//         .translate(egui::vec2(460.0, 0.0));// X축으로 이동
//         // 독립 레이어에서 그림
//         // let painter = ctx.layer_painter(egui::LayerId::new(
//         //     egui::Order::Foreground,
//         //     egui::Id::new("popup_layer"),
//         // ));
//         if anim > 0.01 {
//             // 팝업 배경은 항상 그림 (애니메이션 유지)
//             ui.painter().rect_filled(
//                 rect,
//                 CornerRadius::same(8),
//                 Color32::from_rgba_unmultiplied(2, 29, 163, (100.0 * anim) as u8),
//             );
//             ui.painter().rect_stroke(
//                 rect,
//                 CornerRadius::same(8),
//                 Stroke::new(1.0, Color32::from_rgba_unmultiplied(15, 163, 2, 250)),
//                 StrokeKind::Inside,
//             );
//             // 내부 UI는 애니메이션이 충분히 진행된 후에만 표시 (예: anim > 0.8)
//             if app.menu_ctl.test_sms_nt && anim > 0.9 {
//                 let builder = egui::UiBuilder::new()
//                     .max_rect(rect.shrink(10.0))
//                     .layout(*ui.layout());
//                 let mut popup_ui = ui.new_child(builder);
//                 popup_ui.vertical_centered(|ui| {
//                     if app.menu_ctl.test_sms_nt && ctx.input(|i| i.pointer.any_click()) {
//                         if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
//                             if !rect.contains(pos) {
//                                 app.menu_ctl.test_sms_pop_nt=true;
//                                 app.menu_ctl.test_sms_nt = false; // 팝업 영역 밖 클릭 → 닫기
//                             }
//                         }
//                     }
//                     ui.heading(RichText::new("📂\n MESSAGE").size(35.));
//                     ui.separator();
//                     ui.add_space(25.);
//                     if ui.add(
//                         egui::Button::new(RichText::new("LTE MODE CHANGE").size(25.).strong().color(egui::Color32::from_rgb(72, 245, 66)))
//                             .fill(egui::Color32::from_rgb(0, 120, 215)) 
//                             .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE)) 
//                     ).clicked() {
//                     }
//                     let d_str = if app.lte_reader_task.cmgf==CMGF::TEXT{"CHANGE LTE SENSOR DPU MODE"}else{"CHANGE LTE SENSOR TEXT MODE"};
//                     ui.label(RichText::new(d_str).size(15.));
//                     // ui.label(d_str);
//                     ui.add_space(25.);
//                     // if ui.button("LTE MODE CHANGE").clicked() {}
//                     if ui.add(
//                         egui::Button::new(RichText::new("REFRESH ALL SMS").size(25.).strong().color(egui::Color32::from_rgb(72, 245, 66)))
//                             .fill(egui::Color32::from_rgb(0, 120, 215)) 
//                             .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE)) 
//                     ).clicked() {

//                     }
//                     // ui.label("🔃UPDATE RECEIVED MESSAGE");
//                     ui.label(RichText::new("🔃UPDATE RECEIVED MESSAGE").size(15.));
//                     // if ui.button("REFRESH ALL SMS").clicked() {}
//                     ui.add_space(25.);
//                     if ui.add(
//                         egui::Button::new(RichText::new("MESSAGE SEND").size(25.).strong().color(egui::Color32::from_rgb(72, 245, 66)))
//                             .fill(egui::Color32::from_rgb(0, 120, 215)) 
//                             .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE)) 
//                     ).clicked() {

//                     }
                    
//                     ui.label(RichText::new("💌ANOTHER PHONE SEND MESSAGE").size(15.));
//                     // if ui.button("MESSAGE SEND").clicked() {}
//                     ui.add_space(25.);
//                     // ui.separator();
//                     // ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                        
//                     //     if ui.add(
//                     //         egui::Button::new(RichText::new("CLOSE").size(20.).strong().color(egui::Color32::from_rgb(255, 0, 0)))
//                     //             .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255,150)) 
//                     //             .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE)) 
//                     //     ).clicked() {
//                     //         app.menu_ctl.test_sms_nt = false;
//                     //     }
//                     // });
                    
//                 });
//             }
//         }
// }


pub fn message_pop_menu(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context) {
    let anim = ctx.animate_bool_with_time("window_anim".into(), app.menu_ctl.test_sms_nt, 0.1);
    let screen_rect = ctx.screen_rect();

    let rect = egui::Rect::from_center_size(
        screen_rect.center() + egui::vec2(460.0, 0.0),
        egui::vec2(300.0 * anim, 450.0 * anim),
    );

    if anim > 0.01 {
        // Foreground 레이어 생성
        let layer_id = egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("popup"));
        let mut root_ui = egui::Ui::new(ctx.clone(), layer_id.id, egui::UiBuilder::new().max_rect(screen_rect));

        // allocate_ui_at_rect로 직접 배경+위젯 순서 제어
        root_ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            // 1️⃣ 배경
            ui.painter().rect_filled(
                rect,
                egui::CornerRadius::same(8),
                egui::Color32::from_rgba_unmultiplied(2, 29, 163, (150.0 * anim) as u8),
            );
            ui.painter().rect_stroke(
                rect,
                CornerRadius::same(8),
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(15, 163, 2, 250)),
                StrokeKind::Inside,
            );

            // 2️⃣ 위젯 (배경 바로 뒤 순서)
            if anim > 0.9 {
                ui.vertical_centered(|ui| {
                    if app.menu_ctl.test_sms_nt && ctx.input(|i| i.pointer.any_click()) {
                        if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
                            if !rect.contains(pos) {
                                app.menu_ctl.test_sms_pop_nt=true;
                                app.menu_ctl.test_sms_nt = false; // 팝업 영역 밖 클릭 → 닫기
                            }
                        }
                    }
                    ui.add_space(25.);
                    ui.heading(RichText::new("📂MESSAGE").size(35.));
                    ui.separator();
                    ui.add_space(25.);
                    if ui.add(
                        egui::Button::new(RichText::new("LTE MODE CHANGE").size(25.).strong().color(egui::Color32::from_rgb(72, 245, 66)))
                            .fill(egui::Color32::from_rgb(0, 120, 215)) 
                            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE)) 
                    ).clicked() {
                    }
                    let d_str = if app.lte_reader_task.cmgf==CMGF::TEXT{"CHANGE LTE SENSOR DPU MODE"}else{"CHANGE LTE SENSOR TEXT MODE"};
                    ui.label(RichText::new(d_str).size(15.));
                    // ui.label(d_str);
                    ui.add_space(25.);
                    // if ui.button("LTE MODE CHANGE").clicked() {}
                    if ui.add(
                        egui::Button::new(RichText::new("REFRESH ALL SMS").size(25.).strong().color(egui::Color32::from_rgb(72, 245, 66)))
                            .fill(egui::Color32::from_rgb(0, 120, 215)) 
                            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE)) 
                    ).clicked() {
                        // let asdad = String::new()
                        app.lte_reader_task.app_tx.send("AT+CMGL=\"ALL\"".to_string()).unwrap();
                    }
                    // ui.label("🔃UPDATE RECEIVED MESSAGE");
                    ui.label(RichText::new("🔃UPDATE RECEIVED MESSAGE").size(15.));
                    // if ui.button("REFRESH ALL SMS").clicked() {}
                    ui.add_space(25.);
                    if ui.add(
                        egui::Button::new(RichText::new("MESSAGE SEND").size(25.).strong().color(egui::Color32::from_rgb(72, 245, 66)))
                            .fill(egui::Color32::from_rgb(0, 120, 215)) 
                            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE)) 
                    ).clicked() {

                    }
                    
                    ui.label(RichText::new("💌ANOTHER PHONE SEND MESSAGE").size(15.));
                    // if ui.button("MESSAGE SEND").clicked() {}
                    ui.add_space(25.);
                    // ui.separator();
                    // ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                        
                    //     if ui.add(
                    //         egui::Button::new(RichText::new("CLOSE").size(20.).strong().color(egui::Color32::from_rgb(255, 0, 0)))
                    //             .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255,150)) 
                    //             .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE)) 
                    //     ).clicked() {
                    //         app.menu_ctl.test_sms_nt = false;
                    //     }
                    // });
                });
            }
        });
    }
}

pub fn message_pop_button(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context){
    let anim = ctx.animate_bool_with_time("message_bt".into(), app.menu_ctl.test_sms_pop_nt, 0.1);
        // 화면 전체 rect
        let screen_rect = ctx.screen_rect();
        // 애니메이션 패널 크기 (중앙에서 확대/축소)
        let target_width = 150.0;
        let target_height = 50.0;
        let width = target_width * anim;
        let height = target_height * anim;
        let center = screen_rect.center();
        let rect = egui::Rect::from_center_size(center, egui::vec2(width, height))
        .translate(egui::vec2(550.0, -200.0));// X축으로 이동
        // 독립 레이어에서 그림
        // let painter = ctx.layer_painter(egui::LayerId::new(
        //     egui::Order::Foreground,
        //     egui::Id::new("popup_layer"),
        // ));
        if anim > 0.01 {
            // 팝업 배경은 항상 그림 (애니메이션 유지)
            ui.painter().rect_filled(
                rect,
                CornerRadius::same(8),
                Color32::from_rgba_unmultiplied(2, 29, 163, (100.0 * anim) as u8),
            );
            ui.painter().rect_stroke(
                rect,
                CornerRadius::same(8),
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(15, 163, 2, 250)),
                StrokeKind::Inside,
            );
            // 내부 UI는 애니메이션이 충분히 진행된 후에만 표시 (예: anim > 0.8)
            if !app.menu_ctl.test_sms_nt && anim > 0.9 {
                let builder = egui::UiBuilder::new()
                    .max_rect(rect.shrink(10.0))
                    .layout(*ui.layout());
                let mut popup_ui = ui.new_child(builder);
                popup_ui.vertical_centered_justified(|ui|{
                    if ui.label(RichText::new("MESSAGE").size(25.).strong().color(egui::Color32::from_rgb(72, 245, 66))).clicked(){
                        app.menu_ctl.test_sms_pop_nt=false;
                        app.menu_ctl.test_sms_nt=true;
                    };
                });

            }
        }
}