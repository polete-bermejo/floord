use std::path::Path;
use std::time::Duration;

mod cleanup_service;
mod cleanups;
mod saving_service;
mod savings;

pub fn saving_service(path: &Path, duration: &Duration) {
    saving_service::service(path, duration);
}

pub fn cleanup_service(paths: &[&Path], duration_old: &Duration, duration_grace: &Duration) {
    cleanup_service::service(paths, duration_old, duration_grace);
}
