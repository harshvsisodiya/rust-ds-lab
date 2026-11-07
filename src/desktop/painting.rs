use super::*;
pub(super) fn topic_title(i: usize) -> &'static str {
    if i < 18 {
        LESSONS[i].title
    } else if i < 34 {
        TOPICS[i - 18].title
    } else {
        crate::extras::TOPICS[i - 34].title
    }
}
pub(super) fn page_title(ui: &mut egui::Ui, category: &str, title: &str, description: &str) {
    eyebrow(ui, category);
    ui.label(crate::theme::title(title, 32.0));
    muted(ui, description);
    ui.add_space(8.0);
}
pub(super) fn small_card(ui: &mut egui::Ui, number: &str, title: &str, body: &str, color: Color32) {
    card().show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.set_min_height(135.0);
        badge(ui, number, color);
        ui.label(RichText::new(title).size(20.0).strong().color(TAUPE));
        muted(ui, body);
    });
}
pub(super) fn mini_array(ui: &mut egui::Ui, values: &[&str], active: Option<usize>, caption: &str) {
    let width = ui.available_width();
    let cell = ((width - 20.0) / values.len() as f32).min(82.0);
    let (r, _) = ui.allocate_exact_size(Vec2::new(width, 115.0), egui::Sense::hover());
    let start = r.center().x - cell * values.len() as f32 / 2.0;
    let p = ui.painter();
    for (i, v) in values.iter().enumerate() {
        let rect = Rect::from_min_size(
            Pos2::new(start + i as f32 * cell, r.top() + 10.0),
            Vec2::new(cell - 8.0, 49.0),
        );
        p.rect_filled(rect.translate(Vec2::new(0.0, 3.0)), 11.0, LINE);
        p.rect_filled(rect, 11.0, if active == Some(i) { BLUE } else { PAPER });
        p.text(
            rect.center(),
            Align2::CENTER_CENTER,
            *v,
            FontId::proportional(if v.len() > 3 { 14.0 } else { 20.0 }),
            TAUPE,
        );
        p.text(
            rect.center_bottom() + Vec2::new(0.0, 6.0),
            Align2::CENTER_TOP,
            format!("{i}"),
            FontId::monospace(11.0),
            MUTED,
        );
    }
    p.text(
        Pos2::new(r.center().x, r.bottom() - 7.0),
        Align2::CENTER_BOTTOM,
        caption,
        FontId::proportional(12.0),
        TAUPE,
    );
}
pub(super) fn legend(ui: &mut egui::Ui) {
    ui.horizontal_wrapped(|ui| {
        for (color, text) in [
            (BLUE, "Active / just changed"),
            (SOFT_GREEN, "Other values"),
            (BEIGE, "Relationships & roles"),
        ] {
            let (r, _) = ui.allocate_exact_size(Vec2::splat(10.0), egui::Sense::hover());
            ui.painter().circle_filled(r.center(), 5.0, color);
            ui.label(RichText::new(text).size(11.0).color(MUTED));
        }
    });
}
pub(super) fn frame_diagram(f: &practice::Frame) -> model::Diagram {
    if let Some(d) = &f.diagram {
        return d.clone();
    }
    let columns = f.columns.max(1);
    model::Diagram {
        nodes: f
            .cells
            .iter()
            .enumerate()
            .map(|(i, c)| model::Node {
                label: c.clone(),
                note: format!("index {i}"),
                x: 70.0 + (i % columns) as f64 * 126.0,
                y: 60.0 + (i / columns) as f64 * 106.0,
                active: f.active.contains(&i),
            })
            .collect(),
        edges: f
            .edges
            .iter()
            .map(|&(from, to)| model::Edge {
                from,
                to,
                directed: true,
                both: false,
            })
            .collect(),
        rows: vec![],
        width: columns as f64 * 126.0 + 30.0,
        height: f.cells.len().div_ceil(columns).max(1) as f64 * 106.0 + 35.0,
    }
}
pub(super) fn thumbnail(ui: &mut egui::Ui, d: &model::Diagram, id: &str) {
    egui::ScrollArea::horizontal().id_salt(id).show(ui, |ui| {
        draw_diagram(ui, d, None, 1.0, 190.0);
    });
}
pub(super) fn draw_diagram(
    ui: &mut egui::Ui,
    d: &model::Diagram,
    old: Option<&model::Diagram>,
    t: f32,
    min_height: f32,
) {
    if !d.nodes.is_empty() && d.nodes.iter().all(|n| n.note.starts_with("bar")) {
        draw_bars(ui, d, old, t);
        return;
    }
    let width = ui.available_width().max(250.0);
    let max_x = d.nodes.iter().map(|n| n.x as f32).fold(0.0, f32::max) + 75.0;
    let max_y = d.nodes.iter().map(|n| n.y as f32).fold(0.0, f32::max) + 78.0;
    let scale = ((width - 20.0) / max_x.max(1.0)).clamp(0.75, 1.15);
    let size = Vec2::new(
        width.max(max_x * scale + 20.0),
        (max_y * scale + 24.0).max(min_height),
    );
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
    let visible = rect.intersect(ui.clip_rect());
    let p = ui.painter().with_clip_rect(visible);
    p.rect_filled(rect, 16.0, CREAM);
    // A subtle sparse grid only covers the visible viewport.
    let mut y = rect.top() + ((visible.top() - rect.top()) / 40.0).floor() * 40.0 + 20.0;
    while y < visible.bottom() {
        let mut x = rect.left() + ((visible.left() - rect.left()) / 40.0).floor() * 40.0 + 20.0;
        while x < visible.right() {
            p.circle_filled(Pos2::new(x, y), 0.6, LINE);
            x += 40.0;
        }
        y += 40.0;
    }
    let base = rect.min
        + Vec2::new(
            (size.x - max_x * scale) * 0.5,
            (size.y - max_y * scale) * 0.5,
        );
    let positions: Vec<_> = d
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let target = Vec2::new(n.x as f32, n.y as f32);
            let before = old
                .and_then(|prev| {
                    let occurrence = d.nodes[..i].iter().filter(|v| v.label == n.label).count();
                    prev.nodes
                        .iter()
                        .filter(|v| v.label == n.label)
                        .nth(occurrence)
                })
                .map(|n| Vec2::new(n.x as f32, n.y as f32))
                .unwrap_or(target + Vec2::new(0.0, 18.0));
            base + (before + (target - before) * t) * scale
        })
        .collect();
    for e in &d.edges {
        if e.from >= positions.len() || e.to >= positions.len() {
            continue;
        }
        let a = positions[e.from];
        let b = positions[e.to];
        if e.from == e.to {
            p.circle_stroke(a + Vec2::new(0.0, 25.0), 35.0, Stroke::new(2.0_f32, BEIGE));
            continue;
        }
        let dir = (b - a).normalized();
        let normal = Vec2::new(-dir.y, dir.x);
        let radius = if dir.x.abs() > 0.75 {
            48.0 * scale
        } else {
            24.0 * scale
        };
        let start = a + dir * radius;
        let end = b - dir * radius;
        p.line_segment([start, end], Stroke::new(2.0_f32, BEIGE));
        if e.directed {
            for sign in [-1.0, 1.0] {
                p.line_segment(
                    [end - dir * 8.0 + normal * 4.0 * sign, end],
                    Stroke::new(2.0_f32, TAUPE),
                );
                if e.both {
                    p.line_segment(
                        [start + dir * 8.0 + normal * 4.0 * sign, start],
                        Stroke::new(2.0_f32, TAUPE),
                    );
                }
            }
        }
    }
    for (i, n) in d.nodes.iter().enumerate() {
        let center = positions[i];
        if !visible.intersects(Rect::from_center_size(
            center,
            Vec2::new(130.0, 110.0) * scale,
        )) {
            continue;
        }
        let node_width = if n.note.starts_with("text ") || n.note.starts_with("pattern ") {
            50.0
        } else if n.note == "leaf" || n.note == "routing keys" {
            150.0
        } else {
            96.0
        };
        let r = Rect::from_center_size(center, Vec2::new(node_width, 48.0) * scale);
        p.rect_filled(r.translate(Vec2::new(0.0, 3.0)), 11.0, LINE);
        let previous = old.and_then(|d| d.nodes.iter().find(|v| v.label == n.label));
        let from = if previous.is_some_and(|n| n.active) {
            BLUE
        } else {
            SOFT_GREEN
        };
        let to = if n.note == "red node" {
            ERROR
        } else if n.note == "black node" {
            TAUPE
        } else if n.active {
            BLUE
        } else {
            SOFT_GREEN
        };
        let mix = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t) as u8;
        p.rect_filled(
            r,
            11.0,
            Color32::from_rgb(
                mix(from.r(), to.r()),
                mix(from.g(), to.g()),
                mix(from.b(), to.b()),
            ),
        );
        if n.active {
            p.rect_stroke(
                r,
                11.0,
                Stroke::new(1.5_f32, TAUPE),
                egui::StrokeKind::Inside,
            );
        }
        let font = FontId::monospace(
            if n.label.len() > 12 {
                10.0
            } else if n.label.len() > 7 {
                12.0
            } else {
                18.0
            } * scale,
        );
        p.text(
            center,
            Align2::CENTER_CENTER,
            &n.label,
            font,
            if n.note == "black node" { PAPER } else { TAUPE },
        );
        let note = p.layout(
            n.note.clone(),
            FontId::proportional(11.0 * scale),
            MUTED,
            108.0 * scale,
        );
        p.galley(
            center + Vec2::new(-note.size().x / 2.0, 30.0 * scale),
            note,
            MUTED,
        );
    }
    if d.nodes.is_empty() {
        p.text(
            rect.center() - Vec2::new(0.0, 14.0),
            Align2::CENTER_CENTER,
            "A little room for possibility.",
            FontId::proportional(22.0),
            TAUPE,
        );
        p.text(
            rect.center() + Vec2::new(0.0, 20.0),
            Align2::CENTER_CENTER,
            "Empty state · press Next or insert a value",
            FontId::proportional(13.0),
            MUTED,
        );
    }
}
pub(super) fn complexity(ui: &mut egui::Ui, cost: &str) {
    card().show(ui,
|ui| {
  ui.set_width(ui.available_width());
eyebrow(ui,
"HOW THE WORK GROWS");
ui.heading("Time & space, in plain terms.");
ui.label(cost);
muted(ui,
"n is the amount of input. O(1) stays roughly constant; O(log n) grows slowly; O(n) visits a proportional \
                        amount of data. Space measures extra memory.");

  let (r,
_)=ui.allocate_exact_size(Vec2::new(ui.available_width(),
200.0),
egui::Sense::hover());
let plot=r.shrink2(Vec2::new(35.0,
30.0));
let p=ui.painter();
p.line_segment([plot.left_top(),
plot.left_bottom()],
Stroke::new(1.0_f32,
GRAY));
p.line_segment([plot.left_bottom(),
plot.right_bottom()],
Stroke::new(1.0_f32,
GRAY));

  for (kind,
color) in [(0,
SAGE),
(1,
TAUPE),
(2,
MUTED)] {
let points:Vec<_>=(0..50).map(|i| {
let x=i as f32/49.0;
let y=match kind {
0=>0.12,
1=>(1.0+x*15.0).log2()/5.0,
_=>x*0.95}
;
Pos2::new(plot.left()+x*plot.width(),
plot.bottom()-y*plot.height())}
).collect();
p.add(egui::Shape::line(points,
Stroke::new(2.5_f32,
color)));
}

  p.text(r.left_top()+Vec2::new(0.0,
3.0),
Align2::LEFT_TOP,
"Relative work",
FontId::proportional(11.0),
MUTED);
p.text(r.right_bottom(),
Align2::RIGHT_BOTTOM,
"Input size →",
FontId::proportional(11.0),
MUTED);

  ui.horizontal_wrapped(|ui| {
badge(ui,
"O(1) · constant",
SOFT_GREEN);
badge(ui,
"O(log n) · logarithmic",
BEIGE);
badge(ui,
"O(n) · linear",
BLUE);
}
);
muted(ui,
"Illustrative growth curves, not benchmarks. The operation costs above describe this lesson; the chart compares common growth rates.");

 }
);
}

