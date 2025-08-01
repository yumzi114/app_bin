use std::sync::atomic::Ordering;

use chrono::Local;
use eframe::egui::{self, Align, Layout, RichText, ViewportBuilder};

use crate::{
    components::setup_custom_fonts,
    contents::{
        car_page::car_content_view, connect_bt, gps_page::gps_content_view,
        lte_page::lte_content_view, main_page::main_content_view, rf_page::rf_content_view,
    },
};
mod components;
mod contents;
use app_lib::ctr_mod::{
    gps_ctr::{self, gps_reader_thread, Gps_Reader_task},
    lte_ctr::{lte_reader_thread, lte_sender_thread, Lte_Reader_Task},
};

#[derive(Default)]
enum App_Menu {
    #[default]
    MAIN,
    GPS,
    LTE,
    CAR,
    RF,
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
            gps_reader_thread(
                app.gps_reader_task.msg_tx.clone(),
                app.gps_reader_task.is_running.clone(),
                app.gps_reader_task.nmea.clone(),
                app.gps_reader_task.power_pin.clone(),
                app.gps_reader_task.start_time.clone(),
            );
            lte_reader_thread(app.lte_reader_task.msg_tx.clone());
            lte_sender_thread(
                // app.lte_reader_task.msg_tx.clone()
            );
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<RasApp>::new(app))
        }),
    );
}
// #[derive(Clone)]
// #[derive(Clone, Copy, Default)]
struct RasApp {
    gps_reader_task: Gps_Reader_task,
    lte_reader_task: Lte_Reader_Task,
    menu_state: App_Menu,
    test_list: Vec<String>,
}

impl RasApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        RasApp {
            gps_reader_task: Gps_Reader_task::new(),
            lte_reader_task: Lte_Reader_Task::new(),
            menu_state: App_Menu::default(),
            test_list: vec![],
        }
        // Self::default()
    }
}

impl eframe::App for RasApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        catppuccin_egui::set_theme(&ctx, catppuccin_egui::FRAPPE);
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
                    ui.spacing_mut().button_padding = egui::vec2(15.0, 1.0); // 버튼 패딩 확대

                    ui.menu_button("🏠", |ui| {
                        if ui
                            .button(RichText::new("MAIN").strong().size(30.0))
                            .clicked()
                        {
                            self.menu_state = App_Menu::MAIN;
                        }
                        if ui
                            .button(RichText::new("Quit").strong().size(30.0))
                            .clicked()
                        {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    if ui.button("🗺").clicked() {
                        self.menu_state = App_Menu::GPS;
                    }
                    if ui.button("📞").clicked() {
                        self.menu_state = App_Menu::LTE;
                    }
                    if ui.button("🚗").clicked() {
                        self.menu_state = App_Menu::CAR;
                    }
                    if ui.button("📡").clicked() {
                        self.menu_state = App_Menu::RF;
                    }
                    connect_bt(self, ui);
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.separator();
            match self.menu_state {
                App_Menu::CAR => {
                    car_content_view(self, ui);
                }
                App_Menu::GPS => {
                    gps_content_view(self, ui);
                }
                App_Menu::LTE => {
                    lte_content_view(self, ui);
                }
                App_Menu::MAIN => {
                    main_content_view(self, ui);
                }
                App_Menu::RF => {
                    rf_content_view(self, ui);
                }
            }
            if let Ok(msg) = self.gps_reader_task.msg_rx.try_recv() {
                ui.label(msg);
            }
            if let Ok(l_msg) = self.lte_reader_task.msg_rx.try_recv() {
                // self.test_list.push(l_msg);
                ui.label(l_msg);
            }
        });
    }
}
