use eframe::egui::{self, vec2, Align2, Area, Color32, CornerRadius, Frame, InnerResponse, Margin, Rect, RichText, Rounding, SidePanel, Stroke, Ui};
use egui::{StrokeKind};
// use eframe::egui::Ui::child_ui;
use crate::RasApp;







pub fn lte_sub_sms_view(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.label("SMS");
        let mut show_menu = app.menu_ctl.test_sms_nt;
        let screen_rect = ctx.screen_rect();
        let offset_x = 20.0;        // 우측에서 안쪽으로 당김
        let offset_y = 190.0;        // 상단에서 내려오는 거리
        let panel_width = 180.0;
        let panel_height = screen_rect.height() - 250.0; // 전체 높이에서 줄일 값 (예: 100px 줄이기)

        let rect = egui::Rect::from_min_size(
            egui::Pos2::new(screen_rect.max.x - panel_width - offset_x, screen_rect.top() + offset_y),
            egui::vec2(panel_width, panel_height),
        );

        // 배경
        ui.painter().rect_filled(rect, CornerRadius::same(8), Color32::from_rgba_unmultiplied(40, 40, 50, 230));
        ui.painter().rect_stroke(rect, CornerRadius::same(8), Stroke::new(1.0, Color32::BLACK), StrokeKind::Middle);

        // 내부 UI
        let mut menu_ui = ui.child_ui(rect.shrink(10.0), *ui.layout(), None);

        egui::ScrollArea::vertical().show(&mut menu_ui, |ui| {
            ui.vertical_centered(|ui| {
                // ui.heading("📂 메뉴");
                // ui.separator();
                // if ui.button("📡 RF").clicked() {}
                // if ui.button("📞 LTE").clicked() {}
                // if ui.button("🚗 CAR").clicked() {}
                // if ui.button("🗺 GPS").clicked() {}
                // ui.separator();
                // if ui.button("❌ 닫기").clicked() {}
            });
        });
    })
    
}