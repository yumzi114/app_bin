use std::sync::atomic::Ordering;

use chrono::{DateTime, Local};
use eframe::egui::{self, Align, Color32, Layout, RichText, ViewportBuilder};
use better_default::Default;
use crate::{
    components::{layout::{feild_font_edit, menu_button_layout, sub_menu_open}, setup_custom_fonts},
    contents::{
        car_adapter::car_content_view, connect_bt, gps_adapter::gps_content_view,
        lte_adapter::lte_content_view, main_page::main_content_view, rf_page::rf_content_view,
    },
};
mod components;
mod contents;
use app_lib::ctr_mod::board_ctl::{board_reader_thread, Board_task};


#[derive(Default,PartialEq)]
struct Menu_Ctl{
    state:App_Menu,
    #[default(false)]
    side_open:bool,
    anim:f32,
    #[default(Color32::from_rgb(255, 0, 0))]
    value_color:Color32,
    #[default(24.)]
    feild_font_size:f32,
    #[default(false)]
    test_sms_nt:bool,
    #[default(true)]
    test_sms_pop_nt:bool,
    #[default(String::new())]
    test_gps_str:String,
    // #[default(false)]
    // lte_side:bool,
    // lte_anim:f32,
}
// #[derive(PartialEq)]
#[derive(Default,PartialEq)]
enum App_Menu {
    #[default(0:0)]
    MAIN(u8),
    GPS(u8),
    LTE(u8),
    CAR(u8),
    RF(u8),
}

#[tokio::main]
async fn main() {
    let windows = ViewportBuilder {
        title: Some(String::from("Ras Board app")),
        app_id: Some(String::from("Ras Board as")),
        fullsize_content_view: Some(true),
        // inner_size: Some([1280.0, 800.0].into()),
        titlebar_shown: Some(false),
        resizable: Some(false),
        fullscreen: Some(true),
        ..Default::default()
    };
    // windows.with_inner_size([1280.0, 240.0]);
    let options = eframe::NativeOptions {
        viewport: windows,
        // default_theme:Theme::Dark,
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Ras Board app",
        options,
        Box::new(|cc| {
            let mut app = RasApp::new(cc);
            // app.gps_reader_task.runner().unwrap();
            // gps_reader_thread(app.gps_reader_task.msg_tx.clone(), app.gps_reader_task.closer_rx.clone());
            // gps_reader_thread(
            //     app.gps_reader_task.msg_tx.clone(),
            //     app.gps_reader_task.is_running.clone(),
            //     app.gps_reader_task.nmea.clone(),
            //     // app.gps_reader_task.power_pin.clone(),
            //     app.gps_reader_task.start_time.clone(),
            // );
            // lte_reader_thread(app.lte_reader_task.msg_tx.clone());
            // lte_sender_thread(
            //     app.lte_reader_task.app_rx.clone()
            //     // app.lte_reader_task.msg_tx.clone()
            // );
            board_reader_thread(app.board_task.protocol_tx.clone());
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<RasApp>::new(app))
        }),
    );
}
// #[derive(Default,PartialEq)]

// #[derive(Clone)]
// #[derive(Clone, Copy, Default)]
struct RasApp {
    board_task:Board_task,
    menu_ctl: Menu_Ctl,
    test_list: Vec<String>,
    sms_list:Vec<(String,String)>
}

impl RasApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        RasApp {
            board_task: Board_task::new(),
            menu_ctl: Menu_Ctl::default(),
            test_list: vec![],
            sms_list:vec![]
        }
        // Self::default()
    }
}

