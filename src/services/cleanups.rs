use crate::services::reports::{Outcome, Report};
use crate::services::service_utils::Action;
use crate::services::service_utils::{rename_and_notify, rename_destination};
use crate::{SECURE, SOON_DELETED};
use anyhow::{Result, anyhow};
use std::fs::{DirEntry, create_dir_all, remove_dir, remove_dir_all, remove_file};
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime};

pub fn cleanup_phase(
    folder_path: &Path,
    duration_old: &Duration,
    notify: bool,
    report: &mut Report,
) -> Result<()> {
    for entry in folder_path.read_dir()? {
        let Ok(entry) = entry else {
            report.record(Outcome::Skipped);
            continue;
        };
        let path = entry.path();
        match is_non_cleanup_folders(&path) {
            Ok(is) => {
                if is {
                    report.record(Outcome::Skipped);
                    continue;
                }
            }
            Err(error) => {
                report.fail(entry.path(), error);
            }
        }
        match cleanup_entry(folder_path, &path, duration_old, notify) {
            Ok(outcome) => report.record(outcome),
            Err(error) => report.fail(entry.path(), error),
        }
    }

    Ok(())
}

fn cleanup_entry(
    folder_path: &Path,
    path: &Path,
    duration_old: &Duration,
    notify: bool,
) -> Result<Outcome> {
    if must_skip_by_time(path, duration_old)? {
        return Ok(Outcome::Skipped);
    }
    create_dir_all(folder_path.join(SOON_DELETED))?;
    let soon_deleted = folder_path.join(SOON_DELETED);
    let file_name = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidFilename,
            format!("reading file_name {}", path.display()),
        )
    })?;
    let new_path = &rename_destination(soon_deleted.join(file_name));
    rename_and_notify(path, new_path, notify, &Action::Cleanup)?;

    Ok(Outcome::Cleaned)
}

fn must_skip_by_time(path: &Path, duration_old: &Duration) -> Result<bool> {
    let metadata = path.metadata()?;
    if metadata
        .accessed()
        .ok()
        .and_then(|t| SystemTime::now().duration_since(t).ok())
        .is_some_and(|d| d < *duration_old)
    {
        return Ok(true);
    }

    if metadata
        .modified()
        .ok()
        .and_then(|t| SystemTime::now().duration_since(t).ok())
        .is_some_and(|d| d < *duration_old)
    {
        return Ok(true);
    }
    Ok(false)
}

fn is_non_cleanup_folders(path: &Path) -> Result<bool> {
    let file_name = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidFilename,
            format!("reading file_name {}", path.display()),
        )
    })?;

    if file_name == SOON_DELETED || file_name == SECURE {
        return Ok(true);
    }

    Ok(false)
}

pub fn deletion_phase(folder_path: &Path, limit: &Duration, report: &mut Report) -> Result<()> {
    if let Some(outcome) = soon_delete_not_exists(folder_path) {
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

fn soon_delete_not_exists(folder_path: &Path) -> Option<Outcome> {
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
