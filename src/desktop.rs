use crate::{
    catalog::TOPICS,
    curriculum::{LESSONS, LINKS},
    model,
    practice::{self, Exercise},
};
use eframe::egui::{self, Align2, Color32, FontId, Rect, RichText, Stroke, Vec2};
use std::time::{Duration, Instant};
const BG: Color32 = Color32::from_rgb(0, 0, 0);
const PANEL: Color32 = Color32::from_rgb(28, 28, 28);
const BORDER: Color32 = Color32::from_rgb(61, 61, 61);
const MID: Color32 = Color32::from_rgb(125, 125, 125);
const SILVER: Color32 = Color32::from_rgb(176, 176, 176);
const INK: Color32 = Color32::from_rgb(242, 242, 242);
#[derive(PartialEq)]
enum View {
    Roadmap,
    Lesson,
    Structures,
    Practice,
}
pub struct Lab {
    view: View,
    lesson: usize,
    structure: usize,
    case: usize,
    structure_questions: bool,
    exercise: Exercise,
    step: usize,
    play: bool,
    last: Instant,
    speed: f32,
    reveal: bool,
    data: String,
    value: String,
    index: usize,
    items: Vec<String>,
    diagram: model::Diagram,
    message: String,
    zoom: f32,
    done: [bool; 18],
}
impl Lab {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::with_context(&cc.egui_ctx)
    }
    fn with_context(ctx: &egui::Context) -> Self {
        let mut v = egui::Visuals::dark();
        v.panel_fill = PANEL;
        v.window_fill = PANEL;
        v.extreme_bg_color = BG;
        v.faint_bg_color = PANEL;
        v.override_text_color = Some(INK);
        v.selection.bg_fill = BORDER;
        v.selection.stroke = Stroke::new(1.0_f32, INK);
        for w in [
            &mut v.widgets.noninteractive,
            &mut v.widgets.inactive,
            &mut v.widgets.hovered,
            &mut v.widgets.active,
            &mut v.widgets.open,
        ] {
            w.bg_fill = PANEL;
            w.weak_bg_fill = BORDER;
            w.bg_stroke = Stroke::new(1.0_f32, MID);
            w.fg_stroke = Stroke::new(1.0_f32, INK);
        }
        ctx.set_visuals(v);
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = Vec2::new(10.0, 10.0);
        style.spacing.button_padding = Vec2::new(12.0, 7.0);
        ctx.set_style(style);
        let items = model::tokens(TOPICS[0].example);
        let diagram = model::build(TOPICS[0].id, &items);
        Self {
            view: View::Roadmap,
            lesson: 0,
            structure: 0,
            case: 0,
            structure_questions: false,
            exercise: practice::exercise(0, 0),
            step: 0,
            play: false,
            last: Instant::now(),
            speed: 1.5,
            reveal: false,
            data: TOPICS[0].example.into(),
            value: "42".into(),
            index: 0,
            items,
            diagram,
            message: String::new(),
            zoom: 0.85,
            done: [false; 18],
        }
    }
    fn load_exercise(&mut self) {
        self.exercise = if self.structure_questions {
            practice::structure_exercise(self.structure, self.case)
        } else {
            practice::exercise(self.lesson, self.case)
        };
        self.step = 0;
        self.play = false;
        self.reveal = false;
    }
    fn select_structure(&mut self, i: usize) {
        self.structure = i;
        self.data = TOPICS[i].example.into();
        self.items = model::tokens(&self.data);
        self.step = self.items.len();
        self.diagram = model::build(TOPICS[i].id, &self.items);
        self.message.clear();
        self.play = false;
        self.index = self.items.len();
        self.view = View::Structures;
    }
    fn rebuild(&mut self) {
        self.diagram = model::build(
            TOPICS[self.structure].id,
            &self.items[..self.step.min(self.items.len())],
        );
    }
    fn action(&mut self, op: &str) {
        match model::mutate(
            TOPICS[self.structure].id,
            &self.items[..self.step.min(self.items.len())],
            op,
            &self.value,
            self.index,
        ) {
            Ok((items, msg)) => {
                self.items = items;
                self.data = self.items.join(",");
                self.step = self.items.len();
                self.index = self.items.len();
                self.message = msg;
                self.rebuild();
            }
            Err(e) => self.message = e,
        };
    }
    fn roadmap(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("YOUR LEARNING PATH").color(SILVER).size(12.0));
        ui.heading(RichText::new("Build understanding, one connection at a time.").size(30.0));
        ui.label(
            "Follow the prerequisite edges. Open a topic for its lesson and 10 worked exercises.",
        );
        ui.horizontal(|ui| {
            ui.label(format!(
                "{} / 18 topics marked complete",
                self.done.iter().filter(|v| **v).count()
            ));
            ui.label("Progress lasts for this session.");
            ui.add(egui::Slider::new(&mut self.zoom, 0.55..=1.2).text("Map zoom"));
        });
        egui::ScrollArea::both().id_salt("roadmap").show(ui, |ui| {
            let size = Vec2::new(1250.0, 1030.0) * self.zoom;
            let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
            let pos: Vec<_> = LESSONS
                .iter()
                .map(|l| rect.min + Vec2::new(560.0 + l.x * 140.0, 55.0 + l.y * 105.0) * self.zoom)
                .collect();
            let p = ui.painter();
            p.rect_filled(rect, 12.0, BG);
            for &(a, b) in LINKS {
                let from = pos[a] + Vec2::new(0.0, 23.0 * self.zoom);
                let to = pos[b] - Vec2::new(0.0, 23.0 * self.zoom);
                p.line_segment([from, to], Stroke::new(1.8_f32, MID));
                let dir = (to - from).normalized();
                let normal = Vec2::new(-dir.y, dir.x);
                p.line_segment(
                    [to - dir * 8.0 + normal * 4.0, to],
                    Stroke::new(1.8_f32, MID),
                );
                p.line_segment(
                    [to - dir * 8.0 - normal * 4.0, to],
                    Stroke::new(1.8_f32, MID),
                );
            }
            for (i, l) in LESSONS.iter().enumerate() {
                let r = Rect::from_center_size(pos[i], Vec2::new(180.0, 53.0) * self.zoom);
                let response = ui.interact(r, ui.id().with(i), egui::Sense::click());
                let fill = if response.hovered() || self.done[i] {
                    BORDER
                } else {
                    PANEL
                };
                ui.painter().rect_filled(r, 8.0, fill);
                ui.painter().rect_stroke(
                    r,
                    8.0,
                    Stroke::new(
                        if self.done[i] { 2.0_f32 } else { 1.0_f32 },
                        if self.done[i] { INK } else { MID },
                    ),
                    egui::StrokeKind::Inside,
                );
                let title = l
                    .title
                    .replace("Dynamic Programming", "DP")
                    .replace("Heap / Priority Queue", "Heap / Priority queue");
                ui.painter().text(
                    r.center(),
                    Align2::CENTER_CENTER,
                    title,
                    FontId::proportional(13.0 * self.zoom.max(0.8)),
                    INK,
                );
                if response.clicked() {
                    self.lesson = i;
                    self.view = View::Lesson;
                    self.structure_questions = false;
                    self.load_exercise();
                }
            }
        });
        ui.hyperlink_to(
            "Roadmap inspiration: NeetCode · original lessons and exercises",
            "https://neetcode.io/roadmap",
        );
    }
    fn lesson(&mut self, ui: &mut egui::Ui) {
        let l = &LESSONS[self.lesson];
        ui.heading(RichText::new(l.title).size(32.0));
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.done[self.lesson], "Mark understood");
            if ui.button("Practice this topic →").clicked() {
                self.view = View::Practice;
                self.structure_questions = false;
                self.load_exercise();
            }
        });
        for (title, body) in [
            ("Why it exists", l.why),
            ("How it works", l.how),
            ("When to use it", l.when),
            ("Industry examples", l.industry),
            ("Origins and historical context", l.history),
        ] {
            egui::Frame::new()
                .fill(PANEL)
                .stroke(Stroke::new(1.0_f32, BORDER))
                .inner_margin(18.0)
                .corner_radius(10.0)
                .show(ui, |ui| {
                    ui.heading(title);
                    ui.label(body);
                });
        }
        ui.hyperlink_to("Read the reference", l.source);
        ui.separator();
        ui.heading("Explore its structures");
        ui.horizontal_wrapped(|ui| {
            for id in l.structures.split(',') {
                if let Some(i) = TOPICS.iter().position(|t| t.id == id) {
                    if ui.button(TOPICS[i].title).clicked() {
                        self.select_structure(i);
                    }
                }
            }
        });
        ui.label("A useful drill: predict one operation, take one step, and explain which rule still holds. Then change an input and repeat.");
    }
    fn controls(&mut self, ui: &mut egui::Ui, max: usize) -> bool {
        let old = self.step;
        ui.horizontal_wrapped(|ui| {
            if ui.button("Restart").clicked() {
                self.step = 0;
                self.play = false;
            }
            if ui
                .add_enabled(self.step > 0, egui::Button::new("← Previous"))
                .clicked()
            {
                self.step -= 1;
                self.play = false;
            }
            if ui
                .button(if self.play { "Pause" } else { "Play" })
                .clicked()
            {
                if self.step >= max {
                    self.step = 0;
                }
                self.play = !self.play;
                self.last = Instant::now();
            }
            if ui
                .add_enabled(self.step < max, egui::Button::new("Next →"))
                .clicked()
            {
                self.step += 1;
                self.play = false;
            }
            ui.label(format!("{} / {max}", self.step));
            ui.add(egui::Slider::new(&mut self.speed, 0.5..=5.0).text("steps/sec"));
        });
        old != self.step
    }
    fn structures(&mut self, ui: &mut egui::Ui) {
        let t = &TOPICS[self.structure];
        if ui
            .button("10 worked questions for this structure →")
            .clicked()
        {
            self.view = View::Practice;
            self.structure_questions = true;
            self.case = 0;
            self.load_exercise();
        }
        ui.heading(RichText::new(t.title).size(30.0));
        let background = &crate::background::BACKGROUNDS[self.structure];
        egui::CollapsingHeader::new("Start here · why, how, when, industry, history")
            .default_open(true)
            .show(ui, |ui| {
                for (heading, body) in [
                    ("Why", background.why),
                    ("How", t.about),
                    ("When", background.when),
                    ("Industry example", background.industry),
                    ("Origins", background.history),
                ] {
                    ui.strong(heading);
                    ui.label(body);
                }
                ui.hyperlink_to("Reference", background.source);
            });
        ui.label(RichText::new(t.invariant).strong());
        ui.label(t.cost);
        ui.horizontal(|ui| {
            ui.label("Input:");
            ui.add(egui::TextEdit::singleline(&mut self.data).desired_width(450.0));
            if ui.button("Build").clicked() {
                let items = model::tokens(&self.data);
                match model::validate(t.id, &items) {
                    Ok(()) => {
                        self.items = items;
                        self.step = self.items.len();
                        self.index = self.items.len();
                        self.message = "Built input. Restart to replay each insertion.".into();
                        self.rebuild();
                    }
                    Err(e) => self.message = e,
                }
            }
            if ui.button("Example").clicked() {
                self.select_structure(self.structure);
            }
        });
        if self.controls(ui, self.items.len()) {
            self.rebuild();
        }
        ui.label("Each frame is the state after a complete insertion. Use the practice view for algorithm-level traces.");
        egui::ScrollArea::both()
            .id_salt("structure_canvas")
            .max_height(420.0)
            .show(ui, |ui| {
                draw_diagram(ui, &self.diagram);
            });
        ui.horizontal_wrapped(|ui| {
            ui.label("Value/key:");
            ui.add(egui::TextEdit::singleline(&mut self.value).desired_width(100.0));
            ui.label("Index:");
            ui.add(egui::DragValue::new(&mut self.index).range(0..=model::MAX_ITEMS));
            if ui.button("Insert / push").clicked() {
                self.action("insert");
            }
            if ui.button("Remove / pop").clicked() {
                self.action("remove");
            }
            if matches!(t.id, "array" | "segment" | "fenwick") && ui.button("Set index").clicked() {
                self.action("set");
            }
            if t.id == "deque" {
                if ui.button("Push front").clicked() {
                    self.action("front");
                }
                if ui.button("Pop front").clicked() {
                    self.action("popfront");
                }
            }
        });
        ui.label(&self.message);
        egui::CollapsingHeader::new("Python reference example").show(ui, |ui| {
            ui.label("Reference text only; no Python interpreter is used.");
            ui.add(
                egui::TextEdit::multiline(&mut t.python.to_string())
                    .font(egui::TextStyle::Monospace)
                    .desired_rows(12)
                    .desired_width(f32::INFINITY)
                    .interactive(false),
            );
        });
        egui::CollapsingHeader::new("Inspect values and roles").show(ui, |ui| {
            for n in &self.diagram.nodes {
                ui.label(format!("{} — {}", n.label, n.note));
            }
            for (k, v) in &self.diagram.rows {
                ui.label(format!("{k}: {v}"));
            }
        });
    }
    fn practice(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            RichText::new(format!(
                "{} · Practice",
                (if self.structure_questions {
                    TOPICS[self.structure].title
                } else {
                    LESSONS[self.lesson].title
                })
            ))
            .size(28.0),
        );
        let before = self.case;
        ui.horizontal_wrapped(|ui| {
            for i in 0..10 {
                ui.selectable_value(
                    &mut self.case,
                    i,
                    format!(
                        "{} · {}",
                        i + 1,
                        if i < 3 {
                            "E"
                        } else if i < 7 {
                            "M"
                        } else {
                            "H"
                        }
                    ),
                );
            }
        });
        if before != self.case {
            self.load_exercise();
        }
        ui.strong(format!(
            "{} / {}",
            self.exercise.difficulty, self.exercise.title
        ));
        ui.label(&self.exercise.prompt);
        ui.label("Difficulty is relative to this learning topic. Hard cases also require a correctness argument and complexity analysis.");
        if !self.reveal {
            if ui.button("Reveal worked solution").clicked() {
                self.reveal = true;
            }
            return;
        }
        ui.separator();
        ui.label(&self.exercise.explanation);
        ui.label(RichText::new(self.exercise.cost).color(SILVER));
        self.controls(ui, self.exercise.frames.len() - 1);
        let f = &self.exercise.frames[self.step.min(self.exercise.frames.len() - 1)];
        ui.label(RichText::new(&f.note).strong());
        egui::ScrollArea::both()
            .id_salt("trace_canvas")
            .max_height(420.0)
            .show(ui, |ui| {
                if let Some(diagram) = &f.diagram {
                    draw_diagram(ui, diagram);
                    return;
                }
                let cols = f.columns;
                let rows = f.cells.len().div_ceil(cols).max(1);
                let (rect, _) = ui.allocate_exact_size(
                    Vec2::new(cols as f32 * 135.0 + 20.0, rows as f32 * 94.0 + 30.0),
                    egui::Sense::hover(),
                );
                let p = ui.painter();
                p.rect_filled(rect, 10.0, BG);
                let positions: Vec<_> = (0..f.cells.len())
                    .map(|i| {
                        rect.min
                            + Vec2::new(
                                75.0 + (i % cols) as f32 * 135.0,
                                45.0 + (i / cols) as f32 * 94.0,
                            )
                    })
                    .collect();
                for &(a, b) in &f.edges {
                    if a < positions.len() && b < positions.len() {
                        p.line_segment([positions[a], positions[b]], Stroke::new(1.5_f32, MID));
                    }
                }
                for (i, c) in f.cells.iter().enumerate() {
                    let r = Rect::from_center_size(positions[i], Vec2::new(120.0, 50.0));
                    let active = f.active.contains(&i);
                    p.rect_filled(r, 8.0, if active { SILVER } else { PANEL });
                    p.rect_stroke(r, 8.0, Stroke::new(1.0_f32, MID), egui::StrokeKind::Inside);
                    p.text(
                        r.center(),
                        Align2::CENTER_CENTER,
                        c,
                        FontId::monospace(12.0),
                        if active { BG } else { INK },
                    );
                    p.text(
                        r.center_bottom() + Vec2::new(0.0, 5.0),
                        Align2::CENTER_TOP,
                        format!("index {i}"),
                        FontId::monospace(10.0),
                        SILVER,
                    );
                }
                if f.cells.is_empty() {
                    p.text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        "Empty state",
                        FontId::proportional(18.0),
                        SILVER,
                    );
                }
            });
        ui.separator();
        ui.strong("Final answer");
        ui.label(&self.exercise.answer);
    }
}
impl Lab {
    fn show(&mut self, ctx: &egui::Context) {
        if self.play && matches!(self.view, View::Practice | View::Structures) {
            let max = if self.view == View::Practice {
                self.exercise.frames.len() - 1
            } else {
                self.items.len()
            };
            let delay = Duration::from_secs_f32(1.0 / self.speed);
            if self.last.elapsed() >= delay {
                if self.step < max {
                    self.step += 1;
                    if self.view == View::Structures {
                        self.rebuild();
                    }
                } else {
                    self.play = false;
                }
                self.last = Instant::now();
            }
            if self.play {
                ctx.request_repaint_after(delay.saturating_sub(self.last.elapsed()));
            }
        }
        egui::SidePanel::left("navigation")
            .exact_width(230.0)
            .show(ctx, |ui| {
                ui.add_space(14.0);
                ui.heading("RUST DS LAB");
                ui.label(
                    RichText::new("BLACK KNIGHT / DESKTOP")
                        .size(10.0)
                        .color(SILVER),
                );
                ui.add_space(18.0);
                if ui
                    .selectable_label(self.view == View::Roadmap, "◈  Learning roadmap")
                    .clicked()
                {
                    self.view = View::Roadmap;
                    self.play = false;
                }
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.strong("18 ROADMAP TOPICS");
                    for (i, l) in LESSONS.iter().enumerate() {
                        if ui
                            .selectable_label(
                                self.lesson == i
                                    && matches!(self.view, View::Lesson | View::Practice),
                                l.title,
                            )
                            .clicked()
                        {
                            self.lesson = i;
                            self.view = View::Lesson;
                            self.structure_questions = false;
                            self.load_exercise();
                        }
                    }
                    ui.separator();
                    ui.strong("16 STRUCTURE LABS");
                    for (i, t) in TOPICS.iter().enumerate() {
                        if ui
                            .selectable_label(
                                self.structure == i && self.view == View::Structures,
                                t.title,
                            )
                            .clicked()
                        {
                            self.select_structure(i);
                        }
                    }
                    ui.separator();
                    ui.label(
                        RichText::new(
                            "Offline · native Rust\n180 roadmap + 160 structure exercises",
                        )
                        .color(SILVER)
                        .size(11.0),
                    );
                });
            });
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG).inner_margin(24.0))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("main_content")
                    .show(ui, |ui| match self.view {
                        View::Roadmap => self.roadmap(ui),
                        View::Lesson => self.lesson(ui),
                        View::Structures => self.structures(ui),
                        View::Practice => self.practice(ui),
                    });
            });
    }
}
fn draw_diagram(ui: &mut egui::Ui, d: &model::Diagram) {
    let scale = 0.8;
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(d.width as f32 * scale, d.height as f32 * scale),
        egui::Sense::hover(),
    );
    let p = ui.painter();
    p.rect_filled(rect, 10.0, PANEL);
    let point = |i: usize| rect.min + Vec2::new(d.nodes[i].x as f32, d.nodes[i].y as f32) * scale;
    for e in &d.edges {
        let a = point(e.from);
        let b = point(e.to);
        if e.from == e.to {
            p.circle_stroke(a + Vec2::new(0.0, 26.0), 35.0, Stroke::new(1.5_f32, MID));
            continue;
        }
        let dir = (b - a).normalized();
        let end = b - dir * 27.0;
        p.line_segment([a, end], Stroke::new(1.5_f32, MID));
        if e.directed {
            let normal = Vec2::new(-dir.y, dir.x);
            for sign in [-1.0, 1.0] {
                p.line_segment(
                    [end - dir * 8.0 + normal * 4.0 * sign, end],
                    Stroke::new(1.5_f32, SILVER),
                );
                if e.both {
                    let start = a + dir * 27.0;
                    p.line_segment(
                        [start + dir * 8.0 + normal * 4.0 * sign, start],
                        Stroke::new(1.5_f32, SILVER),
                    );
                }
            }
        }
    }
    for (i, n) in d.nodes.iter().enumerate() {
        let center = point(i);
        let r = Rect::from_center_size(center, Vec2::new(92.0, 38.0));
        p.rect_filled(r, 8.0, if n.active { SILVER } else { BORDER });
        p.text(
            center,
            Align2::CENTER_CENTER,
            &n.label,
            FontId::monospace(if n.label.len() > 12 { 9.0 } else { 13.0 }),
            if n.active { BG } else { INK },
        );
        p.text(
            center + Vec2::new(0.0, 25.0),
            Align2::CENTER_TOP,
            &n.note,
            FontId::monospace(10.0),
            SILVER,
        );
    }
    if d.nodes.is_empty() {
        p.text(
            rect.center(),
            Align2::CENTER_CENTER,
            "Empty structure · take the next step",
            FontId::proportional(18.0),
            SILVER,
        );
    }
}

