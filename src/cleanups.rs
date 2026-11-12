use anyhow::Context;
use std::fs::{create_dir_all, rename};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub fn cleanup(duration_old: &Duration, path: &PathBuf, soon_deleted_folder_name: &str) {
    let Some(file_name) = path.file_name() else {
        return;
    };
    let metadata = path.metadata().context("reading metadata").unwrap();
    if metadata
        .accessed()
        .ok()
        .and_then(|t| SystemTime::now().duration_since(t).ok())
        .is_some_and(|d| d < *duration_old)
    {
        return;
    }
    if let Some(soon_deleted) = path.parent().map(|i| i.join(soon_deleted_folder_name)) {
        create_dir_all(&soon_deleted)
            .context("creating SoonDeleted")
            .unwrap();
        let _ = rename(path, soon_deleted.join(file_name));
    }
}