pub(super) fn error_message(ui: &mut egui::Ui, message: &str) {
    egui::Frame::new()
        .fill(ERROR_FILL)
        .stroke(Stroke::new(1.0_f32, ERROR))
        .corner_radius(10)
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.label(RichText::new(format!("Input error: {message}")).color(ERROR_TEXT));
        });
}

pub(super) fn code_panel(ui: &mut egui::Ui, code: &str, description: &str) {
    muted(ui, description);

    if ui.button("Copy code").clicked() {
        ui.ctx().copy_text(code.to_string());
    }

    // Virtual rows avoid laying out hundreds of off-screen source lines.
    let lines: Vec<_> = code.lines().collect();

    egui::Frame::new()
        .fill(PAPER)
        .stroke(Stroke::new(1.0_f32, LINE))
        .corner_radius(12)
        .inner_margin(12.0)
        .show(ui, |ui| {
            egui::ScrollArea::both()
                .id_salt("code_listing")
                .max_height(560.0)
                .show_rows(ui, 20.0, lines.len(), |ui, rows| {
                    for row in rows {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{:>3}", row + 1))
                                    .monospace()
                                    .color(MUTED),
                            );

                            ui.label(RichText::new(lines[row]).monospace().color(INK));
                        });
                    }
                });
        });
}

