use super::*;
impl Lab {
    pub(super) fn select_extra(&mut self, index: usize) {
        self.nav.extra = index;
        self.study.case = 0;
        self.editor.data = crate::extras::example(index, 0);
        self.nav.tab = Tab::Understand;
        self.build_extra();
        self.navigate(View::Extensions);
    }
    pub(super) fn build_extra(&mut self) -> bool {
        match crate::extras::trace(self.nav.extra, &self.editor.data, self.study.case) {
            Ok(exercise) => {
                self.study.diagrams = exercise.frames.iter().map(frame_diagram).collect();
                self.study.exercise = exercise;
                self.playback.step = 0;
                self.playback.play = false;
                self.study.reveal = true;
                self.scene.key.clear();
                self.editor.message.clear();
                self.editor.error = false;
                true
            }
            Err(e) => {
                self.editor.message = e;
                self.editor.error = true;
                false
            }
        }
    }
    pub(super) fn extensions(&mut self, ui: &mut egui::Ui) {
        let t = &crate::extras::TOPICS[self.nav.extra];
        if self.nav.tab == Tab::Playground {
            ui.label(title(t.title, 28.0));
        } else {
            page_title(ui, t.group, t.title, t.why);
        }
        ui.horizontal_wrapped(|ui| {
            let previous = self.nav.extra;
            egui::ComboBox::from_id_salt("extension-topic")
                .selected_text(t.title)
                .show_ui(ui, |ui| {
                    for (i, topic) in crate::extras::TOPICS.iter().enumerate() {
                        ui.selectable_value(&mut self.nav.extra, i, topic.title);
                    }
                });
            if previous != self.nav.extra {
                self.select_extra(self.nav.extra);
            }
            for (tab, name) in [
                (Tab::Understand, "Understand"),
                (Tab::Playground, "Lab & practice"),
                (Tab::Code, "Rust source"),
            ] {
                if ui.selectable_value(&mut self.nav.tab, tab, name).clicked() {
                    self.playback.play = false;
                }
            }
        });
        self.checkpoint_summary(ui, 34 + self.nav.extra);
        match self.nav.tab {
            Tab::Understand => {
                section(ui, "THE PURPOSE", "Why this idea matters", t.why);
                section(ui, "THE PROCEDURE", "How it works", t.how);
                if primary(ui, "Open the interactive lab →").clicked() {
                    self.nav.tab = Tab::Playground;
                }
                egui::CollapsingHeader::new("When to use it · history · complexity")
                    .default_open(false)
                    .show(ui, |ui| {
                        section(ui, "APPLICATIONS", "When it is useful", t.when);
                        section(ui, "ORIGINS", "Historical context", t.history);
                        ui.hyperlink_to("Reference ↗", t.source);
                        complexity(ui, t.cost);
                    });
                self.checkpoint(ui, 34 + self.nav.extra, 0);
            }
            Tab::Playground => {
                egui::CollapsingHeader::new(format!(
                    "Input & cases · case {} · {}",
                    self.study.case + 1,
                    self.study.exercise.difficulty
                ))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    eyebrow(ui, "CHANGE THE INPUT");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.editor.data)
                            .desired_width(ui.available_width()),
                    );
                    ui.horizontal_wrapped(|ui| {
                        if primary(ui, "Build trace").clicked() {
                            self.build_extra();
                        }
                        if ui.button("Restore example").clicked() {
                            self.editor.data = t.example.into();
                            self.build_extra();
                        }
                        if ui.button("Predict before revealing").clicked() {
                            self.study.reveal = false;
                            self.playback.play = false;
                        }
                    });
                    if !self.editor.message.is_empty() {
                        error_message(ui, &self.editor.message);
                    }
                    let before = self.study.case;
                    egui::ComboBox::from_id_salt("extra-case")
                        .selected_text(format!(
                            "Case {} · {}",
                            self.study.case + 1,
                            self.study.exercise.difficulty
                        ))
                        .show_ui(ui, |ui| {
                            for i in 0..10 {
                                let level = if i < 3 {
                                    "Easy"
                                } else if i < 7 {
                                    "Medium"
                                } else {
                                    "Hard"
                                };
                                ui.selectable_value(
                                    &mut self.study.case,
                                    i,
                                    format!("Case {} · {level}", i + 1),
                                );
                            }
                        });
                    if before != self.study.case {
                        self.editor.data = crate::extras::example(self.nav.extra, self.study.case);
                        self.study.answer.clear();
                        self.build_extra();
                    }
                    muted(
                        ui,
                        concat!(
                            "Ten input variations · E: easy, M: medium, H: hard. Describe the ",
                            "invariant as well as the output."
                        ),
                    );
                });
                if !self.study.reveal {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.study.answer)
                            .hint_text("Predict the result and explain why…")
                            .desired_rows(3)
                            .desired_width(ui.available_width()),
                    );
                    if primary(ui, "Reveal the computed solution").clicked() {
                        self.study.reveal = true;
                    }
                } else {
                    self.trace_panel(ui);
                    ui.label(title(&self.study.exercise.answer, 18.0));
                    self.checkpoint(ui, 34 + self.nav.extra, 1);
                    egui::CollapsingHeader::new("Compare your reasoning and finish practice").show(
                        ui,
                        |ui| {
                            ui.label(&self.study.exercise.explanation);
                            ui.label(&self.study.answer);
                            ui.label(self.study.exercise.cost);
                            self.checkpoint(ui, 34 + self.nav.extra, 2);
                        },
                    );
                }
            }
            Tab::Code => code_panel(
                ui,
                crate::extras::rust_source(self.nav.extra),
                concat!(
                    "Rust implementation from this project. This module includes ",
                    "trace-recording helpers; standalone examples for the core lessons ",
                    "are available in their Code tabs."
                ),
            ),
        }
    }
}
