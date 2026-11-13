use crate::SOON_DELETED;
use crate::service_utils::rename_destination;
use anyhow::Context;
use std::fs::{create_dir_all, remove_dir_all, remove_file, rename};
use std::path::Path;
use std::time::{Duration, SystemTime};

pub fn cleanup_phase(duration_old: &Duration, path: &Path) {
    let metadata = path.metadata().context("reading metadata").unwrap();
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
        create_dir_all(&soon_deleted)
            .context("creating SoonDeleted")
            .unwrap();

        let Some(new_path) = path.parent() else {
            return;
        };

        let new_path = rename_destination(new_path.join(SOON_DELETED).join(file_name));
        let _ = rename(path, new_path);
    }
}

pub fn deletion_phase(folder_path: &Path, limit: &Duration) {
    let soon_deleted = folder_path.join(SOON_DELETED);
    if !soon_deleted.exists() {
        return;
    }

    if let Ok(entries) = soon_deleted.read_dir() {
        for entry in entries {
            let Ok(entry) = entry else {
                continue;
            };
            let Ok(metadata) = entry.metadata() else {
                continue;
            };

            let accessed = metadata.accessed();
            let modified = metadata.modified();

            if modified.is_err() && accessed.is_err() {
                continue;
            }

            let now = SystemTime::now();

            if let Ok(accessed) = accessed
                && let Ok(got) = now.duration_since(accessed)
                && got < *limit
            {
                continue;
            }

            if let Ok(modified) = modified
                && let Ok(got) = now.duration_since(modified)
                && got < *limit
            {
                continue;
            }
            let _ = remove_file(entry.path());
            let _ = remove_dir_all(entry.path());
        }
    }
}
