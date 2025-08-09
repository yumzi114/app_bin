use std::f32::consts::TAU;

use eframe::{egui::{self, pos2, vec2, Align2, Color32, CornerRadius, FontId, Painter, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2}, epaint::{PathShape, RectShape}};






pub fn speed_donut(
    ui: &mut Ui,
    speed_kmh: f32,    // 현재 속도(km/h)
    max_kmh: f32,      // 게이지 최대치 (예: 180)
    size: f32,         // 정사각 크기 (px)
    color_active: Color32, // 진행 링 색
    color_track: Color32,  // 배경 트랙 색
    label: &str,           // 하단 소제목 (예: "SPEED")
) -> Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter_at(rect);

    // 레이아웃 파라미터
    let c = rect.center();
    let radius = size * 0.42;
    let track_w = size * 0.065;     // 트랙 두께
    let active_w = size * 0.075;    // 진행 링 두께

    // 시작각에서 끝 간격
    let gap_rad = 2.09;                 
    let a_start = 150f32.to_radians();   // 시작: 아래쪽 살짝 왼쪽
    let sweep_track = TAU - gap_rad;     // 트랙이 실제로 도는 각도
    let a_end   = a_start + sweep_track; // 트랙 끝각

    // 진행 비율
    let t = (speed_kmh / max_kmh).clamp(0.0, 1.0);

    // 캡(원) 때문에 끝이 튀어나가는 걸 방지하려면, 각도로 살짝 줄이기
    let cap_ang = (active_w * 0.0) / radius; // 바늘 두께/반지름 → 각도
    let a_curr  = (a_start + sweep_track * t).min(a_end - cap_ang);
    
    // 1) 배경 트랙(원호)
    draw_arc(&p, c, radius, a_start, a_end, track_w, color_track, true);

    // 2) 진행 링(원호 + 둥근 캡)
    draw_arc(&p, c, radius, a_start, a_curr, active_w, color_active, true);

    // 3) 중앙 텍스트
    let speed_text = format!("{:.0}", speed_kmh.clamp(0.0, 999.0));
    p.text(
        c,
        Align2::CENTER_CENTER,
        speed_text,
        FontId::monospace(size * 0.26),
        Color32::WHITE,
    );

    // 4) 단위/라벨
    let sub = format!("{}  •  KM/H", label);
    p.text(
        c + vec2(0.0, size * 0.20),
        Align2::CENTER_CENTER,
        sub,
        FontId::proportional(size * 0.07),
        Color32::from_gray(200),
    );

    resp
}

/// a0→a1로 그려주는 원호 (둥근 캡 옵션)
fn draw_arc(
    p: &Painter,
    center: Pos2,
    r: f32,
    a0: f32,
    a1: f32,
    width: f32,
    color: Color32,
    rounded_caps: bool, // 여전히 지원
) {
    let arc_len = (a1 - a0).abs() * r;
    let n = (arc_len / 3.0).clamp(16.0, 128.0) as usize;

    let mut pts = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let a = egui::lerp(a0..=a1, t);
        pts.push(center + vec2(a.cos(), a.sin()) * r);
    }

    // 스트로크만 사용(캡/조인은 기본값). 시각적 캡은 아래서 원으로 보정
    let stroke = Stroke::new(width, color);
    let path = PathShape::line(pts, stroke);
    p.add(path);

    // 끝부분 둥근 캡 보정(원 두 개)
    if rounded_caps {
        p.circle_filled(center + vec2(a0.cos(), a0.sin()) * r, width * 0.5, color);
        p.circle_filled(center + vec2(a1.cos(), a1.sin()) * r, width * 0.5, color);
    }
}



