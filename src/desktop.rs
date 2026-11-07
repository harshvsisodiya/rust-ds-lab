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
mod browse;
mod extensions;
mod learning;
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
        let steps = [
            (
                "A calmer way to learn.",
                "Start with one idea. You don't need to know the roadmap or any code yet.",
                concat!(
                    "Your first lesson explains why a structure exists, how it works, ",
                    "and where you would use it."
                ),
            ),
            (
                "Read it. Then see it.",
                "Every lesson follows the same path: Understand → Visual walkthrough → Practice.",
                concat!(
                    "In a structure lab, open Playground to enter your own values. The ",
                    "blue note explains what happened after each step."
                ),
            ),
            (
                "You control the pace.",
                concat!(
                    "Next advances one state. Back revisits it. Play moves ",
                    "automatically; Reset begins again."
                ),
                concat!(
                    "Blue identifies the active item. Node labels and step ",
                    "explanations communicate meaning alongside color. Reduce motion ",
                    "turns off transitions."
                ),
            ),
            (
                "Pick up where you left off.",
                concat!(
                    "Mark a checkpoint after understanding, visualizing and ",
                    "practicing. All three complete a topic."
                ),
                concat!(
                    "Progress is saved on this computer. Your roadmap shows ",
                    "checkpoints, and Home shows your most recently finished topic. ",
                    "Reopen this tour from the sidebar."
                ),
            ),
        ];
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
                    ui.menu_button("Aa · Text size", |ui| {
                        ui.add(
                            egui::Slider::new(&mut self.nav.text_scale, 0.8..=1.4)
                                .fixed_decimals(1)
                                .text("Scale"),
                        );
                    });
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
