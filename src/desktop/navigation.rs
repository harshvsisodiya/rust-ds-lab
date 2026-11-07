use super::*;
impl Lab {
    pub(super) fn route(&self) -> Route {
        Route {
            view: self.nav.view,
            tab: self.nav.tab,
            lesson: self.nav.lesson,
            structure: self.nav.structure,
            extra: self.nav.extra,
            case: self.study.case,
            structure_questions: self.study.structure_questions,
        }
    }
    pub(super) fn remember_route(&mut self) {
        let route = self.route();
        if self.nav.previous.is_some_and(|last| last != route) && !self.nav.restoring {
            if let Some(last) = self.nav.previous {
                self.nav.back.push(last);
            }
            if self.nav.back.len() > 64 {
                self.nav.back.remove(0);
            }
            self.nav.forward.clear();
        }
        self.nav.previous = Some(route);
        self.nav.restoring = false;
    }
    fn restore_route(&mut self, route: Route) {
        match route.view {
            View::Lesson => {
                self.select_lesson(route.lesson);
                self.study.case = route.case;
                self.load_exercise();
            }
            View::Structures => self.select_structure(route.structure),
            View::Practice => {
                self.nav.lesson = route.lesson;
                self.nav.structure = route.structure;
                self.study.structure_questions = route.structure_questions;
                self.study.case = route.case;
                self.load_exercise();
            }
            View::Extensions => {
                self.select_extra(route.extra);
                self.study.case = route.case;
                self.editor.data = crate::extras::example(route.extra, route.case);
                self.build_extra();
            }
            _ => (),
        }
        self.nav.view = route.view;
        self.nav.tab = route.tab;
        self.playback.play = false;
        self.nav.restoring = true;
    }
    pub(super) fn history_back(&mut self) {
        if let Some(route) = self.nav.back.pop() {
            self.nav.forward.push(self.route());
            self.restore_route(route);
        }
    }
    pub(super) fn history_forward(&mut self) {
        if let Some(route) = self.nav.forward.pop() {
            self.nav.back.push(self.route());
            self.restore_route(route);
        }
    }
    pub(super) fn shortcuts(&mut self, ctx: &egui::Context) {
        let command = egui::Modifiers::COMMAND;
        if ctx.input_mut(|i| i.consume_key(command, egui::Key::K)) {
            self.nav.command_open = !self.nav.command_open;
            self.nav.focus_search = true;
            self.playback.play = false;
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            self.nav.command_open = false;
            self.playback.play = false;
        }
        if ctx.wants_keyboard_input() || self.nav.command_open || self.nav.tutorial.is_some() {
            return;
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::ALT, egui::Key::ArrowLeft)) {
            self.history_back();
            return;
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::ALT, egui::Key::ArrowRight)) {
            self.history_forward();
            return;
        }
        let active = self.nav.view == View::Practice && self.study.reveal
            || matches!(self.nav.view, View::Lesson | View::Structures)
                && self.nav.tab == Tab::Playground
            || self.nav.view == View::Extensions
                && self.study.reveal
                && self.nav.tab == Tab::Playground;
        if !active {
            return;
        }
        let max = if self.nav.view == View::Structures {
            self.editor.items.len()
        } else {
            self.study.exercise.frames.len().saturating_sub(1)
        };
        let before = self.playback.step;
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight)) {
            self.playback.step = (self.playback.step + 1).min(max);
            self.playback.play = false;
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft)) {
            self.playback.step = self.playback.step.saturating_sub(1);
            self.playback.play = false;
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Space)) {
            if self.playback.step >= max {
                self.playback.step = 0;
            }
            self.playback.play = !self.playback.play;
            self.playback.last = Instant::now();
        }
        if ctx.input_mut(|i| i.consume_key(command, egui::Key::R)) {
            self.playback.step = 0;
            self.playback.play = false;
        }
        if before != self.playback.step && self.nav.view == View::Structures {
            self.rebuild();
        }
    }
    pub(super) fn command_palette(&mut self, ctx: &egui::Context) {
        if !self.nav.command_open {
            return;
        }
        egui::Modal::new(egui::Id::new("global-search"))
            .frame(card())
            .show(ctx, |ui| {
                ui.set_width((ctx.screen_rect().width() - 100.0).min(580.0));
                eyebrow(ui, "JUMP TO ANY TOPIC");
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.nav.command_query)
                        .hint_text("Search lessons, algorithms and structures…")
                        .desired_width(ui.available_width()),
                );
                if self.nav.focus_search {
                    response.request_focus();
                    self.nav.focus_search = false;
                }
                let query = self.nav.command_query.to_lowercase();
                let mut entries: Vec<(usize, &str, &str)> = LESSONS
                    .iter()
                    .enumerate()
                    .map(|(i, t)| (i, t.title, t.why))
                    .chain(
                        TOPICS
                            .iter()
                            .enumerate()
                            .map(|(i, t)| (18 + i, t.title, t.about)),
                    )
                    .chain(
                        crate::extras::TOPICS
                            .iter()
                            .enumerate()
                            .map(|(i, t)| (34 + i, t.title, t.why)),
                    )
                    .filter(|(_, title, body)| {
                        format!("{title} {body}").to_lowercase().contains(&query)
                    })
                    .collect();
                entries.truncate(12);
                let enter = ui.input(|i| i.key_pressed(egui::Key::Enter));
                let mut choice = None;
                egui::ScrollArea::vertical()
                    .max_height(390.0)
                    .show(ui, |ui| {
                        for (n, (id, title, _)) in entries.iter().enumerate() {
                            if ui
                                .add_sized([ui.available_width(), 38.0], egui::Button::new(*title))
                                .clicked()
                                || enter && n == 0
                            {
                                choice = Some(*id);
                            }
                        }
                        if entries.is_empty() {
                            muted(ui, "No matches. Try sorting, cache, tree, or search.");
                        }
                    });
                muted(ui, "Enter opens the first match · Esc closes search");
                if let Some(id) = choice {
                    self.nav.command_open = false;
                    if id < 18 {
                        self.select_lesson(id);
                    } else if id < 34 {
                        self.select_structure(id - 18);
                    } else {
                        self.select_extra(id - 34);
                    }
                }
            });
    }
    pub(super) fn session_text(&self) -> String {
        let r = self.route();
        let view = match r.view {
            View::Home => 0,
            View::Library => 1,
            View::Roadmap => 2,
            View::Lesson => 3,
            View::Structures => 4,
            View::Practice => 5,
            View::Extensions => 6,
        };
        let tab = match r.tab {
            Tab::Understand => 0,
            Tab::Playground => 1,
            Tab::Code => 2,
        };
        let text = format!(
            "1\t{view}\t{tab}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            r.lesson,
            r.structure,
            r.extra,
            r.case,
            u8::from(r.structure_questions),
            self.playback.step,
            self.playback.speed,
            u8::from(self.study.reveal),
            self.editor.index,
            self.nav.scroll,
            self.nav.text_scale,
            self.editor.data,
            self.editor.value,
            self.study.answer.replace(['\t', '\r', '\n'], " ")
        );
        text.bytes().map(|b| format!("{b:02x}")).collect()
    }
    pub(super) fn restore_session(&mut self, text: &str) -> bool {
        if text.len() > 16000 || !text.len().is_multiple_of(2) {
            return false;
        }
        let bytes: Option<Vec<u8>> = text
            .as_bytes()
            .as_chunks::<2>()
            .0
            .iter()
            .map(|b| {
                std::str::from_utf8(b)
                    .ok()
                    .and_then(|s| u8::from_str_radix(s, 16).ok())
            })
            .collect();
        let Some(value) = bytes.and_then(|b| String::from_utf8(b).ok()) else {
            return false;
        };
        let f: Vec<_> = value.split('\t').collect();
        if f.len() != 17 || f[0] != "1" {
            return false;
        }
        let index = |i: usize| f[i].parse::<usize>().ok();
        let (Some(view), Some(tab), Some(lesson), Some(structure), Some(extra), Some(case)) =
            (index(1), index(2), index(3), index(4), index(5), index(6))
        else {
            return false;
        };
        if view > 6
            || tab > 2
            || lesson >= 18
            || structure >= 16
            || extra >= crate::extras::TOPICS.len()
            || case >= 10
        {
            return false;
        }
        let route = Route {
            view: [
                View::Home,
                View::Library,
                View::Roadmap,
                View::Lesson,
                View::Structures,
                View::Practice,
                View::Extensions,
            ][view],
            tab: [Tab::Understand, Tab::Playground, Tab::Code][tab],
            lesson,
            structure,
            extra,
            case,
            structure_questions: f[7] == "1",
        };
        // Validate user-controlled state before changing the live session.
        if route.view == View::Structures
            && model::validate(TOPICS[structure].id, &model::tokens(f[14])).is_err()
        {
            return false;
        }
        if route.view == View::Extensions && crate::extras::trace(extra, f[14], case).is_err() {
            return false;
        }
        self.restore_route(route);
        self.editor.data = f[14].into();
        self.editor.value = f[15].into();
        self.study.answer = f[16].into();
        if route.view == View::Structures {
            let items = model::tokens(&self.editor.data);
            if model::validate(TOPICS[structure].id, &items).is_err() {
                return false;
            }
            self.editor.items = items;
        }
        if route.view == View::Extensions && !self.build_extra() {
            return false;
        }
        let max = if route.view == View::Structures {
            self.editor.items.len()
        } else {
            self.study.exercise.frames.len().saturating_sub(1)
        };
        self.playback.step = index(8).unwrap_or(0).min(max);
        self.playback.speed = f[9]
            .parse::<f32>()
            .ok()
            .filter(|v| v.is_finite())
            .unwrap_or(1.5)
            .clamp(0.5, 5.0);
        self.study.reveal = f[10] == "1";
        self.editor.index = index(11).unwrap_or(0).min(16);
        self.nav.scroll = f[12]
            .parse::<f32>()
            .ok()
            .filter(|v| v.is_finite())
            .unwrap_or(0.0)
            .clamp(0.0, 50000.0);
        self.nav.text_scale = f[13]
            .parse::<f32>()
            .ok()
            .filter(|v| v.is_finite())
            .unwrap_or(1.0)
            .clamp(0.8, 1.4);
        self.nav.restore_scroll = true;
        self.playback.play = false;
        if route.view == View::Structures {
            self.rebuild();
        }
        self.nav.previous = Some(self.route());
        true
    }
    pub(super) fn save_now(&mut self) {
        self.storage.progress.session = self.session_text();
        if let Some(path) = &self.storage.pref_path {
            match self.storage.progress.save(path) {
                Ok(()) => {
                    self.storage.saved = self.storage.progress.clone();
                    self.storage.save_error.clear();
                }
                Err(e) => self.storage.save_error = format!("Could not save progress: {e}"),
            }
        } else {
            self.storage.saved = self.storage.progress.clone();
        }
        self.storage.dirty_since = None;
    }
}
