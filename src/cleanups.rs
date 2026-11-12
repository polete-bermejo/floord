use crate::SOON_DELETED;
use crate::service_utils::rename_destination;
use anyhow::Context;
use std::fs::{create_dir_all, rename};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub fn cleanup_phase(duration_old: &Duration, path: &PathBuf, soon_deleted_folder_name: &str) {
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

    if let Some(soon_deleted) = path.parent().map(|i| i.join(soon_deleted_folder_name)) {
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
