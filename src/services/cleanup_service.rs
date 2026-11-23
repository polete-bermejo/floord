use crate::services::cleanups::{cleanup_phase, deletion_phase, remove_empty_soon_deleted};
use crate::services::reports::{Outcome, Report};
use crate::{SECURE, SOON_DELETED};
use anyhow::Result;
use std::fs::DirEntry;
use std::ops::Add;
use std::path::Path;
use std::time::Duration;

pub fn service(
    paths: &Vec<&Path>,
    duration_old: &Duration,
    duration_grace: &Duration,
    allow_deletion: bool,
    notify: bool,
    report: &mut Report,
) -> Result<()> {
    let limit = duration_old.add(*duration_grace);

    for folder in paths {
        if let Ok(entries) = folder.read_dir() {
            for entry in entries {
                let Ok(entry) = entry else {
                    continue;
                };
                manage_entry(&entry, duration_old, notify);
            }
        }

        if allow_deletion {
            deletion_phase(folder, &limit, report)?;
        }
        remove_empty_soon_deleted(folder);
    }
    Ok(())
}

fn manage_entry(dir_entry: &DirEntry, duration_old: &Duration, notify: bool) {
    let path = dir_entry.path();
    if path
        .file_name()
        .is_some_and(|name| name == SOON_DELETED || name == SECURE)
        && path.is_dir()
    {
        return;
    }

    cleanup_phase(duration_old, &path, notify);
}
