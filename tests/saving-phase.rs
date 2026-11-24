use dirs::{audio_dir, document_dir, picture_dir, video_dir};
use floord::UNKNOWN;
use floord::services::reports::Report;
use floord::services::saving_service;
use std::fs::{File, create_dir, create_dir_all, remove_dir, remove_file};
use std::thread::sleep;
use std::time::Duration;
use tempfile::{tempdir, tempdir_in};

const TEST_FILE: &str = "SAVING";

//-------------------------------------------------------------------------------------------------
//      SAVINGS TESTS
//-------------------------------------------------------------------------------------------------

#[test]
fn dont_move_by_duration() {
    let tmp = tempdir().unwrap();
    let secs = Duration::new(300, 0);
    let probe = &tmp.path().join(TEST_FILE);
    create_dir(probe).unwrap();
    let report = saving_service(tmp.path(), &secs, false).unwrap();
    assert!(probe.exists());

    assert_eq!(
        report,
        Report {
            saved: Default::default(),
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: 1,
            failed: Default::default()
        }
    )
}

#[test]
fn move_by_duration() {
    // And creates Unknown folder
    let tmp = tempdir().unwrap();
    let tmp = tmp.path();
    let secs = Duration::ZERO;
    let probe = tmp.join(TEST_FILE);
    create_dir(&probe).unwrap();
    let report = saving_service(tmp, &secs, false).unwrap();
    assert!(!probe.exists());

    assert_eq!(
        report,
        Report {
            saved: 1,
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    )
}

#[test]
fn move_to_unknown() {
    let tmp = tempdir().unwrap();
    let tmp = tmp.path();
    let secs = Duration::ZERO;
    let probe = tmp.join(TEST_FILE);
    create_dir(&probe).unwrap();
    let report = saving_service(tmp, &secs, false).unwrap();
    assert!(!probe.exists());

    assert_eq!(
        report,
        Report {
            saved: 1,
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
    assert!(tmp.join(UNKNOWN).exists());
    assert!(tmp.join(UNKNOWN).join(TEST_FILE).exists())
}

#[test]
fn move_to_unknown_uppercase_extension() {
    let tmp = tempdir_in(".").unwrap();
    let secs = Duration::ZERO;

    let test_file = format!("{TEST_FILE}.FOAIJNF");
    let probe = tmp.path().join(&test_file);

    File::create_new(&probe).unwrap();

    let report = saving_service(tmp.path(), &secs, false).unwrap();
    let destination = tmp.path().join(UNKNOWN).join(&test_file);

    assert_eq!(
        report,
        Report {
            saved: 1,
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
    assert!(!probe.exists());
    assert!(destination.exists());
}

#[test]
fn move_to_unknown_by_duration() {
    let tmp = tempdir().unwrap();
    let tmp = tmp.path();
    let secs = Duration::from_millis(500);
    let probe = tmp.join(TEST_FILE);
    create_dir(&probe).unwrap();
    sleep(Duration::new(1, 0));
    let report = saving_service(tmp, &secs, false).unwrap();
    assert!(!probe.exists());

    assert_eq!(
        report,
        Report {
            saved: 1,
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
    assert!(tmp.join(UNKNOWN).exists());
    assert!(tmp.join(UNKNOWN).join(TEST_FILE).exists())
}

#[test]
fn no_unknown_folder_when_nothing_to_move() {
    let tmp = tempdir().unwrap();
    let tmp = tmp.path();
    saving_service(tmp, &Duration::ZERO, false).unwrap();
    assert!(!tmp.join(UNKNOWN).exists())
}

#[test]
fn no_unknown_folder_when_files_are_too_recent() {
    let tmp = tempdir().unwrap();
    let tmp = tmp.path();
    create_dir(tmp.join(TEST_FILE)).unwrap();
    saving_service(tmp, &Duration::new(300, 0), false).unwrap();
    assert!(!tmp.join(UNKNOWN).exists())
}

#[test]
fn remove_empty_unknown_folder() {
    let tmp = tempdir().unwrap();
    let tmp = tmp.path();
    create_dir(tmp.join(UNKNOWN)).unwrap();
    saving_service(tmp, &Duration::ZERO, false).unwrap();
    assert!(!tmp.join(UNKNOWN).exists())
}

#[test]
fn remove_unknown_folder_once_emptied() {
    let tmp = tempdir().unwrap();
    let tmp = tmp.path();
    let probe = tmp.join(TEST_FILE);
    create_dir(&probe).unwrap();
    saving_service(tmp, &Duration::ZERO, false).unwrap();
    let moved = tmp.join(UNKNOWN).join(TEST_FILE);
    assert!(moved.exists());

    // The user empties Unknown, the next run drops the empty folder
    remove_dir(&moved).unwrap();
    saving_service(tmp, &Duration::ZERO, false).unwrap();
    assert!(!tmp.join(UNKNOWN).exists())
}

#[test]
fn keep_unknown_folder_with_files() {
    let tmp = tempdir().unwrap();
    let tmp = tmp.path();
    create_dir_all(tmp.join(UNKNOWN).join(TEST_FILE)).unwrap();
    let report = saving_service(tmp, &Duration::ZERO, false).unwrap();
    assert_eq!(
        report,
        Report {
            saved: Default::default(),
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: 1,
            failed: Default::default()
        }
    );
    assert!(tmp.join(UNKNOWN).join(TEST_FILE).exists())
}

#[test]
#[ignore]
fn move_to_documents() {
    let tmp = tempdir_in(".").unwrap();
    let secs = Duration::ZERO;

    let test_file = format!("{TEST_FILE}.pdf");
    let probe = tmp.path().join(&test_file);

    File::create_new(&probe).unwrap();

    let report = saving_service(tmp.path(), &secs, false).unwrap();
    let destination = document_dir().unwrap().join(&test_file);

    assert_eq!(
        report,
        Report {
            saved: 1,
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
    assert!(!probe.exists());
    assert!(destination.exists());
    remove_file(destination).unwrap();
}

#[test]
#[ignore]
fn move_to_documents_uppercase_extension() {
    let tmp = tempdir_in(".").unwrap();
    let secs = Duration::ZERO;

    let test_file = format!("{TEST_FILE}.PDF");
    let probe = tmp.path().join(&test_file);

    File::create_new(&probe).unwrap();

    let report = saving_service(tmp.path(), &secs, false).unwrap();
    let destination = document_dir().unwrap().join(&test_file);

    assert_eq!(
        report,
        Report {
            saved: 1,
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
    assert!(!probe.exists());
    assert!(destination.exists());
    remove_file(destination).unwrap();
}

#[test]
#[ignore]
fn move_to_pictures() {
    let tmp = tempdir_in(".").unwrap();
    let secs = Duration::ZERO;

    let test_file = format!("{TEST_FILE}.jpg");
    let probe = tmp.path().join(&test_file);

    File::create_new(&probe).unwrap();

    let report = saving_service(tmp.path(), &secs, false).unwrap();
    let destination = picture_dir().unwrap().join(&test_file);

    assert_eq!(
        report,
        Report {
            saved: 1,
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
    assert!(!probe.exists());
    assert!(destination.exists());
    remove_file(destination).unwrap();
}

#[test]
#[ignore]
fn move_to_pictures_uppercase_extension() {
    let tmp = tempdir_in(".").unwrap();
    let secs = Duration::ZERO;

    let test_file = format!("{TEST_FILE}.JPG");
    let probe = tmp.path().join(&test_file);

    File::create_new(&probe).unwrap();

    let report = saving_service(tmp.path(), &secs, false).unwrap();
    let destination = picture_dir().unwrap().join(&test_file);

    assert_eq!(
        report,
        Report {
            saved: 1,
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
    assert!(!probe.exists());
    assert!(destination.exists());
    remove_file(destination).unwrap();
}

#[test]
#[ignore]
fn move_to_audio() {
    let tmp = tempdir_in(".").unwrap();
    let secs = Duration::ZERO;

    let test_file = format!("{TEST_FILE}.mp3");
    let probe = tmp.path().join(&test_file);

    File::create_new(&probe).unwrap();

    let report = saving_service(tmp.path(), &secs, false).unwrap();
    let destination = audio_dir().unwrap().join(&test_file);

    assert_eq!(
        report,
        Report {
            saved: 1,
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
    assert!(!probe.exists());
    assert!(destination.exists());
    remove_file(destination).unwrap();
}

#[test]
#[ignore]
fn move_to_audio_uppercase_extension() {
    let tmp = tempdir_in(".").unwrap();
    let secs = Duration::ZERO;

    let test_file = format!("{TEST_FILE}.MP3");
    let probe = tmp.path().join(&test_file);

    File::create_new(&probe).unwrap();

    let report = saving_service(tmp.path(), &secs, false).unwrap();
    let destination = audio_dir().unwrap().join(&test_file);

    assert_eq!(
        report,
        Report {
            saved: 1,
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
    assert!(!probe.exists());
    assert!(destination.exists());
    remove_file(destination).unwrap();
}

#[test]
#[ignore]
fn move_to_video() {
    let tmp = tempdir_in(".").unwrap();
    let secs = Duration::ZERO;

    let test_file = format!("{TEST_FILE}.mkv");
    let probe = tmp.path().join(&test_file);

    File::create_new(&probe).unwrap();

    let report = saving_service(tmp.path(), &secs, false).unwrap();
    let destination = video_dir().unwrap().join(&test_file);

    assert_eq!(
        report,
        Report {
            saved: 1,
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
    assert!(!probe.exists());
    assert!(destination.exists());
    remove_file(destination).unwrap();
}

#[test]
#[ignore]
fn move_to_video_uppercase_extension() {
    let tmp = tempdir_in(".").unwrap();
    let secs = Duration::ZERO;

    let test_file = format!("{TEST_FILE}.MKV");
    let probe = tmp.path().join(&test_file);

    File::create_new(&probe).unwrap();

    let report = saving_service(tmp.path(), &secs, false).unwrap();
    let destination = video_dir().unwrap().join(&test_file);

    assert_eq!(
        report,
        Report {
            saved: 1,
            cleaned: Default::default(),
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
    assert!(!probe.exists());
    assert!(destination.exists());
    remove_file(destination).unwrap();
}

//-------------------------------------------------------------------------------------------------
//      PATH RENAMING TESTS
//-------------------------------------------------------------------------------------------------

#[test]
fn correct_renaming_savings() {
    let path = tempdir_in(".").unwrap();
    let path = path.path();
    let unknown_path = path.join(UNKNOWN);
    create_dir_all(path.join(TEST_FILE)).unwrap();

    let duration = Duration::ZERO;
    saving_service(path, &duration, false).unwrap(); // File 0

    create_dir_all(path.join(TEST_FILE)).unwrap();
    saving_service(path, &duration, false).unwrap(); // File with (1)

    create_dir_all(path.join(TEST_FILE)).unwrap();
    saving_service(path, &duration, false).unwrap(); // File with (2)

    assert!(unknown_path.join(TEST_FILE).exists());
    let second_file = TEST_FILE.to_owned() + "(1)";
    assert!(unknown_path.join(second_file).exists());
    let third_file = TEST_FILE.to_owned() + "(2)";
    assert!(unknown_path.join(third_file).exists())
}
