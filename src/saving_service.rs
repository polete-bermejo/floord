use crate::savings::save;
use anyhow::Result;
use std::fs::{DirEntry, ReadDir};
use std::path::Path;
use std::time::{Duration, SystemTime};

/// Initialize the service
pub fn service(inbox_dir: &Path, seconds: &Duration) {
    let read_dir: ReadDir = inbox_dir
        .read_dir()
        .expect("[ERROR] Could not read inbox directory");

    for entry in read_dir {
        let Ok(dir_entry) = entry else { continue };
        let _ = manage_entry(&dir_entry, inbox_dir, seconds);
    }
}

/// Filters and manages the file-entry in the folder
fn manage_entry(dir_entry: &DirEntry, inbox_dir: &Path, delay_in_seconds: &Duration) -> Result<()> {
    let metadata = dir_entry.metadata()?;
    let current_time = SystemTime::now();

    // We only need modified info
    if metadata
        .modified()
        .ok()
        .and_then(|t| current_time.duration_since(t).ok())
        .is_some_and(|d| d < *delay_in_seconds)
    {
        return Ok(());
    }

    save(dir_entry, inbox_dir);
    Ok(())
}
