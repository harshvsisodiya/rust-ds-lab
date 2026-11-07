//! Native, event-driven learning workspace. No browser or network at runtime.
use crate::{
    catalog::TOPICS,
    curriculum::{LESSONS, LINKS},
    model,
    practice::{self, Exercise},
    progress::{self, Progress},
    theme::*,
};
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, RichText, Stroke, Vec2};
use std::time::{Duration, Instant};
mod extensions;
mod navigation;
mod painting;
mod state;
use painting::*;
use state::*;
pub struct Lab {
    nav: Navigation,
    study: Study,
    playback: Playback,
    editor: Editor,
    storage: Storage,
    scene: Scene,
}
impl Lab {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut lab = Self::with_context(&cc.egui_ctx);
        lab.storage.pref_path = progress::path();
        if let Some(path) = &lab.storage.pref_path {
            lab.storage.progress = Progress::load(path);
        }
        let session = lab.storage.progress.session.clone();
        lab.restore_session(&session);
        cc.egui_ctx.set_zoom_factor(lab.nav.text_scale);
        lab.storage.saved = lab.storage.progress.clone();
        lab.nav.tutorial = if lab.storage.progress.welcomed {
            None
        } else {
            Some(0)
        };
        lab
    }
    fn with_context(ctx: &egui::Context) -> Self {
        apply(ctx);
        let items = model::tokens(TOPICS[0].example);
        let diagram = model::build(TOPICS[0].id, &items);
        let exercise = practice::exercise(0, 0);
        let diagrams = exercise.frames.iter().map(frame_diagram).collect();
        Self {
            nav: Navigation {
                view: View::Home,
                tab: Tab::Understand,
                lesson: 0,
                structure: 0,
                extra: 0,
                zoom: 1.0,
                search: String::new(),
                library_group: 0,
                tutorial: None,
                command_open: false,
                command_query: String::new(),
                focus_search: false,
                back: Vec::new(),
                forward: Vec::new(),
                previous: None,
                restoring: false,
                text_scale: 1.0,
                scroll: 0.0,
                restore_scroll: false,
            },
            study: Study {
                case: 0,
                structure_questions: false,
                exercise,
                diagrams,
                reveal: false,
                answer: String::new(),
                rust_code: true,
            },
            playback: Playback {
                step: 0,
                play: false,
                last: Instant::now(),
                speed: 1.5,
            },
            editor: Editor {
                data: TOPICS[0].example.into(),
                value: "42".into(),
                index: 0,
                items,
                diagram,
                message: String::new(),
                error: false,
            },
            storage: Storage {
                progress: Progress::default(),
                saved: Progress::default(),
                pref_path: None,
                save_error: String::new(),
                dirty_since: None,
            },
            scene: Scene {
                old: model::Diagram::default(),
                current: model::Diagram::default(),
                key: String::new(),
                transition: Instant::now(),
            },
        }
    }
    fn navigate(&mut self, v: View) {
        self.nav.view = v;
        self.nav.scroll = 0.0;
        self.playback.play = false;
        self.editor.message.clear();
    }
    fn select_lesson(&mut self, i: usize) {
        self.nav.lesson = i;
        self.study.case = 0;
        self.study.structure_questions = false;
        self.load_exercise();
        self.nav.tab = Tab::Understand;
        self.navigate(View::Lesson);
    }
    fn load_exercise(&mut self) {
        self.study.exercise = if self.study.structure_questions {
            practice::structure_exercise(self.nav.structure, self.study.case)
        } else {
            practice::exercise(self.nav.lesson, self.study.case)
        };
        self.study.diagrams = self
            .study
            .exercise
            .frames
            .iter()
            .map(frame_diagram)
            .collect();
        self.playback.step = 0;
        self.playback.play = false;
        self.study.reveal = false;
        self.study.answer.clear();
        self.scene.key.clear();
    }
    fn select_structure(&mut self, i: usize) {
        self.nav.structure = i;
        self.editor.data = TOPICS[i].example.into();
        self.editor.items = model::tokens(&self.editor.data);
        self.playback.step = self.editor.items.len();
        self.editor.diagram = model::build(TOPICS[i].id, &self.editor.items);
        self.editor.value = match TOPICS[i].id {
            "hashmap" => "fox:6",
            "hashset" => "fox",
            "trie" => "can",
            "graph" | "dsu" => "A-D",
            _ => "42",
        }
        .into();
        self.editor.index = self.editor.items.len();
        self.nav.tab = Tab::Understand;
        self.scene.key.clear();
        self.navigate(View::Structures);
    }
    fn start_practice(&mut self, structure: bool) {
        self.study.structure_questions = structure;
        self.study.case = 0;
        self.load_exercise();
        self.navigate(View::Practice);
    }
    fn rebuild(&mut self) {
        self.editor.diagram = model::build(
            TOPICS[self.nav.structure].id,
            &self.editor.items[..self.playback.step.min(self.editor.items.len())],
        );
        self.scene.key.clear();
    }
    fn action(&mut self, op: &str) {
        self.playback.play = false;
        match model::mutate(
            TOPICS[self.nav.structure].id,
            &self.editor.items[..self.playback.step.min(self.editor.items.len())],
            op,
            &self.editor.value,
            self.editor.index,
        ) {
            Ok((items, msg)) => {
                self.editor.items = items;
                self.editor.data = self.editor.items.join(", ");
                self.playback.step = self.editor.items.len();
                self.editor.index = self.editor.items.len();
                self.editor.message = msg;
                self.editor.error = false;
                self.rebuild();
            }
            Err(e) => {
                self.editor.message = e;
                self.editor.error = true;
            }
        }
    }
    fn checkpoint(&mut self, ui: &mut egui::Ui, id: usize, stage: usize) {
        let text = [
            "I understand the idea",
            "I can explain the visual steps",
            "I worked through a practice case",
        ][stage];
        let mut done = self.storage.progress.checkpoints[id] & (1 << stage) != 0;
        egui::Frame::new()
            .fill(SOFT_GREEN)
            .corner_radius(12)
            .inner_margin(14.0)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                eyebrow(ui, "SAVE A CHECKPOINT");
                if ui.checkbox(&mut done, text).changed() {
                    self.storage.progress.set(id, stage, done);
                }
                muted(
                    ui,
                    if self.storage.progress.completed(id) {
                        "Topic finished. All three checkpoints are saved on this computer."
                    } else {
                        "Your progress is saved on this computer. Come back whenever you're ready."
                    },
                );
            });
    }
    fn checkpoint_summary(&self, ui: &mut egui::Ui, id: usize) {
        ui.horizontal_wrapped(|ui| {
            for (stage, label) in ["Understand", "Visualize", "Practice"].iter().enumerate() {
                let done = self.storage.progress.checkpoints[id] & (1 << stage) != 0;
                badge(
                    ui,
                    &format!("{} {label}", if done { "✓" } else { "○" }),
                    if done { SOFT_GREEN } else { CREAM },
                );
            }
        });
    }
    fn home(&mut self, ui: &mut egui::Ui) {
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
    fn home_actions(&mut self, ui: &mut egui::Ui, next: usize) {
        ui.horizontal_wrapped(|ui| {
            if primary(ui, "Continue learning  →").clicked() {
                self.select_lesson(next);
            }
            if ui.button("Explore the roadmap").clicked() {
                self.navigate(View::Roadmap);
            }
        });
    }
    fn library(&mut self, ui: &mut egui::Ui) {
        page_title(
            ui,
            "THE LIBRARY",
            "Find your next idea.",
            "Choose a guided topic or go straight to a data structure. Your \
                        checkpoints travel with you.",
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
    fn roadmap(&mut self, ui: &mut egui::Ui) {
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
            "Each card shows saved checkpoints: Understand · Visualize · Practice. \
                        Select a card to continue.",
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
    fn lesson(&mut self, ui: &mut egui::Ui) {
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
    fn controls(&mut self, ui: &mut egui::Ui, max: usize) -> bool {
        let before = self.playback.step;
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
            ui.add(egui::Slider::new(&mut self.playback.speed, 0.5..=5.0).text("steps/s"));
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
    fn structures(&mut self, ui: &mut egui::Ui) {
        let t = &TOPICS[self.nav.structure];
        page_title(
            ui,
            &format!("STRUCTURE LAB / {}", t.group.to_uppercase()),
            t.title,
            t.about,
        );
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
                section(
                    ui,
                    "A SMALL LEARNING CHALLENGE",
                    t.challenge,
                    "Make a prediction, try it in the playground, and explain the invariant \
                        after the operation.",
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
    fn playground(&mut self, ui: &mut egui::Ui) {
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
            muted(ui,
"Each step shows a completed insertion. Reset, then press Next, to build gradually.");
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
    fn practice(&mut self, ui: &mut egui::Ui) {
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
                "Ten variations on this topic's core algorithm. Hard cases also ask for \
                        correctness and cost reasoning.",
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
            ui.label(
                "Explain why the last state is correct. Then try a different case \
                        without opening the solution first.",
            );
            self.checkpoint(ui, id, 2);
            if self.study.case < 9 && primary(ui, "Try the next case →").clicked() {
                self.study.case += 1;
                self.load_exercise();
            }
        });
    }
    fn trace_panel(&mut self, ui: &mut egui::Ui) {
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
    fn sidebar(&mut self, ctx: &egui::Context) {
        let narrow = ctx.screen_rect().width() < 1050.0;
        egui::SidePanel::left("navigation")
            .exact_width(if narrow { 172.0 } else { 216.0 })
            .resizable(false)
            .frame(egui::Frame::new().fill(PAPER).inner_margin(18.0))
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(30.0), egui::Sense::hover());
                    ui.painter().rect_filled(r, 9.0, TAUPE);
                    for (i, h) in [9.0, 17.0, 12.0].iter().enumerate() {
                        ui.painter().rect_filled(
                            Rect::from_min_size(
                                r.min + Vec2::new(6.0 + i as f32 * 7.0, 23.0 - h),
                                Vec2::new(4.0, *h),
                            ),
                            2.0,
                            BEIGE,
                        );
                    }
                    ui.label(
                        RichText::new("rust ds lab")
                            .size(if narrow { 17.0 } else { 19.0 })
                            .strong()
                            .color(TAUPE),
                    );
                });
                ui.add_space(26.0);
                eyebrow(ui, "LEARN AT YOUR PACE");
                ui.add_space(8.0);
                for (view, label) in [
                    (View::Home, "Start here"),
                    (View::Library, "Lesson library"),
                    (View::Roadmap, "Roadmap"),
                ] {
                    let selected = self.nav.view == view;
                    let response = ui.add_sized(
                        [ui.available_width(), 43.0],
                        egui::Button::new(RichText::new(label).color(if selected {
                            PAPER
                        } else {
                            TAUPE
                        }))
                        .fill(if selected {
                            TAUPE
                        } else {
                            Color32::TRANSPARENT
                        })
                        .stroke(Stroke::NONE),
                    );
                    if response.clicked() {
                        self.navigate(view);
                    }
                }
                ui.add_space(22.0);
                card().inner_margin(14.0).fill(CREAM).show(ui, |ui| {
                    let count = (0..47)
                        .filter(|i| self.storage.progress.completed(*i))
                        .count();
                    eyebrow(ui, "YOUR PROGRESS");
                    ui.label(
                        RichText::new(format!("{count} / 47"))
                            .size(25.0)
                            .strong()
                            .color(TAUPE),
                    );
                    ui.add(
                        egui::ProgressBar::new(count as f32 / 47.0)
                            .fill(SAGE)
                            .desired_height(5.0),
                    );
                    ui.label(
                        RichText::new("Topics & labs finished.\nEvery checkpoint counts.")
                            .size(12.0)
                            .color(MUTED),
                    );
                });
                ui.add_space(16.0);
                if ui.button("?  Take the tour").clicked() {
                    self.nav.tutorial = Some(0);
                    self.playback.play = false;
                }
                ui.checkbox(&mut self.storage.progress.reduced_motion, "Reduce motion");
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.label(
                        RichText::new("v0.4 · native & offline")
                            .size(12.0)
                            .color(MUTED),
                    );
                });
            });
    }
    fn tour(&mut self, ctx: &egui::Context) {
        let Some(step) = self.nav.tutorial else {
            return;
        };
        let steps=[("A calmer way to learn.",
"Start with one idea. You don't need to know the roadmap or any code yet.",
"Your first lesson explains why a structure exists, how it works, and \
                        where you would use it."),
("Read it. Then see it.",
"Every lesson follows the same path: Understand → Visual walkthrough → Practice.",
"In a structure lab, open Playground to enter your own values. The blue \
                        note explains what happened after each step."),
("You control the pace.",
"Next advances one state. Back revisits it. Play moves automatically; \
                        Reset begins again.",
"Blue identifies the active item. Node labels and step explanations communicate meaning alongside color. \
                        Reduce motion turns off transitions."),
("Pick up where you left off.",
"Mark a checkpoint after understanding, visualizing and practicing. All three complete a topic.",
"Progress is saved on this computer. Your roadmap shows checkpoints, and Home shows your most recently \
                        finished topic. Reopen this tour from the sidebar.")];

        egui::Modal::new(egui::Id::new("welcome_tour"))
            .frame(card())
            .show(ctx, |ui| {
                ui.set_width((ctx.screen_rect().width() - 100.0).min(510.0));

                ui.add_space(10.0);

                eyebrow(ui, &format!("A QUICK TOUR / {} OF 4", step + 1));

                ui.label(
                    RichText::new(steps[step].0)
                        .size(32.0)
                        .strong()
                        .color(TAUPE),
                );

                mini_array(
                    ui,
                    &["Learn", "Try", "Explain"],
                    Some(step.min(2)),
                    "Small steps. Clear connections.",
                );

                ui.label(steps[step].1);

                muted(ui, steps[step].2);

                ui.add_space(10.0);

                ui.horizontal_wrapped(|ui| {
                    if step > 0 && ui.button("← Back").clicked() {
                        self.nav.tutorial = Some(step - 1);
                    }

                    if primary(
                        ui,
                        if step == 3 {
                            "Start my first lesson →"
                        } else {
                            "Continue →"
                        },
                    )
                    .clicked()
                    {
                        if step == 3 {
                            self.nav.tutorial = None;

                            self.storage.progress.welcomed = true;

                            self.select_lesson(0);
                        } else {
                            self.nav.tutorial = Some(step + 1);
                        }
                    }

                    if ui.button("Explore on my own").clicked() {
                        self.nav.tutorial = None;

                        self.storage.progress.welcomed = true;
                    }
                });

                ui.add_space(8.0);
            });
    }

    fn show(&mut self, ctx: &egui::Context) {
        self.shortcuts(ctx);

        let playing_view = self.nav.view == View::Practice && self.study.reveal
            || self.nav.view == View::Lesson && self.nav.tab == Tab::Playground
            || self.nav.view == View::Structures && self.nav.tab == Tab::Playground
            || self.nav.view == View::Extensions
                && self.study.reveal
                && self.nav.tab == Tab::Playground;

        if !playing_view || self.nav.tutorial.is_some() || self.nav.command_open {
            self.playback.play = false;
        }

        if self.playback.play {
            let delay = Duration::from_secs_f32(1.0 / self.playback.speed);

            let max = if self.nav.view == View::Structures {
                self.editor.items.len()
            } else {
                self.study.exercise.frames.len() - 1
            };

            if self.playback.last.elapsed() >= delay {
                if self.playback.step < max {
                    self.playback.step += 1;

                    if self.nav.view == View::Structures {
                        self.rebuild();
                    }
                }

                if self.playback.step >= max {
                    self.playback.play = false;
                }

                self.playback.last = Instant::now();
            }

            if self.playback.play {
                ctx.request_repaint_after(delay.saturating_sub(self.playback.last.elapsed()));
            }
        }

        self.sidebar(ctx);

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(CREAM).inner_margin(
                if ctx.screen_rect().width() < 1050.0 {
                    20.0
                } else {
                    34.0
                },
            ))
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui
                        .add_enabled(!self.nav.back.is_empty(), egui::Button::new("←"))
                        .on_hover_text("Back · Alt+Left")
                        .clicked()
                    {
                        self.history_back();
                    }

                    if ui
                        .add_enabled(!self.nav.forward.is_empty(), egui::Button::new("→"))
                        .on_hover_text("Forward · Alt+Right")
                        .clicked()
                    {
                        self.history_forward();
                    }

                    if ui.small_button("Home").clicked() {
                        self.navigate(View::Home);
                    }

                    muted(ui, "/");

                    let location = match self.nav.view {
                        View::Home => "Welcome",

                        View::Library => "Library",

                        View::Roadmap => "Roadmap",

                        View::Lesson => LESSONS[self.nav.lesson].title,

                        View::Structures => TOPICS[self.nav.structure].title,

                        View::Practice => "Practice studio",

                        View::Extensions => crate::extras::TOPICS[self.nav.extra].title,
                    };

                    muted(ui, location);

                    if matches!(
                        self.nav.view,
                        View::Lesson | View::Structures | View::Practice | View::Extensions
                    ) && ui.small_button("← All lessons").clicked()
                    {
                        self.navigate(View::Library);
                    }
                });

                ui.horizontal_wrapped(|ui| {
                    if ui.button("Search everything  Ctrl/⌘ K").clicked() {
                        self.nav.command_open = true;

                        self.nav.focus_search = true;
                    }

                    let before = self.nav.text_scale;

                    ui.add(
                        egui::Slider::new(&mut self.nav.text_scale, 0.8..=1.4).text("Text size"),
                    );

                    if before != self.nav.text_scale {
                        ctx.set_zoom_factor(self.nav.text_scale);
                    }
                });

                if playing_view {
                    card().inner_margin(12.0).show(ui, |ui| {
                        let max = if self.nav.view == View::Structures {
                            self.editor.items.len()
                        } else {
                            self.study.exercise.frames.len().saturating_sub(1)
                        };
                        if self.controls(ui, max) && self.nav.view == View::Structures {
                            self.rebuild();
                        }
                        ui.label(
                            RichText::new("← / → Step     Space Play / pause     Ctrl/⌘ R Reset")
                                .size(11.0)
                                .color(MUTED),
                        );
                    });
                }
                ui.add_space(10.0);
                let mut scroll = egui::ScrollArea::vertical()
                    .id_salt(format!(
                        "page-{:?}-{}-{}-{}-{:?}",
                        self.nav.view,
                        self.nav.lesson,
                        self.nav.structure,
                        self.nav.extra,
                        self.nav.tab
                    ))
                    .auto_shrink([false, false]);

                if self.nav.restore_scroll {
                    scroll = scroll.vertical_scroll_offset(self.nav.scroll);

                    self.nav.restore_scroll = false;
                }

                let output = scroll.show(ui, |ui| {
                    ui.set_max_width(ui.available_width().min(1150.0));

                    match self.nav.view {
                        View::Home => self.home(ui),

                        View::Library => self.library(ui),

                        View::Roadmap => self.roadmap(ui),

                        View::Lesson => self.lesson(ui),

                        View::Structures => self.structures(ui),

                        View::Practice => self.practice(ui),

                        View::Extensions => self.extensions(ui),
                    }

                    ui.add_space(20.0);
                });

                self.nav.scroll = output.state.offset.y;
            });

        self.tour(ctx);

        self.command_palette(ctx);

        self.remember_route();

        self.storage.progress.session = self.session_text();

        if self.storage.saved != self.storage.progress {
            let since = self.storage.dirty_since.get_or_insert_with(Instant::now);

            if since.elapsed() >= Duration::from_millis(800) {
                self.save_now();
            } else {
                ctx.request_repaint_after(
                    Duration::from_millis(800).saturating_sub(since.elapsed()),
                );
            }
        }

        if !self.storage.save_error.is_empty() {
            egui::Window::new("Local progress")
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label(&self.storage.save_error);

                    if ui.button("Dismiss").clicked() {
                        self.storage.save_error.clear();
                    }
                });
        }
    }
}

impl eframe::App for Lab {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_now();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show(ctx);
    }
}

#[cfg(test)]
mod tests;
