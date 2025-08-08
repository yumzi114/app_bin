use eframe::egui::{self, Color32, InnerResponse, Painter, Pos2, RichText, Stroke, Ui};
use egui_plot::{Plot, PlotPoints, Points};
use osmpbf::{Element, ElementReader};

use crate::{components::layout::sub_menu_window, RasApp};





pub fn gps_sub_raw_view(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    
    ui.vertical_centered(|ui|{
        let lat = app.board_task.protocol.gps_lat;
        let lon = app.board_task.protocol.gps_lon;
        let gps_speed_calc = app.board_task.protocol.gps_speed_calc;
        let gps_speed_direct = app.board_task.protocol.gps_speed_direct;
        ui.label(RichText::new("GPS RAW VIEW").size(35.).color(Color32::from_rgb(232, 58, 58)).underline());
        let text = format!("LAT : {:?}, LON : {:?}",lat,lon);
        let text1 = format!("SPEED_CALC : {:?}, SPEED_DIRECT : {:?}",gps_speed_calc,gps_speed_direct);
        ui.label(RichText::new(&text).size(app.menu_ctl.feild_font_size).color(app.menu_ctl.value_color));
        ui.add_space(10.);
        ui.label(RichText::new(&text1).size(app.menu_ctl.feild_font_size).color(app.menu_ctl.value_color));
        // let history: Vec<(f64, f64)> = app.gps_history.iter().map(|(lat, lon)| (*lon, *lat)).collect();
        // let points = PlotPoints::from_iter(history.into_iter().map(|(x, y)| [x, y]));

        // Plot::new("gps_path")
        //     .view_aspect(1.0)
        //     .show(ui, |plot_ui| {
        //         plot_ui.points(Points::new(points).radius(3.0).color(Color32::LIGHT_BLUE));
        //     });
        
    })
}


