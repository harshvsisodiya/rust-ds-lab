use super::*;

impl Lab {
    pub(super) fn lesson(&mut self, ui: &mut egui::Ui) {
        let l = &LESSONS[self.nav.lesson];
        page_title(ui, "GUIDED LESSON", l.title, l.why);
        self.checkpoint_summary(ui, self.nav.lesson);
        ui.horizontal_wrapped(|ui| {
            if ui
                .selectable_value(&mut self.nav.tab, Tab::Understand, "01  Understand")
                .clicked()
            {
                self.playback.play = false;
            }
            ui.selectable_value(&mut self.nav.tab, Tab::Playground, "02  Visual walkthrough");
            ui.selectable_value(&mut self.nav.tab, Tab::Code, "03  Rust code");
            if ui.button("04  Practice  →").clicked() {
                self.start_practice(false);
            }
        });
        if self.nav.tab == Tab::Code {
            code_panel(
                ui,
                crate::reference::LESSONS[self.nav.lesson],
                "Standalone Rust example. The same algorithm powers the visual lesson.",
            );
            return;
        }
        if self.nav.tab == Tab::Playground {
            self.study.reveal = true;
            self.trace_panel(ui);
            self.checkpoint(ui, self.nav.lesson, 1);
            return;
        }
        section(ui, "01 / THE REASON", "Why does it exist?", l.why);
        section(ui, "02 / THE IDEA", "How does it work?", l.how);
        let guide = &crate::guides::GUIDES[self.nav.lesson];
        card().show(ui, |ui| {
            ui.set_width(ui.available_width());
            eyebrow(ui, "A FAMILIAR WAY TO THINK ABOUT IT");
            ui.label(guide.analogy);
            ui.add_space(6.0);
            ui.heading("The steps to follow");
            for (i, step) in guide.steps.iter().enumerate() {
                ui.label(format!("{}. {}", i + 1, step));
            }
        });
        egui::CollapsingHeader::new("Illustrated example: input, middle state, result").show(
            ui,
            |ui| {
                card().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    eyebrow(ui, "03 / SEE THE STORY");
                    ui.heading("From input to understanding.");
                    muted(ui, &self.study.exercise.prompt);
                    let frames = &self.study.exercise.frames;
                    for (n, index) in [0, frames.len() / 2, frames.len() - 1]
                        .into_iter()
                        .enumerate()
                    {
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(format!("{}   {}", n + 1, frames[index].note))
                                .strong()
                                .color(TAUPE),
                        );
                        thumbnail(
                            ui,
                            &self.study.diagrams[index],
                            &format!("lesson-{}-{n}", self.nav.lesson),
                        );
                    }
                    if primary(ui, "Explore every step  →").clicked() {
                        self.nav.tab = Tab::Playground;
                        self.playback.step = 0;
                    }
                });
            },
        );
        egui::CollapsingHeader::new("Applications, industry examples, history & complexity").show(
            ui,
            |ui| {
                section(ui, "04 / APPLY IT", "When should you use it?", l.when);
                section(
                    ui,
                    "05 / IN THE REAL WORLD",
                    "Where does it show up?",
                    l.industry,
                );
                section(
                    ui,
                    "06 / THE BACKSTORY",
                    "Where did the idea begin?",
                    l.history,
                );
                ui.hyperlink_to("Explore the historical / technical reference ↗", l.source);
                complexity(ui, self.study.exercise.cost);
            },
        );
        self.checkpoint(ui, self.nav.lesson, 0);
        card().show(ui, |ui| {
            eyebrow(ui, "CONTINUE EXPLORING");
            ui.heading("Try the structure underneath.");
            ui.horizontal_wrapped(|ui| {
                for id in l.structures.split(',') {
                    if let Some(i) = TOPICS.iter().position(|t| t.id == id) {
                        if ui.button(TOPICS[i].title).clicked() {
                            self.select_structure(i);
                        }
                    }
                }
            });
            if primary(ui, "Test your understanding  →").clicked() {
                self.start_practice(false);
            }
        });
    }
    pub(super) fn controls(&mut self, ui: &mut egui::Ui, max: usize) -> bool {
        let before = self.playback.step;
        ui.spacing_mut().button_padding = Vec2::new(10.0, 7.0);
        ui.horizontal_wrapped(|ui| {
            if primary(
                ui,
                if self.playback.play {
                    "Pause"
                } else {
                    "Play  →"
                },
            )
            .clicked()
            {
                if self.playback.step >= max {
                    self.playback.step = 0;
                }
                self.playback.play = !self.playback.play;
                self.playback.last = Instant::now();
            }
            if ui
                .add_enabled(self.playback.step > 0, egui::Button::new("← Back"))
                .clicked()
            {
                self.playback.step -= 1;
                self.playback.play = false;
            }
            if ui
                .add_enabled(self.playback.step < max, egui::Button::new("Next →"))
                .clicked()
            {
                self.playback.step += 1;
                self.playback.play = false;
            }
            if ui.button("Reset").clicked() {
                self.playback.step = 0;
                self.playback.play = false;
            }
            badge(ui, &format!("Step {} / {max}", self.playback.step), BLUE);
            if ui.available_width() > 220.0 {
                ui.add(
                    egui::Slider::new(&mut self.playback.speed, 0.5..=5.0)
                        .show_value(false)
                        .text("Speed"),
                );
            } else {
                ui.menu_button("Speed", |ui| {
                    ui.add(
                        egui::Slider::new(&mut self.playback.speed, 0.5..=5.0)
                            .fixed_decimals(1)
                            .text("steps/s"),
                    );
                });
            }
        });
        ui.add(
            egui::ProgressBar::new(if max == 0 {
                0.0
            } else {
                self.playback.step as f32 / max as f32
            })
            .fill(SAGE)
            .desired_height(4.0),
        );
        before != self.playback.step
    }
    pub(super) fn structures(&mut self, ui: &mut egui::Ui) {
        let t = &TOPICS[self.nav.structure];
        if self.nav.tab == Tab::Playground {
            ui.label(title(t.title, 28.0));
        } else {
            page_title(
                ui,
                &format!("STRUCTURE LAB / {}", t.group.to_uppercase()),
                t.title,
                t.about,
            );
        }
        self.checkpoint_summary(ui, 18 + self.nav.structure);
        ui.horizontal_wrapped(|ui| {
            for (tab, name) in [
                (Tab::Understand, "01  Understand"),
                (Tab::Playground, "02  Playground"),
                (Tab::Code, "03  Code"),
            ] {
                if ui.selectable_value(&mut self.nav.tab, tab, name).clicked() {
                    self.playback.play = false;
                }
            }
            if ui.button("Practice →").clicked() {
                self.start_practice(true);
            }
        });
        match self.nav.tab {
            Tab::Understand => {
                let b = &crate::background::BACKGROUNDS[self.nav.structure];
                section(ui, "01 / THE REASON", "Why do we need it?", b.why);
                card().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    eyebrow(ui, "02 / THE MENTAL MODEL");
                    ui.heading("See how the pieces fit.");
                    ui.label(t.about);
                    thumbnail(ui, &self.editor.diagram, "structure_intro");
                    ui.strong("The rule that must stay true");
                    ui.label(t.invariant);
                    if primary(ui, "Open the playground  →").clicked() {
                        self.nav.tab = Tab::Playground;
                    }
                });
                egui::CollapsingHeader::new("Applications · history · complexity").show(ui, |ui| {
                    section(
                        ui,
                        "03 / THE RIGHT TOOL",
                        "When should I reach for it?",
                        b.when,
                    );
                    section(ui, "04 / IN PRACTICE", "How industry uses it", b.industry);
                    section(ui, "05 / ORIGINS", "How the idea developed", b.history);
                    ui.hyperlink_to("Read the reference ↗", b.source);
                    complexity(ui, t.cost);
                });
                section(
                    ui,
                    "A SMALL LEARNING CHALLENGE",
                    t.challenge,
                    concat!(
                        "Make a prediction, try it in the playground, and explain the ",
                        "invariant after the operation."
                    ),
                );
                egui::CollapsingHeader::new("Check the reasoning").show(ui, |ui| {
                    ui.label(t.answer);
                });
                self.checkpoint(ui, 18 + self.nav.structure, 0);
            }
            Tab::Code => {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.study.rust_code, true, "Rust");
                    ui.selectable_value(&mut self.study.rust_code, false, "Python");
                });
                let code = if self.study.rust_code {
                    crate::reference::STRUCTURES[self.nav.structure]
                } else {
                    t.python
                };
                code_panel(
                    ui,
                    code,
                    if self.study.rust_code {
                        "Standalone Rust example. Copy it into main.rs and run with Rust."
                    } else {
                        "Python reference text; no Python runtime is used by this app."
                    },
                );
            }
            Tab::Playground => self.playground(ui),
        }
    }
    pub(super) fn playground(&mut self, ui: &mut egui::Ui) {
        let t = &TOPICS[self.nav.structure];
        card().show(ui, |ui| {
            ui.set_width(ui.available_width());
            eyebrow(ui, "LIVE PLAYGROUND");
            draw_scene(
                ui,
                &mut self.scene,
                &self.editor.diagram,
                format!(
                    "structure-{}-{}-{}",
                    self.nav.structure,
                    self.playback.step,
                    self.editor.items.join(",")
                ),
                self.storage.progress.reduced_motion,
            );
            legend(ui);
            ui.label(
                RichText::new(format!(
                    "{} of {} input items inserted",
                    self.playback.step,
                    self.editor.items.len()
                ))
                .strong()
                .color(TAUPE),
            );
            muted(
                ui,
                concat!(
                    "Each step shows a completed insertion. Reset, then press Next, to ",
                    "build gradually."
                ),
            );
        });
        card().show(ui, |ui| {
            ui.set_width(ui.available_width());
            eyebrow(ui, "MAKE IT YOURS");
            ui.heading("Try your own input.");
            muted(
                ui,
                format!(
                    "Up to 16 items, separated by commas. Example: {}",
                    t.example
                ),
            );
            ui.add(
                egui::TextEdit::singleline(&mut self.editor.data)
                    .desired_width(ui.available_width())
                    .margin(Vec2::splat(10.0)),
            );
            ui.horizontal_wrapped(|ui| {
                if primary(ui, "Build input").clicked() {
                    let items = model::tokens(&self.editor.data);
                    match model::validate(t.id, &items) {
                        Ok(()) => {
                            self.editor.items = items;
                            self.playback.step = self.editor.items.len();
                            self.editor.index = self.editor.items.len();
                            self.playback.play = false;
                            self.editor.message =
                                "Built. Reset to replay, or try an operation below.".into();
                            self.rebuild();
                        }
                        Err(e) => {
                            self.editor.message = e;
                            self.editor.error = true;
                        }
                    }
                }
                if ui.button("Restore example").clicked() {
                    self.select_structure(self.nav.structure);
                    self.nav.tab = Tab::Playground;
                }
            });
            ui.separator();
            ui.strong("Change one thing");
            ui.horizontal_wrapped(|ui| {
                ui.label("Value / key");
                ui.add(egui::TextEdit::singleline(&mut self.editor.value).desired_width(110.0));
                if matches!(
                    t.id,
                    "array" | "singly" | "doubly" | "circular" | "fenwick" | "segment"
                ) {
                    ui.label("Index (starts at 0)");
                    ui.add(
                        egui::DragValue::new(&mut self.editor.index).range(0..=model::MAX_ITEMS),
                    );
                }
            });
            ui.horizontal_wrapped(|ui| {
                if ui.button("Insert / push").clicked() {
                    self.action("insert");
                }
                if !matches!(t.id, "bst" | "graph" | "dsu") && ui.button("Remove / pop").clicked() {
                    self.action("remove");
                }
                if matches!(t.id, "array" | "segment" | "fenwick")
                    && ui.button("Replace at index").clicked()
                {
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
            if self.playback.step < self.editor.items.len() {
                muted(
                    ui,
                    "An operation edits the visible prefix and replaces the remaining replay.",
                );
            }
            if !self.editor.message.is_empty() {
                if self.editor.error {
                    error_message(ui, &self.editor.message);
                } else {
                    ui.label(RichText::new(&self.editor.message).strong().color(LEAF));
                }
            }
        });
        section(ui, "KEEP THIS IN MIND", "The invariant", t.invariant);
        egui::CollapsingHeader::new("Inspect node roles and summaries").show(ui, |ui| {
            for n in &self.editor.diagram.nodes {
                ui.label(format!("{} — {}", n.label, n.note));
            }
            for (k, v) in &self.editor.diagram.rows {
                ui.label(format!("{k}: {v}"));
            }
        });
        self.checkpoint(ui, 18 + self.nav.structure, 1);
    }
    pub(super) fn practice(&mut self, ui: &mut egui::Ui) {
        let id = if self.study.structure_questions {
            18 + self.nav.structure
        } else {
            self.nav.lesson
        };
        page_title(
            ui,
            "PRACTICE STUDIO",
            topic_title(id),
            "Make a prediction. Explore the solution. Try a slightly harder case.",
        );
        self.checkpoint_summary(ui, id);
        let before = self.study.case;
        ui.horizontal_wrapped(|ui| {
            for i in 0..10 {
                let difficulty = if i < 3 {
                    "Easy"
                } else if i < 7 {
                    "Medium"
                } else {
                    "Hard"
                };
                ui.selectable_value(&mut self.study.case, i, format!("{} · {difficulty}", i + 1));
            }
        });
        if before != self.study.case {
            self.load_exercise();
        }
        card().show(ui, |ui| {
            ui.set_width(ui.available_width());
            badge(
                ui,
                self.study.exercise.difficulty,
                if self.study.case < 3 {
                    SOFT_GREEN
                } else if self.study.case < 7 {
                    BLUE
                } else {
                    BEIGE
                },
            );
            ui.heading(&self.study.exercise.title);
            ui.label(&self.study.exercise.prompt);
            muted(
                ui,
                concat!(
                    "Ten variations on this topic's core algorithm. Hard cases also ",
                    "ask for correctness and cost reasoning."
                ),
            );
            if !self.study.reveal {
                ui.strong("What do you think will happen?");
                ui.add(
                    egui::TextEdit::multiline(&mut self.study.answer)
                        .hint_text("Write a prediction or reasoning to check…")
                        .desired_rows(3)
                        .desired_width(ui.available_width()),
                );
                if primary(ui, "Explore the worked solution  →").clicked() {
                    self.study.reveal = true;
                    self.playback.step = 0;
                }
            }
        });
        if !self.study.reveal {
            return;
        }
        section(
            ui,
            "THE REASONING",
            "How to solve it",
            &self.study.exercise.explanation,
        );
        self.trace_panel(ui);
        complexity(ui, self.study.exercise.cost);
        card().show(ui, |ui| {
            ui.set_width(ui.available_width());
            eyebrow(ui, "CHECK YOUR UNDERSTANDING");
            ui.heading("Compare your prediction.");
            if !self.study.answer.trim().is_empty() {
                muted(ui, format!("Your prediction: {}", self.study.answer));
            }
            ui.strong(format!("Calculated answer: {}", self.study.exercise.answer));
            ui.label(concat!(
                "Explain why the last state is correct. Then try a different case ",
                "without opening the solution first."
            ));
            self.checkpoint(ui, id, 2);
            if self.study.case < 9 && primary(ui, "Try the next case →").clicked() {
                self.study.case += 1;
                self.load_exercise();
            }
        });
    }
    pub(super) fn trace_panel(&mut self, ui: &mut egui::Ui) {
        card().show(ui, |ui| {
            ui.set_width(ui.available_width());
            eyebrow(ui, "ONE STEP AT A TIME");
            let f = &self.study.exercise.frames
                [self.playback.step.min(self.study.exercise.frames.len() - 1)];
            let note = f.note.clone();
            draw_scene(
                ui,
                &mut self.scene,
                &self.study.diagrams[self.playback.step.min(self.study.diagrams.len() - 1)],
                format!(
                    "trace-{}-{}-{}-{}-{}-{}",
                    self.study.structure_questions,
                    self.nav.lesson,
                    self.nav.structure,
                    self.nav.extra,
                    self.study.case,
                    self.playback.step
                ),
                self.storage.progress.reduced_motion,
            );
            legend(ui);
            egui::Frame::new()
                .fill(BLUE)
                .corner_radius(12)
                .inner_margin(16.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    eyebrow(ui, "WHAT JUST HAPPENED");
                    ui.label(RichText::new(note).strong().color(TAUPE));
                });
        });
    }
}
