use crate::cleanups::cleanup;
use anyhow::Result;
use std::fs::DirEntry;
use std::path::Path;
use std::time::Duration;

pub fn service(paths: &[&Path], duration_old: &Duration, duration_grace: &Duration) {
    for folder in paths {
        if let Ok(entries) = folder.read_dir() {
            for entry in entries {
                let Ok(entry) = entry else {
                    continue;
                };
                manage_entry(&entry, duration_old).unwrap_or(());
            }
        }
    }
}

fn manage_entry(dir_entry: &DirEntry, duration_old: &Duration) -> Result<()> {
    let path = dir_entry.path();
    let soon_deleted_folder_name = "SoonDeleted";
    if path
        .file_name()
        .is_some_and(|name| name == soon_deleted_folder_name)
        && path.is_dir()
    {
        return Ok(());
    }

    let _ = cleanup(duration_old, &path, soon_deleted_folder_name);

    Ok(())
}
