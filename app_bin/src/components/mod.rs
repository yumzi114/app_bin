use eframe::egui::{self, Context};
pub mod layout;
pub mod speed_layout;
pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../../../assets/DepartureMono-Regular.otf"))
            .into(),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    ctx.set_fonts(fonts);
}



pub fn custom_layouter(
    ui: &egui::Ui,
    buf: &dyn egui::TextBuffer,
    wrap_width: f32,
    highlight_values: &[String],
    highlight_color: egui::Color32,
    font_size:f32
) -> std::sync::Arc<egui::Galley> {
    let text = buf.as_str();
    let mut job = egui::text::LayoutJob::default();
    let font_id = egui::FontId::new(font_size, egui::FontFamily::Proportional);
    for line in text.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        for (i, token) in tokens.iter().enumerate() {
            let mut color = egui::Color32::WHITE;
            if i == tokens.len() - 1 && highlight_values.iter().any(|v| v == token) {
                color = highlight_color;
            }
            job.append(
                &format!("{} ", token),
                0.0,
                egui::text::TextFormat {
                    font_id: font_id.clone(),
                    color,
                    ..Default::default()
                },
            );
        }
        job.append(
            "\n",
            0.0,
            egui::text::TextFormat {
                font_id: font_id.clone(),
                color: egui::Color32::WHITE,
                ..Default::default()
            },
        );
    }
    job.wrap.max_width = wrap_width;
    ui.fonts(|f| f.layout_job(job))
}



#[derive(PartialEq)]
pub struct SpeedAnim {
    pub shown: f32, // 화면에 표시 중인 속도(km/h)
}
impl Default for SpeedAnim {
    fn default() -> Self { Self { shown: 0.0 } }
}
impl SpeedAnim {
    /// target 으로 부드럽게 수렴 (critically damped-ish)
    pub fn update(&mut self, ctx: &Context, target: f32) {
        let dt = ctx.input(|i| i.stable_dt).max(1.0/240.0); // 초
        // 반응속도 조절: k 값이 클수록 빠르게 따라감 (3~8 추천)
        let k = 3.0;
        self.shown += (target - self.shown) * (1.0 - (-k * dt).exp());
        ctx.request_repaint(); // 계속 갱신
    }
}
#[derive(Debug, Clone,PartialEq)]
pub struct AltAnim {
    pub shown: f32,
    pub k: f32,
    inited: bool,
}
impl Default for AltAnim {
    fn default() -> Self { Self { shown: 0.0, k: 8.0, inited: false } }
}
impl AltAnim {
    pub fn update(&mut self, ctx: &Context, target: f32) {
        let dt = ctx.input(|i| i.stable_dt).max(1.0/240.0);
        if !self.inited { self.shown = target; self.inited = true; }
        self.shown += (target - self.shown) * (1.0 - (-self.k * dt).exp());
        ctx.request_repaint();
    }
}