pub(super) fn draw_scene(
    ui: &mut egui::Ui,

    scene: &mut Scene,

    diagram: &model::Diagram,

    key: String,

    reduced: bool,
) {
    if scene.key != key {
        scene.old = std::mem::replace(&mut scene.current, diagram.clone());

        scene.key = key;

        scene.transition = Instant::now();
    }

    let t = if reduced {
        1.0
    } else {
        (scene.transition.elapsed().as_secs_f32() / 0.22).min(1.0)
    };

    if t < 1.0 {
        ui.ctx().request_repaint_after(Duration::from_millis(16));
    }

    egui::ScrollArea::horizontal()
        .id_salt("active_canvas")
        .show(ui, |ui| {
            draw_diagram(
                ui,
                &scene.current,
                Some(&scene.old),
                1.0 - (1.0 - t).powi(3),
                270.0,
            );
        });
}

fn draw_bars(ui: &mut egui::Ui, d: &model::Diagram, old: Option<&model::Diagram>, t: f32) {
    let width = ui.available_width().max(280.0);

    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 290.0), egui::Sense::hover());

    if !ui.is_rect_visible(rect) {
        return;
    }

    let p = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));

    p.rect_filled(rect, 12.0, CREAM);

    let values: Vec<f32> = d
        .nodes
        .iter()
        .filter_map(|n| n.label.parse().ok())
        .collect();

    let min = values.iter().copied().fold(f32::INFINITY, f32::min);

    let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);

    let step = (width - 36.0) / d.nodes.len().max(1) as f32;

    let bar_width = (step - 10.0).clamp(15.0, 62.0);

    for (i, n) in d.nodes.iter().enumerate() {
        let value = values[i];
        let h = 45.0 + (value - min) / (max - min).max(1.0) * 150.0;

        let occurrence = d.nodes[..i].iter().filter(|v| v.label == n.label).count();

        let before = old
            .filter(|d| d.nodes.first().is_some_and(|n| n.note.starts_with("bar")))
            .and_then(|d| {
                d.nodes
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.label == n.label)
                    .nth(occurrence)
                    .map(|(i, _)| i)
            })
            .unwrap_or(i);

        let x =
            rect.left() + 18.0 + ((before as f32 + (i as f32 - before as f32) * t) + 0.5) * step;

        let r = Rect::from_min_size(
            Pos2::new(x - bar_width / 2.0, rect.bottom() - 52.0 - h),
            Vec2::new(bar_width, h),
        );

        p.rect_filled(r, 8.0, if n.active { BLUE } else { SAGE });

        p.text(
            r.center_top() - Vec2::new(0.0, 8.0),
            Align2::CENTER_BOTTOM,
            &n.label,
            FontId::monospace(16.0),
            TAUPE,
        );

        p.text(
            Pos2::new(x, rect.bottom() - 41.0),
            Align2::CENTER_TOP,
            i.to_string(),
            FontId::monospace(11.0),
            MUTED,
        );
    }

    p.text(
        rect.center_bottom() - Vec2::new(0.0, 10.0),
        Align2::CENTER_BOTTOM,
        "Relative height · labels show exact values · blue marks the operation",
        FontId::proportional(11.0),
        MUTED,
    );
}
