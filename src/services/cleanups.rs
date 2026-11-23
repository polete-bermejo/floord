use crate::SOON_DELETED;
use crate::services::reports::{Outcome, Report};
use crate::services::service_utils::Action::Cleanup;
use crate::services::service_utils::{rename_and_notify, rename_destination};
use anyhow::{Result, anyhow};
use std::fs::{DirEntry, create_dir_all, remove_dir, remove_dir_all, remove_file};
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime};

pub fn cleanup_phase(duration_old: &Duration, path: &Path, notify: bool) {
    let Ok(metadata) = path.metadata() else {
        // Future report error
        return;
    };
    if metadata
        .accessed()
        .ok()
        .and_then(|t| SystemTime::now().duration_since(t).ok())
        .is_some_and(|d| d < *duration_old)
    {
        return;
    }

    if metadata
        .modified()
        .ok()
        .and_then(|t| SystemTime::now().duration_since(t).ok())
        .is_some_and(|d| d < *duration_old)
    {
        return;
    }

    let Some(file_name) = path.file_name() else {
        return;
    };

    if let Some(soon_deleted) = path.parent().map(|i| i.join(SOON_DELETED)) {
        let Ok(()) = create_dir_all(&soon_deleted) else {
            return;
        }; // Future report error

        let Some(new_path) = path.parent() else {
            return;
        };

        let new_path = rename_destination(new_path.join(SOON_DELETED).join(file_name));
        rename_and_notify(path, &new_path, notify, &Cleanup);
    }
}

// Fix TODO
pub fn deletion_phase(folder_path: &Path, limit: &Duration, report: &mut Report) -> Result<()> {
    if let Some(outcome) = soon_delete_exists(folder_path) {
        report.record(outcome);
        return Ok(());
    }

    let soon_deleted = folder_path.join(SOON_DELETED);

    let entries = soon_deleted.read_dir()?;
    for entry in entries {
        let Ok(entry) = entry else {
            report.record(Outcome::Skipped);
            continue;
        };
        match deletion_entry(&entry, limit) {
            Ok(outcome) => report.record(outcome),
            Err(error) => report.fail(entry.path(), error),
        }
    }

    Ok(())
}

fn soon_delete_exists(folder_path: &Path) -> Option<Outcome> {
    if !folder_path.join(SOON_DELETED).exists() {
        return Some(Outcome::Skipped);
    }
    None
}

fn deletion_entry(entry: &DirEntry, limit: &Duration) -> Result<Outcome> {
    let metadata = entry.metadata()?;

    let accessed = metadata.accessed();
    let modified = metadata.modified();

    if modified.is_err() && accessed.is_err() {
        return Err(anyhow::anyhow!(
            "could not read mtime nor atime at {}",
            entry.path().display()
        ));
    }

    let now = SystemTime::now();

    if let Ok(accessed) = accessed
        && let Some(output) = check_accessed_deletion(accessed, now, limit)
    {
        return Ok(output);
    }

    if let Ok(modified) = modified
        && let Some(output) = check_modified_deletion(modified, now, limit)
    {
        return Ok(output);
    }

    if remove_file(entry.path()).is_ok() || remove_dir_all(entry.path()).is_ok() {
        return Ok(Outcome::Deleted);
    }
    Err(anyhow!(format!(
        "coudn't remove: {}",
        entry.path().display()
    )))
}

fn check_accessed_deletion(
    accessed: SystemTime,
    now: SystemTime,
    limit: &Duration,
) -> Option<Outcome> {
    if let Ok(got) = now.duration_since(accessed)
        && got < *limit
    {
        return Some(Outcome::Skipped);
    }
    None
}

fn check_modified_deletion(
    modified: SystemTime,
    now: SystemTime,
    limit: &Duration,
) -> Option<Outcome> {
    if let Ok(got) = now.duration_since(modified)
        && got < *limit
    {
        return Some(Outcome::Skipped);
    }
    None
}

/// Removes the `SoonDeleted` folder if it is empty.
///
/// `remove_dir` only succeeds on an empty directory, so a folder that still
/// holds files is never touched and there is no check-then-remove race.
pub fn remove_empty_soon_deleted(folder_path: &Path) {
    let _ = remove_dir(folder_path.join(SOON_DELETED));
}
