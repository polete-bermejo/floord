use crate::SOON_DELETED;
use crate::cleanups::cleanup_phase;
use std::fs::DirEntry;
use std::path::Path;
use std::time::Duration;

pub fn service(paths: &Vec<&Path>, duration_old: &Duration, _duration_grace: &Duration) {
    for folder in paths {
        if let Ok(entries) = folder.read_dir() {
            for entry in entries {
                let Ok(entry) = entry else {
                    continue;
                };
                manage_entry(&entry, duration_old);
            }
        }
    }
}

fn manage_entry(dir_entry: &DirEntry, duration_old: &Duration) {
    let path = dir_entry.path();
    if path.file_name().is_some_and(|name| name == SOON_DELETED) && path.is_dir() {
        return;
    }

    cleanup_phase(duration_old, &path, SOON_DELETED);
}
