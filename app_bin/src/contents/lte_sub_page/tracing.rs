use eframe::egui::{self, InnerResponse, Ui};
use egui_plot::{Line, Plot, PlotPoints};

use crate::RasApp;







pub fn lte_sub_tracing_view(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.label("lte_sub_tracing_view");
        Plot::new("example_plot").show(ui, |plot_ui| {
            let points: PlotPoints = (0..1000)
                .map(|i| {
                    let x = i as f64 * 0.01;
                    [x, x.sin()]
                })
                .collect();
        
            plot_ui.line(Line::new("sin(x)", points)); // ✅ 이름 먼저, 데이터 나중
        });
    })
    
}