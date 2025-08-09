use std::f32::consts::{PI, TAU};

use eframe::{egui::{self, lerp, pos2, vec2, Align2, Color32, CornerRadius, FontId, InnerResponse, Painter, Pos2, Rect, Response, RichText, Sense, Stroke, StrokeKind, Ui, Vec2}, epaint::{PathShape, RectShape}};

use crate::{components::speed_layout::{altitude_bar, speed_donut}, RasApp};







pub fn gps_info_view(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        app.menu_ctl.speed_anim.update(ctx, app.menu_ctl.temp_speed);
        app.menu_ctl.alt_anim.update(ctx, app.menu_ctl.temp_speed);
        // alt_anim.update(ui.ctx(), raw_alt_m);
        ui.heading(RichText::new("THIS GPS_VIEW").size(45.).color(Color32::from_rgb(72, 245, 66)).underline());
        ui.add(egui::Slider::new(&mut app.menu_ctl.temp_speed, 0.0..=1000.0)
        .text(RichText::new("Font Size : ").size(25.).color(Color32::from_rgb(255, 255, 255))));
        app.menu_ctl.speed_anim.update(ui.ctx(), app.menu_ctl.temp_speed);
        ui.columns(2, |colums|{
            colums[0].vertical_centered(|ui|{
                speed_donut(
                    ui,
                    app.menu_ctl.speed_anim.shown,
                    300.0,    // max km/h
                    500.0,    // size(px)
                    // Color32::from_rgb(50, 230, 160),
                    app.menu_ctl.value_color,
                    Color32::from_gray(80),
                    "SPEED",
                );
            });
            colums[1].vertical_centered(|ui|{
                altitude_bar(
                    ui,
                    app.menu_ctl.alt_anim.shown,
                    0.0, 1000.0,                 // 범위
                    vec2(540.0, 500.0),          // 전체 크기
                    Color32::from_gray(70),      // 트랙
                    Color32::from_rgb(50,230,160), // 채움
                    "ALT",
                );
            });
        });
        
    })
}