impl eframe::App for Lab {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show(ctx);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_native_views_render_at_small_and_large_sizes() {
        let ctx = egui::Context::default();
        let mut lab = Lab::with_context(&ctx);
        for size in [[800.0, 600.0], [1280.0, 860.0]] {
            let draw = |lab: &mut Lab| {
                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, Vec2::from(size))),
                        ..Default::default()
                    },
                    |ctx| lab.show(ctx),
                );
                assert!(!ctx
                    .tessellate(output.shapes, output.pixels_per_point)
                    .is_empty());
            };
            lab.view = View::Roadmap;
            draw(&mut lab);
            for i in 0..LESSONS.len() {
                lab.lesson = i;
                lab.view = View::Lesson;
                draw(&mut lab);
                lab.structure_questions = false;
                lab.load_exercise();
                lab.view = View::Practice;
                lab.reveal = true;
                draw(&mut lab);
                lab.step = lab.exercise.frames.len() - 1;
                draw(&mut lab);
            }
            for i in 0..TOPICS.len() {
                lab.select_structure(i);
                draw(&mut lab);
                lab.structure_questions = true;
                lab.case = 9;
                lab.load_exercise();
                lab.view = View::Practice;
                lab.reveal = true;
                draw(&mut lab);
                lab.step = lab.exercise.frames.len() - 1;
                draw(&mut lab);
            }
        }
    }
}
