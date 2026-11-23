use crate::services::cleanups::{cleanup_phase, deletion_phase, remove_empty_soon_deleted};
use crate::services::reports::Report;
use anyhow::Result;
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
        cleanup_phase(folder, duration_old, notify, report)?;

        if allow_deletion {
            deletion_phase(folder, &limit, report)?;
        }
        remove_empty_soon_deleted(folder);
    }
    Ok(())
}
