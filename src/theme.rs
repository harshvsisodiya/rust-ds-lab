use eframe::egui::{self, Color32, FontId, RichText, Stroke, Vec2};
pub const CREAM: Color32 = Color32::from_rgb(247, 243, 236);
pub const PAPER: Color32 = Color32::from_rgb(255, 253, 249);
pub const TAUPE: Color32 = Color32::from_rgb(85, 73, 64);
pub const INK: Color32 = Color32::from_rgb(40, 37, 33);
pub const MUTED: Color32 = Color32::from_rgb(101, 104, 104);
pub const GRAY: Color32 = Color32::from_rgb(197, 198, 199);
pub const BLUE: Color32 = Color32::from_rgb(215, 229, 240);
pub const BEIGE: Color32 = Color32::from_rgb(201, 173, 147);
pub const SAGE: Color32 = Color32::from_rgb(135, 154, 119);
pub const LEAF: Color32 = Color32::from_rgb(61, 82, 53);
pub const LINE: Color32 = Color32::from_rgb(226, 219, 208);
pub const SOFT_GREEN: Color32 = Color32::from_rgb(232, 238, 225);
pub fn apply(ctx: &egui::Context) {
    let mut v = egui::Visuals::light();
    v.panel_fill = CREAM;
    v.window_fill = PAPER;
    v.extreme_bg_color = PAPER;
    v.faint_bg_color = CREAM;
    v.override_text_color = Some(INK);
    v.hyperlink_color = LEAF;
    v.selection.bg_fill = BLUE;
    v.selection.stroke = Stroke::new(1.0_f32, TAUPE);
    v.window_corner_radius = 20.into();
    for w in [
        &mut v.widgets.noninteractive,
        &mut v.widgets.inactive,
        &mut v.widgets.hovered,
        &mut v.widgets.active,
        &mut v.widgets.open,
    ] {
        w.corner_radius = 9.into();
        w.bg_stroke = Stroke::new(1.0_f32, LINE);
        w.fg_stroke = Stroke::new(1.0_f32, TAUPE);
        w.bg_fill = PAPER;
        w.weak_bg_fill = CREAM;
    }
    v.widgets.hovered.weak_bg_fill = BLUE;
    v.widgets.active.weak_bg_fill = SOFT_GREEN;
    ctx.set_visuals(v);
    let mut s = (*ctx.style()).clone();
    s.spacing.item_spacing = Vec2::new(12.0, 12.0);
    s.spacing.button_padding = Vec2::new(16.0, 10.0);
    s.spacing.interact_size.y = 36.0;
    s.spacing.slider_width = 100.0;
    s.animation_time = 0.18;
    for (style, size) in [
        (egui::TextStyle::Body, 15.0),
        (egui::TextStyle::Button, 14.0),
        (egui::TextStyle::Small, 12.0),
        (egui::TextStyle::Heading, 24.0),
    ] {
        s.text_styles.insert(style, FontId::proportional(size));
    }
    ctx.set_style(s);
}
pub fn card() -> egui::Frame {
    egui::Frame::new()
        .fill(PAPER)
        .stroke(Stroke::new(1.0_f32, LINE))
        .corner_radius(18)
        .inner_margin(22.0)
}
pub fn eyebrow(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).size(11.0).strong().color(LEAF));
}
pub fn muted(ui: &mut egui::Ui, text: impl Into<String>) {
    ui.label(RichText::new(text.into()).color(MUTED));
}
pub fn primary(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(text).color(PAPER).strong())
            .fill(TAUPE)
            .stroke(Stroke::NONE)
            .min_size(Vec2::new(0.0, 40.0)),
    )
}
pub fn badge(ui: &mut egui::Ui, text: &str, fill: Color32) {
    egui::Frame::new()
        .fill(fill)
        .corner_radius(20)
        .inner_margin(egui::Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.label(RichText::new(text).size(11.0).strong().color(TAUPE));
        });
}
pub fn section(ui: &mut egui::Ui, number: &str, title: &str, body: &str) {
    card().show(ui, |ui| {
        ui.set_width(ui.available_width());
        eyebrow(ui, number);
        ui.label(RichText::new(title).size(22.0).strong().color(TAUPE));
        ui.label(body);
    });
}
