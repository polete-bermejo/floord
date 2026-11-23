use std::{path::Path, time::Duration};

use crate::services::reports::Report;
use anyhow::Result;

mod cleanup_service;
mod cleanups;
pub mod reports;
mod saving_service;
mod savings;
mod service_utils;

/// Distributes files located at the given path into their respective system default
/// folders, based on their file extensions.
///
/// Files without an extension or with an unrecognized extension will be stored
/// in an `Unknown` folder, which will be created if it does not exist.
///
/// A file will only be stored if its last modification or access time is older
/// than the specified duration.
///
/// System notifications can also be enabled to provide additional feedback.
///
/// # Errors
///
/// Returns an error if the given directory cannot be read.
pub fn saving_service(folder_path: &Path, duration: &Duration, notify: bool) -> Result<Report> {
    let mut report = Report::default();
    saving_service::service(folder_path, duration, notify, &mut report)?;
    Ok(report)
}

/// Provides two file management services: cleanup and deletion.
///
/// # Cleanup
///
/// Searches for files whose last modification or access time is older than
/// the specified duration and moves them into a `SoonDeleted` folder,
/// which will be created if it does not exist.
///
/// System notifications can also be enabled to provide additional feedback.
///
/// # Deletion
///
/// If a `SoonDeleted` folder exists in any of the given directories and
/// `allow_deletion` is set to `true`, searches for files whose last modification
/// or access time is older than `duration_old + duration_grace` and deletes them.
///
/// # Errors
///
/// Returns an error if any of the given directories cannot be read.
pub fn cleanup_service(
    folder_paths: &Vec<&Path>,
    duration_old: &Duration,
    duration_grace: &Duration,
    allow_deletion: bool,
    notify: bool,
) -> Result<Report> {
    let mut report = Report::default();
    cleanup_service::service(
        folder_paths,
        duration_old,
        duration_grace,
        allow_deletion,
        notify,
        &mut report,
    )?;
    Ok(report)
}
