use super::*;
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum View {
    Home,
    Library,
    Roadmap,
    Lesson,
    Structures,
    Practice,
    Extensions,
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Tab {
    Understand,
    Playground,
    Code,
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) struct Route {
    pub view: View,
    pub tab: Tab,
    pub lesson: usize,
    pub structure: usize,
    pub extra: usize,
    pub case: usize,
    pub structure_questions: bool,
}
pub(super) struct Navigation {
    pub view: View,
    pub tab: Tab,
    pub lesson: usize,
    pub structure: usize,
    pub extra: usize,
    pub zoom: f32,
    pub search: String,
    pub library_group: u8,
    pub tutorial: Option<usize>,
    pub command_open: bool,
    pub command_query: String,
    pub focus_search: bool,
    pub back: Vec<Route>,
    pub forward: Vec<Route>,
    pub previous: Option<Route>,
    pub restoring: bool,
    pub text_scale: f32,
    pub scroll: f32,
    pub restore_scroll: bool,
}
pub(super) struct Study {
    pub case: usize,
    pub structure_questions: bool,
    pub exercise: Exercise,
    pub reveal: bool,
    pub answer: String,
    pub diagrams: Vec<model::Diagram>,
    pub rust_code: bool,
}
pub(super) struct Playback {
    pub step: usize,
    pub play: bool,
    pub last: Instant,
    pub speed: f32,
}
pub(super) struct Editor {
    pub data: String,
    pub value: String,
    pub index: usize,
    pub items: Vec<String>,
    pub diagram: model::Diagram,
    pub message: String,
    pub error: bool,
}
pub(super) struct Storage {
    pub progress: Progress,
    pub saved: Progress,
    pub pref_path: Option<std::path::PathBuf>,
    pub save_error: String,
    pub dirty_since: Option<Instant>,
}
pub(super) struct Scene {
    pub old: model::Diagram,
    pub current: model::Diagram,
    pub key: String,
    pub transition: Instant,
}
