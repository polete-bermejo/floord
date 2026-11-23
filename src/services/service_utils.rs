use anyhow::Result;
use notify_rust::Notification;
use std::{
    fs::rename,
    io,
    path::{Path, PathBuf},
};

pub enum Action {
    Save,
    Cleanup,
}

pub fn rename_destination(old_path: PathBuf) -> PathBuf {
    let mut new_path = old_path;
    if let (Some(file_name), Some(folder)) = (new_path.file_stem(), new_path.parent()) {
        let file_name = file_name.to_string_lossy().to_string();
        let folder = folder.to_path_buf();

        let extension = new_path.extension().map(std::ffi::OsStr::to_os_string);

        let (base_name, mut count) = match file_name.rsplit_once('(') {
            Some((name, number)) => {
                match number.strip_suffix(')').and_then(|n| n.parse::<u32>().ok()) {
                    Some(n) if !name.is_empty() => (name.to_string(), n + 1),
                    _ => (file_name.clone(), 1),
                }
            }
            None => (file_name.clone(), 1),
        };

        while new_path.exists() {
            let new_name = match &extension {
                Some(ext) => format!("{} ({}).{}", base_name, count, ext.to_string_lossy()),
                None => format!("{base_name}({count})"),
            };

            new_path = folder.join(new_name);
            count += 1;
        }
        new_path
    } else {
        new_path
    }
}

pub fn rename_and_notify(
    file_path: &Path,
    new_path: &Path,
    notify: bool,
    action: &Action,
) -> Result<()> {
    rename(file_path, new_path)?;
    if notify {
        let file_name = file_path.file_name().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidFilename,
                format!("reading file_name {}", file_path.display()),
            )
        })?;
        let mut pretty_path = new_path
            .parent()
            .and_then(Path::file_name)
            .unwrap_or_default()
            .to_os_string();
        pretty_path.push("/");
        pretty_path.push(new_path.file_name().unwrap_or_default());

        let action = match action {
            Action::Save => "Saved",
            Action::Cleanup => "Cleaned",
        };

        let _ = Notification::new()
            .summary("Floord")
            .appname("Floord")
            .body(&format!(
                "{} {} -> {}",
                action,
                file_name.to_string_lossy(),
                pretty_path.to_string_lossy()
            ))
            .show();
    }
    Ok(())
}
