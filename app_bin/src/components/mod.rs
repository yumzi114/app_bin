use eframe::egui;
pub mod layout;
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