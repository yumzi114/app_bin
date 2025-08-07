use chrono::{DateTime, Duration, Local};
use eframe::egui::{self, InnerResponse, TextStyle, Ui};
use egui_plot::{GridMark, Legend, Line, Plot, PlotPoints};

use crate::RasApp;







pub fn lte_sub_tracing_view(app:&mut RasApp, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.label("lte_sub_tracing_view");
        //시작기준 설정시
        let duration_secs = 30; // 예: 30
        let now = Local::now();
        let start = now - Duration::seconds(duration_secs as i64);
        ui.ctx().style_mut(|style: &mut egui::Style| {
            style.visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::TRANSPARENT;
        });
        // let points_rsrp: PlotPoints = app.lte_reader_task.trac_cesq
        //     .iter()
        //     .map(|cesq| {
        //         let x = (cesq.time - start).num_milliseconds() as f64 / 1000.0; // 초 단위
        //         [x, cesq.rsrp as f64]
        //     })
        //     .collect();
        // let points_rsrq: PlotPoints = app.lte_reader_task.trac_cesq
        //     .iter()
        //     .map(|cesq| {
        //         let x = (cesq.time - start).num_milliseconds() as f64 / 1000.0; // 초 단위
        //         [x, cesq.rsrq as f64]
        //     })
        //     .collect();
        
        ui.label(egui::RichText::new("RSRP (dB)").size(22.0).strong());
        // Plot::new("cesq_plot")
        // // .width(600.0)
        // .height(250.0)
        // // .height(480.0)
        // .include_x(0.0)
        // .include_x(duration_secs as f64)
        // .legend(Legend::default().text_style(TextStyle::Heading))              // 상단 라인 범례 표시
        // // .data_aspect(1.5)                       // X:Y 비율 조정 (더 넓게)
        // .allow_drag(false)                      // 드래그 이동 비활성화
        // .allow_zoom(false)                      // 줌 비활성화 (고정 뷰)
        // .show_x(true)                           // X축 표시
        // .show_y(true)                           // Y축 표시
        // // .x_grid_spacer(move |_| {
        // //     let step = 10.0; // 5초 단위 그리드
        // //     let count = (duration_secs as f64 / step).ceil() as usize;
        // //     (0..=count)
        // //         .map(|i| GridMark { value: i as f64 * step, step_size: step })
        // //         .collect::<Vec<GridMark>>()
        // // })
        // .x_axis_formatter(move |mark, _| {
        //     // X축 포맷을 현재 시간 기준으로 변환
        //     let ts = start + Duration::milliseconds((mark.value * 1000.0) as i64);
        //     ts.format("%M:%S").to_string()
        // })
        // // .y_grid_spacer(|_| {
        // //     (0..=6)
        // //         .map(|i| GridMark { value: i as f64 * 5.0, step_size: 10.0 })
        // //         .collect::<Vec<GridMark>>()
        // // })                 // Y축 그리드 간격
        // .show_background(true)                  // 배경 ON
        // .show_axes(true)                        // 축 표시
        // .show(ui, |plot_ui| {
        //     // RSRQ
        //     plot_ui.line(
        //         Line::new("RSRQ", points_rsrq)
        //             .color(egui::Color32::from_rgb(255, 100, 100))
        //             .width(5.)
        //             .name("RSRQ (dB)").allow_hover(true).highlight(true).fill(3.)
        //     );
        //     // RSRP
        //     plot_ui.line(
        //         Line::new("RSRP", points_rsrp)
        //             .color(egui::Color32::from_rgb(100, 200, 255))
        //             .width(5.)
        //             .name("RSRP (dB)").allow_hover(true).highlight(true).fill(3.)
        //     );
  
        // });
    })
    
}