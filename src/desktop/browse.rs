use super::*;

impl Lab {
    pub(super) fn home(&mut self, ui: &mut egui::Ui) {
        eyebrow(ui, "YOUR QUIET SPACE TO THINK");
        ui.label(title(
            "Learn by seeing.",
            if ui.available_width() > 700.0 {
                42.0
            } else {
                32.0
            },
        ));
        muted(
            ui,
            "Understand data structures by seeing what changes — and why.",
        );
        ui.add_space(12.0);
        let next = (0..18)
            .find(|i| !self.storage.progress.completed(*i))
            .unwrap_or(0);
        egui::Frame::new()
            .fill(SOFT_GREEN)
            .corner_radius(22)
            .inner_margin(26.0)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal_wrapped(|ui| {
                    badge(ui, "START SMALL", SAGE);
                    muted(ui, "One idea. One step at a time.");
                });
                ui.label(title(LESSONS[next].title, 26.0));
                ui.label(LESSONS[next].why);
                let compact = ui.available_width() < 650.0;
                if compact {
                    self.home_actions(ui, next);
                }
                mini_array(
                    ui,
                    &["12", "7", "24", "9", "18"],
                    Some(2),
                    "A position gives every value a home.",
                );
                self.checkpoint_summary(ui, next);
                if !compact {
                    self.home_actions(ui, next);
                }
            });
        if let Some(i) = self.storage.progress.recent {
            ui.add_space(5.0);
            ui.horizontal_wrapped(|ui| {
                badge(ui, "RECENTLY COMPLETED", BLUE);
                ui.strong(topic_title(i));
                if ui.button("Revisit").clicked() {
                    if i < 18 {
                        self.select_lesson(i);
                    } else if i < 34 {
                        self.select_structure(i - 18);
                    } else {
                        self.select_extra(i - 34);
                    }
                }
            });
        }
        ui.add_space(12.0);
        ui.label(
            RichText::new("A little structure for your learning.")
                .size(23.0)
                .strong()
                .color(TAUPE),
        );
        let steps = [
            (
                "01",
                "Understand",
                "Start with why it exists. Connect it to something familiar.",
                BLUE,
            ),
            (
                "02",
                "Experiment",
                "Change an input. Predict the result. Watch one step at a time.",
                BEIGE,
            ),
            (
                "03",
                "Make it stick",
                "Try a question, then explore its visual worked solution.",
                SOFT_GREEN,
            ),
        ];
        if ui.available_width() >= 700.0 {
            ui.columns(3, |cols| {
                for (i, (n, t, b, c)) in steps.iter().enumerate() {
                    small_card(&mut cols[i], n, t, b, *c);
                }
            });
        } else {
            for (n, t, b, c) in steps {
                small_card(ui, n, t, b, c);
            }
        }
    }
    pub(super) fn home_actions(&mut self, ui: &mut egui::Ui, next: usize) {
        ui.horizontal_wrapped(|ui| {
            if primary(ui, "Continue learning  →").clicked() {
                self.select_lesson(next);
            }
            if ui.button("Explore the roadmap").clicked() {
                self.navigate(View::Roadmap);
            }
        });
    }
    pub(super) fn library(&mut self, ui: &mut egui::Ui) {
        page_title(
            ui,
            "THE LIBRARY",
            "Find your next idea.",
            concat!(
                "Choose a guided topic or go straight to a data structure. Your ",
                "checkpoints travel with you."
            ),
        );
        ui.horizontal_wrapped(|ui| {
            ui.selectable_value(&mut self.nav.library_group, 0, "Guided topics · 18");
            ui.selectable_value(&mut self.nav.library_group, 1, "Structure labs · 16");
        });
        if ui
            .selectable_label(
                self.nav.library_group == 2,
                "Algorithms & balanced trees · 13",
            )
            .clicked()
        {
            self.nav.library_group = 2;
        }
        ui.add(
            egui::TextEdit::singleline(&mut self.nav.search)
                .hint_text("Search by name, use or concept…")
                .desired_width(ui.available_width())
                .margin(Vec2::new(14.0, 12.0)),
        );
        let query = self.nav.search.to_lowercase();
        let entries: Vec<_> = if self.nav.library_group == 2 {
            crate::extras::TOPICS
                .iter()
                .enumerate()
                .filter(|(_, t)| {
                    format!("{} {} {}", t.title, t.why, t.how)
                        .to_lowercase()
                        .contains(&query)
                })
                .map(|(i, t)| (i, t.title, t.why, t.group))
                .collect()
        } else if self.nav.library_group == 1 {
            TOPICS
                .iter()
                .enumerate()
                .filter(|(_, t)| {
                    format!("{} {} {}", t.title, t.group, t.about)
                        .to_lowercase()
                        .contains(&query)
                })
                .map(|(i, t)| (i, t.title, t.about, t.group))
                .collect()
        } else {
            LESSONS
                .iter()
                .enumerate()
                .filter(|(_, l)| {
                    format!("{} {} {}", l.title, l.why, l.when)
                        .to_lowercase()
                        .contains(&query)
                })
                .map(|(i, l)| (i, l.title, l.why, "GUIDED LESSON"))
                .collect()
        };
        if entries.is_empty() {
            section(
                ui,
                "NO MATCHES",
                "Try a broader word.",
                "Search for array, tree, queue, lookup or a topic name.",
            );
        }
        let cols = if ui.available_width() >= 650.0 { 2 } else { 1 };
        for row in entries.chunks(cols) {
            ui.columns(cols, |uis| {
                for (col, (i, title, body, group)) in row.iter().enumerate() {
                    card().show(&mut uis[col], |ui| {
                        ui.set_min_height(145.0);
                        ui.set_width(ui.available_width());
                        eyebrow(ui, &group.to_uppercase());
                        ui.label(crate::theme::title(*title, 19.0));
                        ui.label(*body);
                        self.checkpoint_summary(
                            ui,
                            if self.nav.library_group == 2 {
                                34 + *i
                            } else if self.nav.library_group == 1 {
                                18 + *i
                            } else {
                                *i
                            },
                        );
                        if ui.button("Open lesson  →").clicked() {
                            if self.nav.library_group == 2 {
                                self.select_extra(*i);
                            } else if self.nav.library_group == 1 {
                                self.select_structure(*i);
                            } else {
                                self.select_lesson(*i);
                            }
                        }
                    });
                }
            });
        }
    }
    pub(super) fn roadmap(&mut self, ui: &mut egui::Ui) {
        page_title(
            ui,
            "THE BIG PICTURE",
            "Every idea has a connection.",
            "Start at Arrays & Hashing. Follow a line when you feel ready for the next idea.",
        );
        ui.horizontal_wrapped(|ui| {
            badge(
                ui,
                &format!(
                    "{} of 18 finished",
                    (0..18)
                        .filter(|i| self.storage.progress.completed(*i))
                        .count()
                ),
                SOFT_GREEN,
            );
            ui.add(egui::Slider::new(&mut self.nav.zoom, 0.65..=1.3).text("Zoom"));
            if ui.button("Start from the beginning").clicked() {
                self.select_lesson(0);
            }
        });
        muted(
            ui,
            concat!(
                "Each card shows saved checkpoints: Understand · Visualize · ",
                "Practice. Select a card to continue."
            ),
        );
        egui::ScrollArea::both()
            .id_salt("map_canvas")
            .show(ui, |ui| {
                let s = self.nav.zoom;
                let (rect, _) =
                    ui.allocate_exact_size(Vec2::new(1200.0, 1080.0) * s, egui::Sense::hover());
                let p = ui.painter();
                p.rect_filled(rect, 20.0, PAPER);
                let positions: Vec<_> = LESSONS
                    .iter()
                    .map(|l| rect.min + Vec2::new(520.0 + l.x * 143.0, 55.0 + l.y * 118.0) * s)
                    .collect();
                for &(a, b) in LINKS {
                    let from = positions[a] + Vec2::new(0.0, 34.0) * s;
                    let to = positions[b] - Vec2::new(0.0, 34.0) * s;
                    let points = [
                        from,
                        from + Vec2::new(0.0, (to.y - from.y) * 0.55),
                        to - Vec2::new(0.0, (to.y - from.y) * 0.55),
                        to,
                    ];
                    p.add(egui::epaint::CubicBezierShape::from_points_stroke(
                        points,
                        false,
                        Color32::TRANSPARENT,
                        Stroke::new(
                            2.0_f32,
                            if self.storage.progress.completed(a) {
                                SAGE
                            } else {
                                LINE
                            },
                        ),
                    ));
                }
                for (i, l) in LESSONS.iter().enumerate() {
                    let r = Rect::from_center_size(positions[i], Vec2::new(182.0, 68.0) * s);
                    let response = ui.interact(r, ui.id().with(i), egui::Sense::click());
                    let fill = if response.hovered() {
                        BLUE
                    } else if self.storage.progress.completed(i) {
                        SOFT_GREEN
                    } else if i == 0 {
                        BEIGE
                    } else {
                        CREAM
                    };
                    p.rect_filled(r.translate(Vec2::new(0.0, 3.0)), 12.0, LINE);
                    p.rect_filled(r, 12.0, fill);
                    let title = l
                        .title
                        .replace("Heap / Priority Queue", "Heap / Priority queue")
                        .replace("Dynamic Programming", "DP");
                    p.text(
                        r.center() - Vec2::new(0.0, 8.0 * s),
                        Align2::CENTER_CENTER,
                        title,
                        FontId::proportional(13.0 * s),
                        TAUPE,
                    );
                    for stage in 0..3 {
                        p.circle_filled(
                            r.center() + Vec2::new((stage as f32 - 1.0) * 14.0, 17.0) * s,
                            3.5 * s,
                            if self.storage.progress.checkpoints[i] & (1 << stage) != 0 {
                                LEAF
                            } else {
                                GRAY
                            },
                        );
                    }
                    if response.on_hover_text(l.why).clicked() {
                        self.select_lesson(i);
                    }
                }
            });
        ui.hyperlink_to(
            "Roadmap inspired by NeetCode",
            "https://neetcode.io/roadmap",
        );
    }
}
