use eframe::egui::{self, color_picker::color_edit_button_srgba, Align, Color32, InnerResponse, Layout, Response, RichText, Stroke, Ui, Window};

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
    let color = if app_menu== app.menu_ctl.state{
        egui::Color32::from_rgb(200, 50, 50)
    }else{egui::Color32::from_rgb(255, 255, 255)
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
            if ui.button(RichText::new(button_text).size(35.).color(egui::Color32::from_rgb(72, 245, 66))).clicked(){
                app.menu_ctl.side_open=!app.menu_ctl.side_open;
        }
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
                                ui.label(
                                    egui::RichText::new("All Info").size(35.)
                                        .underline()
                                        .color(egui::Color32::from_rgba_unmultiplied(72, 245, 66, alpha))
                                );
                                ui.label(
                                    RichText::new("Trafic").size(35.)
                                        .underline()
                                        .color(egui::Color32::from_rgba_unmultiplied(72, 245, 66, alpha))
                                );
                                if ui.label(
                                    egui::RichText::new("Raw Data").size(35.)
                                        .underline()
                                        .color(egui::Color32::from_rgba_unmultiplied(72, 245, 66, alpha))
                                ).clicked(){
                                    app.menu_ctl.state=App_Menu::LTE(2);
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

