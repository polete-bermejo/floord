use std::path::{Path, PathBuf};
use std::time::Duration;

mod cleanup_service;
mod cleanups;
mod saving_service;
mod savings;
mod service_utils;

pub const SOON_DELETED: &str = "SoonDeleted";
pub const UNKNOWN: &str = "Unknown";

pub fn saving_service(path: &Path, duration: &Duration) {
    saving_service::service(path, duration);
}

pub fn cleanup_service(paths: &Vec<PathBuf>, duration_old: &Duration, duration_grace: &Duration) {
    cleanup_service::service(paths, duration_old, duration_grace);
}