pub fn altitude_bar(
    ui: &mut Ui,
    alt_m: f32,            // AltAnim.shown 추천
    min_alt: f32,
    max_alt: f32,
    size: Vec2,            // 전체 위젯 크기(막대+틱+숫자 포함)
    track: Color32,
    fill: Color32,
    label: &str,           // "ALT"
) -> Response {
    let (rect, resp) = ui.allocate_exact_size(size, Sense::hover());
    let p = ui.painter_at(rect);

    // 도넛과 높이 맞는 상/하 마진
    let pad = 8.0;
    let top_m = size.y * 0.04;
    let bot_m = size.y * 0.20;

    // 컨텐츠 박스
    let content = Rect::from_min_max(
        pos2(rect.left() + pad,  rect.top()  + pad + top_m),
        pos2(rect.right() - pad, rect.bottom() - pad - bot_m),
    );
    if content.width() < 10.0 || content.height() < 10.0 { return resp; }

    // ── 3열 레이아웃: [막대] [틱/라벨] [값 숫자 열]
    let bar_w         = (content.width() * 0.28).clamp(18.0, 40.0);   // 막대 폭
    let tick_col_w    = (content.width() * 0.24).clamp(42.0, 120.0);  // 틱/숫자 열 폭
    let value_col_ratio = 0.48;                                       // 값 열(오른쪽) 폭 비율
    let value_col_w   = (content.width() * value_col_ratio).max(60.0);

    // 좌우 영역 계산
    let bar = Rect::from_min_max(
        pos2(content.left(), content.top()),
        pos2(content.left() + bar_w, content.bottom()),
    );
    let tick_col = Rect::from_min_max(
        pos2(bar.right(), content.top()),
        pos2((bar.right() + tick_col_w).min(content.right()), content.bottom()),
    );
    let value_col = Rect::from_min_max(
        pos2(tick_col.right(), content.top()),
        pos2(content.right(),  content.bottom()),
    );

    // ── 막대 트랙
    let r = ((bar.width() * 0.36).clamp(0.0, 255.0)).round() as u8;
    let rounding = CornerRadius::same(r);
    p.add(RectShape::filled(bar, rounding, track));
    p.rect_stroke(bar, rounding, Stroke::new(1.0, Color32::from_gray(90)), StrokeKind::Inside);

    // ── 채움 (아래→위)
    let denom = (max_alt - min_alt).abs().max(1e-6);
    let t = ((alt_m - min_alt) / denom).max(0.0).min(1.0);
    let h = bar.height() * t;
    if h > 0.5 {
        let fill_rect = Rect::from_min_max(
            pos2(bar.left(),  bar.bottom() - h),
            pos2(bar.right(), bar.bottom()),
        );
        p.add(RectShape::filled(fill_rect, rounding, fill));
    }

    // ── 틱/라벨 (tick_col 안에서만 그리기)
    let tick_font = FontId::monospace((size.y * 0.055).clamp(10.0, 18.0));
    let tick_x0 = tick_col.left() + 6.0;
    let tick_x1 = tick_x0 + 8.0;
    let step = nice_step(((max_alt - min_alt).abs()).max(1.0), 6);
    let mut v = (min_alt / step).ceil() * step;
    if tick_col.width() > 24.0 {
        while v <= max_alt + 1e-3 {
            let tt = ((v - min_alt) / denom).max(0.0).min(1.0);
            let y = egui::lerp(bar.bottom()..=bar.top(), tt);
            p.line_segment([pos2(tick_x0, y), pos2(tick_x1, y)],
                           Stroke::new(1.0, Color32::from_gray(140)));

            let s = format!("{:.0}", v);
            let gal = ui.fonts(|f| f.layout_no_wrap(s, tick_font.clone(), Color32::from_gray(210)));
            // tick_col 안쪽으로 안전 배치
            let lx = (tick_x1 + 4.0).max(tick_col.left()).min(tick_col.right() - gal.size().x);
            let ly = y - gal.size().y * 0.5;
            if lx <= tick_col.right() - gal.size().x {
                p.galley(pos2(lx, ly), gal, Color32::from_gray(210));
            }
            v += step;
        }
    }

    // ── 중앙 값 텍스트 (value_col 안에서만; 절대 막대/틱을 침범하지 않음)
    // 숫자 크게: 도넛과 맞추려면 비율만 조정(기본 0.18)
    let value_font = FontId::monospace((size.y * 0.18).clamp(18.0, 96.0));
    let txt = format!("{:.1} m", alt_m);
    let g = ui.fonts(|f| f.layout_no_wrap(txt, value_font, Color32::WHITE));

    // value_col 내 중앙 정렬 + 안전 max/min
    let vx_des = value_col.center().x - g.size().x * 0.5;
    let vy_des = value_col.center().y - g.size().y * 0.5;
    let vx = vx_des.max(value_col.left()).min(value_col.right()+150. - g.size().x);
    let vy = vy_des.max(value_col.top()).min(value_col.bottom() - g.size().y);
    p.galley(pos2(vx-60., vy), g, Color32::WHITE);

    // ── 상/하 라벨(막대 기준)
    p.text(pos2(bar.center().x, content.top() - 4.0),
           Align2::CENTER_BOTTOM, label,
           FontId::proportional((size.y * 0.07).clamp(12.0, 18.0)),
           Color32::from_gray(210));
    p.text(pos2(bar.center().x, content.bottom() + 6.0),
           Align2::CENTER_TOP, "m",
           FontId::proportional((size.y * 0.06).clamp(10.0, 16.0)),
           Color32::from_gray(170));

    resp
}

/// 보기 좋은 틱 간격
fn nice_step(range: f32, target_bins: usize) -> f32 {
    let raw = (range / target_bins as f32).max(1.0);
    let pow10 = 10f32.powf(raw.log10().floor());
    let m = raw / pow10;
    let step = if m < 1.5 { 1.0 } else if m < 3.0 { 2.0 }
        else if m < 7.0 { 5.0 } else { 10.0 };
    step * pow10
}