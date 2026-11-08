use std::path::Path;
use std::time::Duration;

mod savings;
mod service;

pub fn service(path: &Path, duration: &Duration) -> anyhow::Result<()> {
    service::service(path, duration)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirs::{audio_dir, document_dir, picture_dir, video_dir};
    use std::fs::{File, create_dir, remove_file};
    use tempfile::{tempdir, tempdir_in};

    const TEST_FILE: &str = "test_c_a_r_a_c_u_l_o";
    #[test]
    fn service_works_test() {
        let tmp = tempdir().unwrap();
        let secs = Duration::new(300, 0);
        assert!(service::service(tmp.path(), &secs).is_ok())
    }

    #[test]
    fn dont_move_by_duration() {
        let tmp = tempdir().unwrap();
        let secs = Duration::new(300, 0);
        let probe = &tmp.path().join(TEST_FILE);
        create_dir(probe).unwrap();
        service(tmp.path(), &secs).unwrap();
        assert!(probe.exists())
    }

    #[test]
    fn move_by_duration() {
        // And creates Unknown folder
        let tmp = tempdir().unwrap();
        let tmp = tmp.path();
        let secs = Duration::ZERO;
        let probe = tmp.join(TEST_FILE);
        create_dir(&probe).unwrap();
        service(tmp, &secs).unwrap();
        assert!(!probe.exists())
    }

    #[test]
    //#[ignore]
    fn move_to_documents() {
        let tmp = tempdir_in(".").unwrap();
        let secs = Duration::ZERO;

        let test_file = format!("{TEST_FILE}.pdf");
        let probe = tmp.path().join(&test_file);

        File::create_new(&probe).unwrap();

        service(tmp.path(), &secs).unwrap();
        let destination = document_dir().unwrap().join(&test_file);

        assert!(!probe.exists());
        assert!(destination.exists());
        remove_file(destination).unwrap();
    }

    #[test]
    //#[ignore]
    fn move_to_pictures() {
        let tmp = tempdir_in(".").unwrap();
        let secs = Duration::ZERO;

        let test_file = format!("{TEST_FILE}.jpg");
        let probe = tmp.path().join(&test_file);

        File::create_new(&probe).unwrap();

        service(tmp.path(), &secs).unwrap();
        let destination = picture_dir().unwrap().join(&test_file);

        assert!(!probe.exists());
        assert!(destination.exists());
        remove_file(destination).unwrap();
    }

    #[test]
    //#[ignore]
    fn move_to_audio() {
        let tmp = tempdir_in(".").unwrap();
        let secs = Duration::ZERO;

        let test_file = format!("{TEST_FILE}.mp3");
        let probe = tmp.path().join(&test_file);

        File::create_new(&probe).unwrap();

        service(tmp.path(), &secs).unwrap();
        let destination = audio_dir().unwrap().join(&test_file);

        assert!(!probe.exists());
        assert!(destination.exists());
        remove_file(destination).unwrap();
    }

    #[test]
    //#[ignore]
    fn move_to_video() {
        let tmp = tempdir_in(".").unwrap();
        let secs = Duration::ZERO;

        let test_file = format!("{TEST_FILE}.mkv");
        let probe = tmp.path().join(&test_file);

        File::create_new(&probe).unwrap();

        service(tmp.path(), &secs).unwrap();
        let destination = video_dir().unwrap().join(&test_file);

        assert!(!probe.exists());
        assert!(destination.exists());
        remove_file(destination).unwrap();
    }
}
