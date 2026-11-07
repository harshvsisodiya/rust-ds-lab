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
#[derive(Clone, Copy, PartialEq, Debug)]
enum View {
    Home,
    Library,
    Roadmap,
    Lesson,
    Structures,
    Practice,
}
#[derive(Clone, Copy, PartialEq, Debug)]
enum Tab {
    Understand,
    Playground,
    Code,
}
pub struct Lab {
    view: View,
    tab: Tab,
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
    progress: Progress,
    saved: Progress,
    pref_path: Option<std::path::PathBuf>,
    save_error: String,
    tutorial: Option<usize>,
    search: String,
    library_structures: bool,
    answer: String,
    old_diagram: model::Diagram,
    scene: model::Diagram,
    scene_key: String,
    transition: Instant,
}
impl Lab {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut lab = Self::with_context(&cc.egui_ctx);
        lab.pref_path = progress::path();
        if let Some(path) = &lab.pref_path {
            lab.progress = Progress::load(path);
        }
        lab.saved = lab.progress.clone();
        lab.tutorial = if lab.progress.welcomed { None } else { Some(0) };
        lab
    }
    fn with_context(ctx: &egui::Context) -> Self {
        apply(ctx);
        let items = model::tokens(TOPICS[0].example);
        let diagram = model::build(TOPICS[0].id, &items);
        Self {
            view: View::Home,
            tab: Tab::Understand,
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
            zoom: 1.0,
            progress: Progress::default(),
            saved: Progress::default(),
            pref_path: None,
            save_error: String::new(),
            tutorial: None,
            search: String::new(),
            library_structures: false,
            answer: String::new(),
            old_diagram: model::Diagram::default(),
            scene: model::Diagram::default(),
            scene_key: String::new(),
            transition: Instant::now(),
        }
    }
    fn navigate(&mut self, v: View) {
        self.view = v;
        self.play = false;
        self.message.clear();
    }
    fn select_lesson(&mut self, i: usize) {
        self.lesson = i;
        self.case = 0;
        self.structure_questions = false;
        self.load_exercise();
        self.tab = Tab::Understand;
        self.navigate(View::Lesson);
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
        self.answer.clear();
        self.scene_key.clear();
    }
    fn select_structure(&mut self, i: usize) {
        self.structure = i;
        self.data = TOPICS[i].example.into();
        self.items = model::tokens(&self.data);
        self.step = self.items.len();
        self.diagram = model::build(TOPICS[i].id, &self.items);
        self.value = match TOPICS[i].id {
            "hashmap" => "fox:6",
            "hashset" => "fox",
            "trie" => "can",
            "graph" | "dsu" => "A-D",
            _ => "42",
        }
        .into();
        self.index = self.items.len();
        self.tab = Tab::Understand;
        self.scene_key.clear();
        self.navigate(View::Structures);
    }
    fn start_practice(&mut self, structure: bool) {
        self.structure_questions = structure;
        self.case = 0;
        self.load_exercise();
        self.navigate(View::Practice);
    }
    fn rebuild(&mut self) {
        self.diagram = model::build(
            TOPICS[self.structure].id,
            &self.items[..self.step.min(self.items.len())],
        );
        self.scene_key.clear();
    }
    fn action(&mut self, op: &str) {
        self.play = false;
        match model::mutate(
            TOPICS[self.structure].id,
            &self.items[..self.step.min(self.items.len())],
            op,
            &self.value,
            self.index,
        ) {
            Ok((items, msg)) => {
                self.items = items;
                self.data = self.items.join(", ");
                self.step = self.items.len();
                self.index = self.items.len();
                self.message = msg;
                self.rebuild();
            }
            Err(e) => self.message = e,
        }
    }
    fn checkpoint(&mut self, ui: &mut egui::Ui, id: usize, stage: usize) {
        let text = [
            "I understand the idea",
            "I can explain the visual steps",
            "I worked through a practice case",
        ][stage];
        let mut done = self.progress.checkpoints[id] & (1 << stage) != 0;
        egui::Frame::new()
            .fill(SOFT_GREEN)
            .corner_radius(12)
            .inner_margin(14.0)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                eyebrow(ui, "SAVE A CHECKPOINT");
                if ui.checkbox(&mut done, text).changed() {
                    self.progress.set(id, stage, done);
                }
                muted(
                    ui,
                    if self.progress.completed(id) {
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
                let done = self.progress.checkpoints[id] & (1 << stage) != 0;
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
        ui.label(
            RichText::new("Make the invisible\nfeel intuitive.")
                .size(if ui.available_width() > 700.0 {
                    48.0
                } else {
                    36.0
                })
                .strong()
                .color(TAUPE),
        );
        muted(
            ui,
            "Understand data structures by seeing what changes — and why.",
        );
        ui.add_space(12.0);
        let next = (0..18).find(|i| !self.progress.completed(*i)).unwrap_or(0);
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
                ui.label(
                    RichText::new(LESSONS[next].title)
                        .size(28.0)
                        .strong()
                        .color(TAUPE),
                );
                ui.label(LESSONS[next].why);
                mini_array(
                    ui,
                    &["12", "7", "24", "9", "18"],
                    Some(2),
                    "A position gives every value a home.",
                );
                self.checkpoint_summary(ui, next);
                ui.horizontal_wrapped(|ui| {
                    if primary(ui, "Continue learning  →").clicked() {
                        self.select_lesson(next);
                    }
                    if ui.button("Explore the roadmap").clicked() {
                        self.navigate(View::Roadmap);
                    }
                });
            });
        if let Some(i) = self.progress.recent {
            ui.add_space(5.0);
            ui.horizontal_wrapped(|ui| {
                badge(ui, "RECENTLY COMPLETED", BLUE);
                ui.strong(topic_title(i));
                if ui.button("Revisit").clicked() {
                    if i < 18 {
                        self.select_lesson(i);
                    } else {
                        self.select_structure(i - 18);
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
    fn library(&mut self, ui: &mut egui::Ui) {
        page_title(ui,"THE LIBRARY","Find your next idea.","Choose a guided topic or go straight to a data structure. Your checkpoints travel with you.");
        ui.horizontal_wrapped(|ui| {
            ui.selectable_value(&mut self.library_structures, false, "Guided topics · 18");
            ui.selectable_value(&mut self.library_structures, true, "Structure labs · 16");
        });
        ui.add(
            egui::TextEdit::singleline(&mut self.search)
                .hint_text("Search by name, use or concept…")
                .desired_width(ui.available_width())
                .margin(Vec2::new(14.0, 12.0)),
        );
        let query = self.search.to_lowercase();
        let entries: Vec<_> = if self.library_structures {
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
                        ui.set_min_height(185.0);
                        ui.set_width(ui.available_width());
                        eyebrow(ui, &group.to_uppercase());
                        ui.label(RichText::new(*title).size(21.0).strong().color(TAUPE));
                        ui.label(*body);
                        self.checkpoint_summary(
                            ui,
                            if self.library_structures { 18 + *i } else { *i },
                        );
                        if ui.button("Open lesson  →").clicked() {
                            if self.library_structures {
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
                    (0..18).filter(|i| self.progress.completed(*i)).count()
                ),
                SOFT_GREEN,
            );
            ui.add(egui::Slider::new(&mut self.zoom, 0.65..=1.3).text("Zoom"));
            if ui.button("Start from the beginning").clicked() {
                self.select_lesson(0);
            }
        });
        muted(ui,"Each card shows saved checkpoints: Understand · Visualize · Practice. Select a card to continue.");
        egui::ScrollArea::both()
            .id_salt("map_canvas")
            .show(ui, |ui| {
                let s = self.zoom;
                let (rect, _) =
                    ui.allocate_exact_size(Vec2::new(1150.0, 1080.0) * s, egui::Sense::hover());
                let p = ui.painter();
                p.rect_filled(rect, 20.0, PAPER);
                let positions: Vec<_> = LESSONS
                    .iter()
                    .map(|l| rect.min + Vec2::new(480.0 + l.x * 143.0, 55.0 + l.y * 118.0) * s)
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
                            if self.progress.completed(a) {
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
                    } else if self.progress.completed(i) {
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
                            if self.progress.checkpoints[i] & (1 << stage) != 0 {
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
        let l = &LESSONS[self.lesson];
        page_title(ui, "GUIDED LESSON", l.title, l.why);
        self.checkpoint_summary(ui, self.lesson);
        ui.horizontal_wrapped(|ui| {
            if ui
                .selectable_value(&mut self.tab, Tab::Understand, "01  Understand")
                .clicked()
            {
                self.play = false;
            }
            ui.selectable_value(&mut self.tab, Tab::Playground, "02  Visual walkthrough");
            if ui.button("03  Practice  →").clicked() {
                self.start_practice(false);
            }
        });
        if self.tab == Tab::Playground {
            self.reveal = true;
            self.trace_panel(ui);
            self.checkpoint(ui, self.lesson, 1);
            return;
        }
        section(ui, "01 / THE REASON", "Why does it exist?", l.why);
        section(ui, "02 / THE IDEA", "How does it work?", l.how);
        card().show(ui, |ui| {
            ui.set_width(ui.available_width());
            eyebrow(ui, "03 / SEE THE STORY");
            ui.heading("From input to understanding.");
            muted(ui, &self.exercise.prompt);
            let frames = &self.exercise.frames;
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
                    &frame_diagram(&frames[index]),
                    &format!("lesson-{}-{n}", self.lesson),
                );
            }
            if primary(ui, "Explore every step  →").clicked() {
                self.tab = Tab::Playground;
                self.step = 0;
            }
        });
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
        complexity(ui, self.exercise.cost);
        self.checkpoint(ui, self.lesson, 0);
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
        let before = self.step;
        ui.horizontal_wrapped(|ui| {
            if primary(ui, if self.play { "Pause" } else { "Play  →" }).clicked() {
                if self.step >= max {
                    self.step = 0;
                }
                self.play = !self.play;
                self.last = Instant::now();
            }
            if ui
                .add_enabled(self.step > 0, egui::Button::new("← Back"))
                .clicked()
            {
                self.step -= 1;
                self.play = false;
            }
            if ui
                .add_enabled(self.step < max, egui::Button::new("Next →"))
                .clicked()
            {
                self.step += 1;
                self.play = false;
            }
            if ui.button("Reset").clicked() {
                self.step = 0;
                self.play = false;
            }
            badge(ui, &format!("Step {} / {max}", self.step), BLUE);
            ui.add(egui::Slider::new(&mut self.speed, 0.5..=5.0).text("steps/s"));
        });
        ui.add(
            egui::ProgressBar::new(if max == 0 {
                0.0
            } else {
                self.step as f32 / max as f32
            })
            .fill(SAGE)
            .desired_height(4.0),
        );
        before != self.step
    }
    fn structures(&mut self, ui: &mut egui::Ui) {
        let t = &TOPICS[self.structure];
        page_title(
            ui,
            &format!("STRUCTURE LAB / {}", t.group.to_uppercase()),
            t.title,
            t.about,
        );
        self.checkpoint_summary(ui, 18 + self.structure);
        ui.horizontal_wrapped(|ui| {
            for (tab, name) in [
                (Tab::Understand, "01  Understand"),
                (Tab::Playground, "02  Playground"),
                (Tab::Code, "03  Python example"),
            ] {
                if ui.selectable_value(&mut self.tab, tab, name).clicked() {
                    self.play = false;
                }
            }
            if ui.button("Practice →").clicked() {
                self.start_practice(true);
            }
        });
        match self.tab {
            Tab::Understand => {
                let b = &crate::background::BACKGROUNDS[self.structure];
                section(ui, "01 / THE REASON", "Why do we need it?", b.why);
                card().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    eyebrow(ui, "02 / THE MENTAL MODEL");
                    ui.heading("See how the pieces fit.");
                    ui.label(t.about);
                    thumbnail(ui, &self.diagram, "structure_intro");
                    ui.strong("The rule that must stay true");
                    ui.label(t.invariant);
                    if primary(ui, "Open the playground  →").clicked() {
                        self.tab = Tab::Playground;
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
                section(ui,"A SMALL LEARNING CHALLENGE",t.challenge,"Make a prediction, try it in the playground, and explain the invariant after the operation.");
                egui::CollapsingHeader::new("Check the reasoning").show(ui, |ui| {
                    ui.label(t.answer);
                });
                self.checkpoint(ui, 18 + self.structure, 0);
            }
            Tab::Code => {
                card().show(ui,|ui| {ui.set_width(ui.available_width());eyebrow(ui,"READ / CONNECT / EXPERIMENT");ui.heading("The same idea, in Python.");muted(ui,"Read this alongside the playground. This reference code does not run inside the app.");if ui.button("Copy code").clicked() {ui.ctx().copy_text(t.python.to_string());}egui::Frame::new().fill(TAUPE).corner_radius(14).inner_margin(20.0).show(ui,|ui| {egui::ScrollArea::horizontal().id_salt("python").show(ui,|ui| {ui.label(RichText::new(t.python).monospace().color(PAPER));});});if primary(ui,"Back to the playground →").clicked() {self.tab=Tab::Playground;}});
            }
            Tab::Playground => self.playground(ui),
        }
    }
    fn playground(&mut self, ui: &mut egui::Ui) {
        let t = &TOPICS[self.structure];
        card().show(ui,|ui| {
   ui.set_width(ui.available_width());eyebrow(ui,"LIVE PLAYGROUND");if self.controls(ui,self.items.len()) {self.rebuild();}
   self.canvas(ui,self.diagram.clone(),format!("structure-{}-{}-{}",self.structure,self.step,self.data));legend(ui);
   ui.label(RichText::new(format!("{} of {} input items inserted",self.step,self.items.len())).strong().color(TAUPE));muted(ui,"Each step shows a completed insertion. Reset, then press Next, to build gradually.");
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
                egui::TextEdit::singleline(&mut self.data)
                    .desired_width(ui.available_width())
                    .margin(Vec2::splat(10.0)),
            );
            ui.horizontal_wrapped(|ui| {
                if primary(ui, "Build input").clicked() {
                    let items = model::tokens(&self.data);
                    match model::validate(t.id, &items) {
                        Ok(()) => {
                            self.items = items;
                            self.step = self.items.len();
                            self.index = self.items.len();
                            self.play = false;
                            self.message =
                                "Built. Reset to replay, or try an operation below.".into();
                            self.rebuild();
                        }
                        Err(e) => self.message = e,
                    }
                }
                if ui.button("Restore example").clicked() {
                    self.select_structure(self.structure);
                    self.tab = Tab::Playground;
                }
            });
            ui.separator();
            ui.strong("Change one thing");
            ui.horizontal_wrapped(|ui| {
                ui.label("Value / key");
                ui.add(egui::TextEdit::singleline(&mut self.value).desired_width(110.0));
                if matches!(
                    t.id,
                    "array" | "singly" | "doubly" | "circular" | "fenwick" | "segment"
                ) {
                    ui.label("Index (starts at 0)");
                    ui.add(egui::DragValue::new(&mut self.index).range(0..=model::MAX_ITEMS));
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
            if self.step < self.items.len() {
                muted(
                    ui,
                    "An operation edits the visible prefix and replaces the remaining replay.",
                );
            }
            if !self.message.is_empty() {
                ui.label(RichText::new(&self.message).strong().color(TAUPE));
            }
        });
        section(ui, "KEEP THIS IN MIND", "The invariant", t.invariant);
        egui::CollapsingHeader::new("Inspect node roles and summaries").show(ui, |ui| {
            for n in &self.diagram.nodes {
                ui.label(format!("{} — {}", n.label, n.note));
            }
            for (k, v) in &self.diagram.rows {
                ui.label(format!("{k}: {v}"));
            }
        });
        self.checkpoint(ui, 18 + self.structure, 1);
    }
    fn practice(&mut self, ui: &mut egui::Ui) {
        let id = if self.structure_questions {
            18 + self.structure
        } else {
            self.lesson
        };
        page_title(
            ui,
            "PRACTICE STUDIO",
            topic_title(id),
            "Make a prediction. Explore the solution. Try a slightly harder case.",
        );
        self.checkpoint_summary(ui, id);
        let before = self.case;
        ui.horizontal_wrapped(|ui| {
            for i in 0..10 {
                let difficulty = if i < 3 {
                    "Easy"
                } else if i < 7 {
                    "Medium"
                } else {
                    "Hard"
                };
                ui.selectable_value(&mut self.case, i, format!("{} · {difficulty}", i + 1));
            }
        });
        if before != self.case {
            self.load_exercise();
        }
        card().show(ui,|ui| {
   ui.set_width(ui.available_width());badge(ui,self.exercise.difficulty,if self.case<3 {SOFT_GREEN} else if self.case<7 {BLUE} else {BEIGE});ui.heading(&self.exercise.title);ui.label(&self.exercise.prompt);
   muted(ui,"Ten variations on this topic's core algorithm. Hard cases also ask for correctness and cost reasoning.");
   if !self.reveal {ui.strong("What do you think will happen?");ui.add(egui::TextEdit::multiline(&mut self.answer).hint_text("Write a prediction or reasoning to check…").desired_rows(3).desired_width(ui.available_width()));if primary(ui,"Explore the worked solution  →").clicked() {self.reveal=true;self.step=0;}}
  });
        if !self.reveal {
            return;
        }
        section(
            ui,
            "THE REASONING",
            "How to solve it",
            &self.exercise.explanation,
        );
        self.trace_panel(ui);
        complexity(ui, self.exercise.cost);
        card().show(ui,|ui| {
   ui.set_width(ui.available_width());eyebrow(ui,"CHECK YOUR UNDERSTANDING");ui.heading("Compare your prediction.");if !self.answer.trim().is_empty() {muted(ui,format!("Your prediction: {}",self.answer));}ui.strong(format!("Calculated answer: {}",self.exercise.answer));ui.label("Explain why the last state is correct. Then try a different case without opening the solution first.");
   self.checkpoint(ui,id,2);if self.case<9 && primary(ui,"Try the next case →").clicked() {self.case+=1;self.load_exercise();}
  });
    }
    fn trace_panel(&mut self, ui: &mut egui::Ui) {
        card().show(ui, |ui| {
            ui.set_width(ui.available_width());
            eyebrow(ui, "ONE STEP AT A TIME");
            self.controls(ui, self.exercise.frames.len() - 1);
            let f = &self.exercise.frames[self.step.min(self.exercise.frames.len() - 1)];
            let note = f.note.clone();
            let d = frame_diagram(f);
            self.canvas(
                ui,
                d,
                format!(
                    "trace-{}-{}-{}-{}-{}",
                    self.structure_questions, self.lesson, self.structure, self.case, self.step
                ),
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
    fn canvas(&mut self, ui: &mut egui::Ui, diagram: model::Diagram, key: String) {
        if self.scene_key != key {
            self.old_diagram = std::mem::replace(&mut self.scene, diagram);
            self.scene_key = key;
            self.transition = Instant::now();
        }
        let t = if self.progress.reduced_motion {
            1.0
        } else {
            (self.transition.elapsed().as_secs_f32() / 0.28).min(1.0)
        };
        if t < 1.0 {
            ui.ctx().request_repaint_after(Duration::from_millis(16));
        }
        egui::ScrollArea::both()
            .id_salt("active_canvas")
            .max_height(410.0)
            .show(ui, |ui| {
                draw_diagram(
                    ui,
                    &self.scene,
                    Some(&self.old_diagram),
                    1.0 - (1.0 - t).powi(3),
                    300.0,
                );
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
                    let selected = self.view == view;
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
                    let count = (0..18).filter(|i| self.progress.completed(*i)).count();
                    eyebrow(ui, "YOUR PROGRESS");
                    ui.label(
                        RichText::new(format!("{count} / 18"))
                            .size(25.0)
                            .strong()
                            .color(TAUPE),
                    );
                    ui.add(
                        egui::ProgressBar::new(count as f32 / 18.0)
                            .fill(SAGE)
                            .desired_height(5.0),
                    );
                    ui.label(
                        RichText::new("Topics finished.\nEvery checkpoint counts.")
                            .size(12.0)
                            .color(MUTED),
                    );
                });
                ui.add_space(16.0);
                if ui.button("?  Take the tour").clicked() {
                    self.tutorial = Some(0);
                    self.play = false;
                }
                ui.checkbox(&mut self.progress.reduced_motion, "Reduce motion");
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.label(
                        RichText::new("v0.3 · native & offline")
                            .size(12.0)
                            .color(MUTED),
                    );
                });
            });
    }
    fn tour(&mut self, ctx: &egui::Context) {
        let Some(step) = self.tutorial else {
            return;
        };
        let steps=[("A calmer way to learn.","Start with one idea. You don't need to know the roadmap or any code yet.","Your first lesson explains why a structure exists, how it works, and where you would use it."),("Read it. Then see it.","Every lesson follows the same path: Understand → Visual walkthrough → Practice.","In a structure lab, open Playground to enter your own values. The blue note explains what happened after each step."),("You control the pace.","Next advances one state. Back revisits it. Play moves automatically; Reset begins again.","Blue identifies the active item. Node labels and step explanations communicate meaning alongside color. Reduce motion turns off transitions."),("Pick up where you left off.","Mark a checkpoint after understanding, visualizing and practicing. All three complete a topic.","Progress is saved on this computer. Your roadmap shows checkpoints, and Home shows your most recently finished topic. Reopen this tour from the sidebar.")];
        egui::Modal::new(egui::Id::new("welcome_tour")).show(ctx, |ui| {
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
                    self.tutorial = Some(step - 1);
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
                        self.tutorial = None;
                        self.progress.welcomed = true;
                        self.select_lesson(0);
                    } else {
                        self.tutorial = Some(step + 1);
                    }
                }
                if ui.button("Explore on my own").clicked() {
                    self.tutorial = None;
                    self.progress.welcomed = true;
                }
            });
            ui.add_space(8.0);
        });
    }
    fn show(&mut self, ctx: &egui::Context) {
        let playing_view = self.view == View::Practice && self.reveal
            || self.view == View::Lesson && self.tab == Tab::Playground
            || self.view == View::Structures && self.tab == Tab::Playground;
        if !playing_view || self.tutorial.is_some() {
            self.play = false;
        }
        if self.play {
            let delay = Duration::from_secs_f32(1.0 / self.speed);
            let max = if self.view == View::Structures {
                self.items.len()
            } else {
                self.exercise.frames.len() - 1
            };
            if self.last.elapsed() >= delay {
                if self.step < max {
                    self.step += 1;
                    if self.view == View::Structures {
                        self.rebuild();
                    }
                }
                if self.step >= max {
                    self.play = false;
                }
                self.last = Instant::now();
            }
            if self.play {
                ctx.request_repaint_after(delay.saturating_sub(self.last.elapsed()));
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
                    if ui.small_button("Home").clicked() {
                        self.navigate(View::Home);
                    }
                    muted(ui, "/");
                    let location = match self.view {
                        View::Home => "Welcome",
                        View::Library => "Library",
                        View::Roadmap => "Roadmap",
                        View::Lesson => LESSONS[self.lesson].title,
                        View::Structures => TOPICS[self.structure].title,
                        View::Practice => "Practice studio",
                    };
                    muted(ui, location);
                    if matches!(self.view, View::Lesson | View::Structures | View::Practice)
                        && ui.small_button("← All lessons").clicked()
                    {
                        self.navigate(View::Library);
                    }
                });
                ui.add_space(20.0);
                egui::ScrollArea::vertical()
                    .id_salt(format!(
                        "page-{:?}-{}-{}-{:?}",
                        self.view, self.lesson, self.structure, self.tab
                    ))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_max_width(1150.0);
                        match self.view {
                            View::Home => self.home(ui),
                            View::Library => self.library(ui),
                            View::Roadmap => self.roadmap(ui),
                            View::Lesson => self.lesson(ui),
                            View::Structures => self.structures(ui),
                            View::Practice => self.practice(ui),
                        }
                        ui.add_space(30.0);
                    });
            });
        self.tour(ctx);
        if self.saved != self.progress {
            if let Some(path) = &self.pref_path {
                if let Err(e) = self.progress.save(path) {
                    self.save_error = format!("Progress could not be saved: {e}");
                }
            }
            self.saved = self.progress.clone();
        }
        if !self.save_error.is_empty() {
            egui::Window::new("Local progress")
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label(&self.save_error);
                    if ui.button("Dismiss").clicked() {
                        self.save_error.clear();
                    }
                });
        }
    }
}
fn topic_title(i: usize) -> &'static str {
    if i < 18 {
        LESSONS[i].title
    } else {
        TOPICS[i - 18].title
    }
}
fn page_title(ui: &mut egui::Ui, category: &str, title: &str, description: &str) {
    eyebrow(ui, category);
    ui.label(RichText::new(title).size(34.0).strong().color(TAUPE));
    muted(ui, description);
    ui.add_space(8.0);
}
fn small_card(ui: &mut egui::Ui, number: &str, title: &str, body: &str, color: Color32) {
    card().show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.set_min_height(135.0);
        badge(ui, number, color);
        ui.label(RichText::new(title).size(20.0).strong().color(TAUPE));
        muted(ui, body);
    });
}
fn mini_array(ui: &mut egui::Ui, values: &[&str], active: Option<usize>, caption: &str) {
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
fn legend(ui: &mut egui::Ui) {
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
fn frame_diagram(f: &practice::Frame) -> model::Diagram {
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
fn thumbnail(ui: &mut egui::Ui, d: &model::Diagram, id: &str) {
    egui::ScrollArea::both()
        .id_salt(id)
        .max_height(270.0)
        .show(ui, |ui| {
            draw_diagram(ui, d, None, 1.0, 190.0);
        });
}
fn draw_diagram(
    ui: &mut egui::Ui,
    d: &model::Diagram,
    old: Option<&model::Diagram>,
    t: f32,
    min_height: f32,
) {
    let width = ui.available_width().max(250.0);
    let max_x = d.nodes.iter().map(|n| n.x as f32).fold(0.0, f32::max) + 75.0;
    let max_y = d.nodes.iter().map(|n| n.y as f32).fold(0.0, f32::max) + 78.0;
    let scale = ((width - 20.0) / max_x.max(1.0)).clamp(0.75, 1.15);
    let size = Vec2::new(
        width.max(max_x * scale + 20.0),
        (max_y * scale + 24.0).max(min_height),
    );
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let p = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    p.rect_filled(rect, 16.0, CREAM);
    let mut y = rect.top() + 18.0;
    while y < rect.bottom() {
        let mut x = rect.left() + 18.0;
        while x < rect.right() {
            p.circle_filled(Pos2::new(x, y), 0.65, LINE);
            x += 24.0;
        }
        y += 24.0;
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
        let r = Rect::from_center_size(center, Vec2::new(96.0, 48.0) * scale);
        p.rect_filled(r.translate(Vec2::new(0.0, 3.0)), 11.0, LINE);
        p.rect_filled(r, 11.0, if n.active { BLUE } else { SOFT_GREEN });
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
        p.text(center, Align2::CENTER_CENTER, &n.label, font, TAUPE);
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
fn complexity(ui: &mut egui::Ui, cost: &str) {
    card().show(ui,|ui| {
  ui.set_width(ui.available_width());eyebrow(ui,"HOW THE WORK GROWS");ui.heading("Time & space, in plain terms.");ui.label(cost);muted(ui,"n is the amount of input. O(1) stays roughly constant; O(log n) grows slowly; O(n) visits a proportional amount of data. Space measures extra memory.");
  let (r,_)=ui.allocate_exact_size(Vec2::new(ui.available_width(),200.0),egui::Sense::hover());let plot=r.shrink2(Vec2::new(35.0,30.0));let p=ui.painter();p.line_segment([plot.left_top(),plot.left_bottom()],Stroke::new(1.0_f32,GRAY));p.line_segment([plot.left_bottom(),plot.right_bottom()],Stroke::new(1.0_f32,GRAY));
  for (kind,color) in [(0,SAGE),(1,TAUPE),(2,MUTED)] {let points:Vec<_>=(0..50).map(|i| {let x=i as f32/49.0;let y=match kind {0=>0.12,1=>(1.0+x*15.0).log2()/5.0,_=>x*0.95};Pos2::new(plot.left()+x*plot.width(),plot.bottom()-y*plot.height())}).collect();p.add(egui::Shape::line(points,Stroke::new(2.5_f32,color)));}
  p.text(r.left_top()+Vec2::new(0.0,3.0),Align2::LEFT_TOP,"Relative work",FontId::proportional(11.0),MUTED);p.text(r.right_bottom(),Align2::RIGHT_BOTTOM,"Input size →",FontId::proportional(11.0),MUTED);
  ui.horizontal_wrapped(|ui| {badge(ui,"O(1) · constant",SOFT_GREEN);badge(ui,"O(log n) · logarithmic",BEIGE);badge(ui,"O(n) · linear",BLUE);});muted(ui,"Illustrative growth curves, not benchmarks. The operation costs above describe this lesson; the chart compares common growth rates.");
 });
}
impl eframe::App for Lab {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show(ctx);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn draw(ctx: &egui::Context, lab: &mut Lab, size: [f32; 2]) -> egui::FullOutput {
        ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::from(size))),
                ..Default::default()
            },
            |ctx| lab.show(ctx),
        )
    }
    #[test]
    fn all_native_views_render_at_small_and_large_sizes() {
        let ctx = egui::Context::default();
        let mut lab = Lab::with_context(&ctx);
        lab.progress.reduced_motion = true;
        for size in [[800.0, 600.0], [1280.0, 860.0]] {
            let check = |lab: &mut Lab| {
                let o = draw(&ctx, lab, size);
                assert!(!ctx.tessellate(o.shapes, o.pixels_per_point).is_empty());
            };
            for v in [View::Home, View::Library, View::Roadmap] {
                lab.view = v;
                check(&mut lab);
            }
            for i in 0..LESSONS.len() {
                lab.select_lesson(i);
                check(&mut lab);
                lab.tab = Tab::Playground;
                check(&mut lab);
                lab.start_practice(false);
                lab.reveal = true;
                check(&mut lab);
                lab.step = lab.exercise.frames.len() - 1;
                check(&mut lab);
            }
            for i in 0..TOPICS.len() {
                lab.select_structure(i);
                for tab in [Tab::Understand, Tab::Playground, Tab::Code] {
                    lab.tab = tab;
                    check(&mut lab);
                }
                lab.start_practice(true);
                lab.case = 9;
                lab.load_exercise();
                lab.reveal = true;
                check(&mut lab);
                lab.step = lab.exercise.frames.len() - 1;
                check(&mut lab);
            }
            for i in 0..4 {
                lab.tutorial = Some(i);
                check(&mut lab);
            }
            lab.tutorial = None;
        }
    }
    #[test]
    fn navigation_stops_playback_and_example_inputs_are_valid() {
        let ctx = egui::Context::default();
        let mut lab = Lab::with_context(&ctx);
        for (i, t) in TOPICS.iter().enumerate() {
            lab.play = true;
            lab.select_structure(i);
            assert!(!lab.play);
            assert_eq!(lab.tab, Tab::Understand);
            assert!(model::validate(t.id, &lab.items).is_ok());
            lab.action("insert");
            assert!(!lab.message.contains("must"), "{}: {}", t.id, lab.message);
        }
        lab.play = true;
        lab.select_lesson(3);
        assert_eq!(lab.step, 0);
        assert!(!lab.play);
    }
    /// Exports real egui meshes and font atlas for offline visual review.
    #[test]
    #[ignore]
    fn export_native_previews() {
        let dir = std::env::var("DS_LAB_PREVIEW").expect("set DS_LAB_PREVIEW");
        std::fs::create_dir_all(&dir).unwrap();
        for name in [
            "home",
            "lesson",
            "playground",
            "practice",
            "library",
            "roadmap",
            "tour",
            "small",
        ] {
            let ctx = egui::Context::default();
            let mut lab = Lab::with_context(&ctx);
            lab.progress.reduced_motion = true;
            match name {
                "lesson" => lab.select_lesson(3),
                "playground" => {
                    lab.select_structure(9);
                    lab.tab = Tab::Playground;
                }
                "practice" => {
                    lab.select_lesson(3);
                    lab.start_practice(false);
                    lab.reveal = true;
                    lab.step = 1;
                }
                "library" => lab.view = View::Library,
                "roadmap" => lab.view = View::Roadmap,
                "tour" => lab.tutorial = Some(0),
                _ => (),
            }
            let size = if name == "small" {
                [800.0, 600.0]
            } else {
                [1280.0, 900.0]
            };
            let mut textures = Vec::new();
            let mut meshes = Vec::new();
            for _ in 0..3 {
                let output = draw(&ctx, &mut lab, size);
                for (id, delta) in output.textures_delta.set {
                    let (w, h) = (delta.image.width(), delta.image.height());
                    let pixels: Vec<[u8; 4]> = match &delta.image {
                        egui::ImageData::Color(im) => {
                            im.pixels.iter().map(|c| c.to_array()).collect()
                        }
                        egui::ImageData::Font(im) => {
                            im.srgba_pixels(None).map(|c| c.to_array()).collect()
                        }
                    };
                    textures.push(serde_json::json!({"id":format!("{id:?}"),"w":w,"h":h,"pos":delta.pos,"pixels":pixels}));
                }
                meshes=ctx.tessellate(output.shapes,output.pixels_per_point).into_iter().filter_map(|p| {if let egui::epaint::Primitive::Mesh(m)=p.primitive {Some(serde_json::json!({"texture":format!("{:?}",m.texture_id),"clip":[p.clip_rect.min.x,p.clip_rect.min.y,p.clip_rect.max.x,p.clip_rect.max.y],"indices":m.indices,"vertices":m.vertices.iter().map(|v|[v.pos.x,v.pos.y,v.uv.x,v.uv.y,v.color.r() as f32,v.color.g() as f32,v.color.b() as f32,v.color.a() as f32]).collect::<Vec<_>>()}))} else {None}}).collect();
            }
            std::fs::write(
                format!("{dir}/{name}.json"),
                serde_json::to_vec(
                    &serde_json::json!({"size":size,"textures":textures,"meshes":meshes}),
                )
                .unwrap(),
            )
            .unwrap();
        }
    }
}