impl eframe::App for RasApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        
        catppuccin_egui::set_theme(&ctx, catppuccin_egui::FRAPPE);
        if let Ok(pro) = self.board_task.protocol_rx.try_recv() {
            self.board_task.self_update(pro);
            let asdasd = format!("{:?}",pro);
            self.test_list.push(asdasd);
            // ui.label(RichText::new(msg).size(self.menu_ctl.feild_font_size).color(self.menu_ctl.value_color).underline());
        };
        // if let Ok(l_msg) = self.lte_reader_task.msg_rx.try_recv() {
        //     match self.lte_reader_task.pending_cmgl.as_mut(){
        //         Some(header)=>{
        //             let combined = (header.clone(), l_msg.clone());
        //             if !self.sms_list.contains(&combined){
        //                 self.sms_list.push(combined);
        //             }
        //              // (헤더, 본문) 형태 저장
        //             // *header = String::new();       // 상태 초기화
        //             self.lte_reader_task.pending_cmgl = None;
        //         },
        //         None => {
        //             match l_msg{
                
        //                 msg if msg.starts_with("+CSQ: ") => {
        //                     self.lte_reader_task.last_csq=Some(Csq::new(msg.clone()));
        //                 },
        //                 msg if msg.starts_with("+CESQ: ") => {
        //                     let cesq = Cesq::new(msg.clone());
        //                     self.lte_reader_task.check_push_cesq(cesq.clone());
        //                     self.lte_reader_task.last_cesq=Some(cesq);
        //                 },
        //                 msg if msg.starts_with("+CGPADDR: ") => {
        //                     self.lte_reader_task.last_cgpaddr=Some(CgpAddr::new(msg.clone()));
        //                 },
        //                 msg if msg.starts_with("+CNUM: ") => {
        //                     self.lte_reader_task.last_cnum=Some(Cnum::new(msg.clone()));
        //                 },
        //                 msg if msg.starts_with("+CMGF: ") => {
        //                     self.lte_reader_task.cmgf=CMGF::new(msg.clone());
        //                 },
        //                 msg if msg.starts_with("+CMGL: ") => {
        //                     self.lte_reader_task.pending_cmgl = Some(msg.clone());

        //                     // self.test_list.push(msg);
        //                 },
        //                 msg if msg.starts_with("+CPMS: ") => {
        //                     // self.test_list.push(msg);
        //                 },
        //                 msg if msg.starts_with("OK") => {
        //                     // self.lte_reader_task.last_csq=Some(Csq::new(msg.clone()));
        //                 },
        //                 _=>{
        //                     self.test_list.push(l_msg);
        //                     // self.test_list.push(l_msg);
        //                 }
        //             }

        //         }
        //     }
            
        //     // ui.label(l_msg);
        // }
        egui::TopBottomPanel::top("time_head")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(&Local::now().format("%Y-%m-%d %H:%M:%S").to_string())
                            .strong()
                            .size(25.0),
                    );
                });
            });
        egui::TopBottomPanel::top("menu")
            .show_separator_line(false)
            .show(ctx, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.style_mut().override_font_id = Some(egui::FontId::proportional(55.0)); // 글씨 크기
                    ui.spacing_mut().button_padding = egui::vec2(10.0, 1.0); // 버튼 패딩 확대
                    ui.spacing_mut().item_spacing.x = 50.0;
                    ui.menu_button("🏠", |ui| {
                        if ui.button(RichText::new("MAIN").strong().size(30.0))
                            .clicked()
                        {
                            self.menu_ctl.state = App_Menu::MAIN(0);
                        }
                        if ui.button(RichText::new("Quit").strong().size(30.0))
                            .clicked()
                        {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    // let mut buttond = egui::Button::new("🗺").frame(true).fill(egui::Color32::from_rgb(200, 50, 50)).t;
                    if menu_button_layout(ui,self, App_Menu::GPS(0)).clicked() {
                        self.menu_ctl.state = App_Menu::GPS(0);
                    }
                   
                    if menu_button_layout(ui,self, App_Menu::LTE(0)).clicked() {
                        self.menu_ctl.state = App_Menu::LTE(0);
                    }
                    if menu_button_layout(ui,self, App_Menu::CAR(0)).clicked() {
                        self.menu_ctl.state = App_Menu::CAR(0);
                    }
                    
                    if menu_button_layout(ui,self, App_Menu::RF(0)).clicked() {
                        self.menu_ctl.state = App_Menu::RF(0);
                    }
                    connect_bt(self, ui);
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.separator();
            ui.horizontal_wrapped(|ui|{
                sub_menu_open(ui,self);
                feild_font_edit(ui,self);
            });
            
            match self.menu_ctl.state {
                App_Menu::CAR(_) => {
                    car_content_view(self, ui);
                }
                App_Menu::GPS(_) => {
                    gps_content_view(self, ui,ctx);
                }
                App_Menu::LTE(_) => {
                    lte_content_view(self, ui,ctx);
                }
                App_Menu::MAIN(_) => {
                    main_content_view(self, ui);
                }
                App_Menu::RF(_) => {
                    rf_content_view(self, ui);
                }
                _=>{}
            }
            
            
            
        });
    }
    
}
