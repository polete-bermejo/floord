use crate::UNKNOWN;
use crate::services::reports::{Outcome, Report};
use crate::services::savings::save;
use anyhow::Result;
use std::fs::{DirEntry, remove_dir};
use std::path::Path;
use std::time::{Duration, SystemTime};

/// Initialize the service
pub fn service(
    inbox_dir: &Path,
    seconds: &Duration,
    notify: bool,
    report: &mut Report,
) -> Result<()> {
    let read_dir = inbox_dir.read_dir()?;

    for entry in read_dir {
        let Ok(dir_entry) = entry else { continue };
        match manage_entry(&dir_entry, inbox_dir, seconds, notify) {
            Ok(a) => report.record(a),
            Err(e) => report.fail(dir_entry.path(), e),
        }
    }

    remove_empty_unknown(inbox_dir);
    Ok(())
}

/// Removes the Unknown folder if it is empty.
///
/// `remove_dir` only succeeds on an empty directory, so a folder that still
/// holds files is never touched and there is no check-then-remove race.
fn remove_empty_unknown(inbox_dir: &Path) {
    let _ = remove_dir(inbox_dir.join(UNKNOWN));
}

/// Filters and manages the file-entry in the folder
fn manage_entry(
    dir_entry: &DirEntry,
    inbox_dir: &Path,
    delay_in_seconds: &Duration,
    notify: bool,
) -> Result<Outcome> {
    let metadata = dir_entry.metadata()?;
    let current_time = SystemTime::now();

    // We only need modified info
    if metadata
        .modified()
        .ok()
        .and_then(|t| current_time.duration_since(t).ok())
        .is_some_and(|d| d < *delay_in_seconds)
    {
        return Ok(Outcome::Skipped);
    }

    if dir_entry.path() == inbox_dir.join(UNKNOWN) {
        return Ok(Outcome::Skipped);
    }

    save(dir_entry, inbox_dir, notify)?;
    Ok(Outcome::Saved)
}
