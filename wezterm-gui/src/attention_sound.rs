use mux::pane::PaneId;

#[cfg(windows)]
mod imp {
    use super::*;
    use std::collections::HashMap;
    use std::os::windows::ffi::OsStrExt;
    use std::path::PathBuf;
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::HINSTANCE;
    use windows::Win32::Media::Audio::{PlaySoundW, SND_ASYNC, SND_FILENAME, SND_NODEFAULT};

    static SOUND_BANK: OnceLock<Mutex<SoundBank>> = OnceLock::new();

    struct SoundBank {
        sounds: Vec<PathBuf>,
        bag: Vec<usize>,
        assigned: HashMap<PaneId, usize>,
        seed: u64,
    }

    impl SoundBank {
        fn new() -> Self {
            let sounds = discover_sound_files();
            let mut bank = Self {
                sounds,
                bag: vec![],
                assigned: HashMap::new(),
                seed: seed(),
            };
            bank.refill_bag();
            bank
        }

        fn path_for_pane(&mut self, pane_id: PaneId) -> Option<PathBuf> {
            if self.sounds.is_empty() {
                return None;
            }

            let index = match self.assigned.get(&pane_id) {
                Some(index) => *index,
                None => {
                    if self.bag.is_empty() {
                        self.refill_bag();
                    }
                    let index = self.bag.pop()?;
                    self.assigned.insert(pane_id, index);
                    index
                }
            };

            self.sounds.get(index).cloned()
        }

        fn forget_pane(&mut self, pane_id: PaneId) {
            self.assigned.remove(&pane_id);
        }

        fn refill_bag(&mut self) {
            self.bag = (0..self.sounds.len()).collect();

            if self.bag.len() < 2 {
                return;
            }

            for i in (1..self.bag.len()).rev() {
                let j = (self.next_random() as usize) % (i + 1);
                self.bag.swap(i, j);
            }
        }

        fn next_random(&mut self) -> u64 {
            let mut x = self.seed;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.seed = x;
            x
        }
    }

    pub fn play_for_pane(pane_id: PaneId) {
        let Some(path) = SOUND_BANK
            .get_or_init(|| Mutex::new(SoundBank::new()))
            .lock()
            .ok()
            .and_then(|mut bank| bank.path_for_pane(pane_id))
        else {
            return;
        };

        let wide: Vec<u16> = path
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            PlaySoundW(
                PCWSTR(wide.as_ptr()),
                HINSTANCE(0),
                SND_FILENAME as u32 | SND_ASYNC | SND_NODEFAULT,
            );
        }
    }

    pub fn forget_pane(pane_id: PaneId) {
        if let Some(bank) = SOUND_BANK.get() {
            if let Ok(mut bank) = bank.lock() {
                bank.forget_pane(pane_id);
            }
        }
    }

    fn seed() -> u64 {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(0);

        time ^ ((std::process::id() as u64) << 32) ^ 0xa5a5_5a5a_c3c3_3c3c
    }

    fn discover_sound_files() -> Vec<PathBuf> {
        candidate_sound_dirs()
            .into_iter()
            .find_map(|dir| {
                let mut sounds = std::fs::read_dir(&dir)
                    .ok()?
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .filter(|path| {
                        path.extension()
                            .map(|ext| ext.eq_ignore_ascii_case("wav"))
                            .unwrap_or(false)
                    })
                    .collect::<Vec<_>>();

                sounds.sort();
                if sounds.is_empty() {
                    None
                } else {
                    Some(sounds)
                }
            })
            .unwrap_or_default()
    }

    fn candidate_sound_dirs() -> Vec<PathBuf> {
        let mut dirs = vec![];

        if let Some(dir) = std::env::var_os("BENTERM_SOUND_DIR") {
            dirs.push(PathBuf::from(dir));
        }

        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                dirs.push(
                    parent
                        .join("assets")
                        .join("sounds")
                        .join("benterm-soft-cues"),
                );
                dirs.push(parent.join("sounds").join("benterm-soft-cues"));
                // v2.0.0 backwards compat: pre-rename installs shipped
                // benjaminterm-soft-cues. Keep the fallback for one
                // release cycle so existing installs don't go silent
                // mid-upgrade. Drop in v3.0.0.
                dirs.push(
                    parent
                        .join("assets")
                        .join("sounds")
                        .join("benjaminterm-soft-cues"),
                );
                dirs.push(parent.join("sounds").join("benjaminterm-soft-cues"));
            }
        }

        dirs.push(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("assets")
                .join("sounds")
                .join("benterm-soft-cues"),
        );
        dirs.push(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("assets")
                .join("sounds")
                .join("benjaminterm-soft-cues"),
        );

        dirs
    }
}

#[cfg(not(windows))]
mod imp {
    use super::*;

    pub fn play_for_pane(_pane_id: PaneId) {}

    pub fn forget_pane(_pane_id: PaneId) {}
}

pub fn play_for_pane(pane_id: PaneId) {
    imp::play_for_pane(pane_id);
}

pub fn forget_pane(pane_id: PaneId) {
    imp::forget_pane(pane_id);
}
