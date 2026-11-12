use std::path::PathBuf;

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
