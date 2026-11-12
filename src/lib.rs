use std::path::Path;
use std::time::Duration;

mod saving_service;
mod savings;

pub fn saving_service(path: &Path, duration: &Duration) {
    saving_service::service(path, duration);
}
#[allow(unused_variables)] // Leave this while working on it TODO: cleanup_service.rs
pub fn cleanup_service(
    exclude_paths: &[&Path],
    duration_old: &Duration,
    duration_grace: &Duration,
) -> anyhow::Result<()> {
    Ok(())
}
