//! Versioned, local-only learning checkpoints. No account or network needed.
use std::{
    fs,
    path::{Path, PathBuf},
};
#[derive(Clone, Debug, PartialEq)]
pub struct Progress {
    pub welcomed: bool,
    pub reduced_motion: bool,
    pub checkpoints: [u8; 34],
    pub recent: Option<usize>,
}
impl Default for Progress {
    fn default() -> Self {
        Self {
            welcomed: false,
            reduced_motion: false,
            checkpoints: [0; 34],
            recent: None,
        }
    }
}
impl Progress {
    pub fn completed(&self, i: usize) -> bool {
        self.checkpoints[i] == 7
    }
    pub fn set(&mut self, i: usize, stage: usize, done: bool) {
        if i >= 34 || stage >= 3 {
            return;
        }
        let before = self.completed(i);
        if done {
            self.checkpoints[i] |= 1 << stage;
        } else {
            self.checkpoints[i] &= !(1 << stage);
        }
        if !before && self.completed(i) {
            self.recent = Some(i);
        }
        if self.recent == Some(i) && !self.completed(i) {
            self.recent = None;
        }
    }
    pub fn encode(&self) -> String {
        format!(
            "version=1\nwelcomed={}\nmotion={}\ncheckpoints={}\nrecent={}\n",
            u8::from(self.welcomed),
            u8::from(self.reduced_motion),
            self.checkpoints
                .iter()
                .map(|v| char::from(b'0' + v))
                .collect::<String>(),
            self.recent.map(|i| i.to_string()).unwrap_or_default()
        )
    }
    pub fn decode(text: &str) -> Self {
        let mut p = Self::default();
        for line in text.lines() {
            if line == "welcomed=1" {
                p.welcomed = true;
            }
            if line == "motion=1" {
                p.reduced_motion = true;
            }
            if let Some(bits) = line.strip_prefix("checkpoints=") {
                for (i, c) in bits.chars().take(34).enumerate() {
                    p.checkpoints[i] = c.to_digit(8).unwrap_or(0) as u8;
                }
            }
            if let Some(s) = line.strip_prefix("recent=") {
                p.recent = s.parse::<usize>().ok().filter(|i| *i < 34);
            }
        }
        if p.recent.is_some_and(|i| !p.completed(i)) {
            p.recent = None;
        }
        p
    }
    pub fn load(path: &Path) -> Self {
        fs::read_to_string(path)
            .or_else(|_| fs::read_to_string(path.with_extension("backup")))
            .map(|s| Self::decode(&s))
            .unwrap_or_default()
    }
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        // Keep the old file if a write fails. Windows rename cannot overwrite, so
        // retain a backup until the replacement has successfully been written.
        let temp = path.with_extension("tmp");
        fs::write(&temp, self.encode())?;
        #[cfg(target_os = "windows")]
        if path.exists() {
            fs::copy(path, path.with_extension("backup"))?;
            fs::remove_file(path)?;
        }
        fs::rename(temp, path)
    }
}
pub fn path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    let base = std::env::var_os("APPDATA").map(PathBuf::from);
    #[cfg(target_os = "macos")]
    let base =
        std::env::var_os("HOME").map(|p| PathBuf::from(p).join("Library/Application Support"));
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".config")));
    base.map(|p| p.join("rust-ds-lab/progress.txt"))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn saved_checkpoints_survive_restart_and_replacement() {
        let dir = std::env::temp_dir().join(format!("rust-ds-lab-test-{}", std::process::id()));
        let path = dir.join("progress.txt");
        let mut p = Progress {
            welcomed: true,
            ..Progress::default()
        };
        for stage in 0..3 {
            p.set(5, stage, true);
        }
        p.save(&path).unwrap();
        assert_eq!(Progress::load(&path), p);
        p.set(5, 2, false);
        p.save(&path).unwrap();
        assert_eq!(Progress::load(&path), p);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn checkpoints_round_trip_and_recent_completion() {
        let mut p = Progress::default();
        p.set(3, 0, true);
        p.set(3, 1, true);
        assert!(!p.completed(3));
        assert_eq!(p.recent, None);
        p.set(3, 2, true);
        assert!(p.completed(3));
        assert_eq!(p.recent, Some(3));
        assert_eq!(Progress::decode(&p.encode()), p);
        p.set(20, 0, true);
        p.set(20, 1, true);
        p.set(20, 2, true);
        assert_eq!(p.recent, Some(20));
        p.set(20, 2, false);
        assert_eq!(p.recent, None);
        assert!(p.completed(3));
        assert_eq!(
            Progress::decode("checkpoints=9garbage\nrecent=999"),
            Progress::default()
        );
    }
}
