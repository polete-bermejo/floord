use std::path::Path;
use std::time::Duration;

mod savings;
mod service;

pub fn service(path: &Path, duration: &Duration) -> anyhow::Result<()> {
    service::service(path, duration)?;
    Ok(())
}
