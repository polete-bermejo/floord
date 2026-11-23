use std::{path::Path, time::Duration};

use crate::services::reports::Report;
use anyhow::Result;

mod cleanup_service;
mod cleanups;
pub mod reports;
mod saving_service;
mod savings;
mod service_utils;

pub fn saving_service(path: &Path, duration: &Duration, notify: bool) -> Result<Report> {
    let mut report = Report::default();
    saving_service::service(path, duration, notify, &mut report)?;
    Ok(report)
}

pub fn cleanup_service(
    paths: &Vec<&Path>,
    duration_old: &Duration,
    duration_grace: &Duration,
    allow_deletion: bool,
    notify: bool,
) -> Result<Report> {
    let mut report = Report::default();
    cleanup_service::service(
        paths,
        duration_old,
        duration_grace,
        allow_deletion,
        notify,
        &mut report,
    )?;
    Ok(report)
}
