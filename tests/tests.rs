use dirs::{audio_dir, document_dir, picture_dir, video_dir};
use floord::saving_service;
use std::fs::{File, create_dir, remove_file};
use std::time::Duration;
use tempfile::{tempdir, tempdir_in};

const TEST_FILE: &str = "test_c_a_r_a_c_u_l_o";

#[test]
fn dont_move_by_duration() {
    let tmp = tempdir().unwrap();
    let secs = Duration::new(300, 0);
    let probe = &tmp.path().join(TEST_FILE);
    create_dir(probe).unwrap();
    saving_service(tmp.path(), &secs);
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
    saving_service(tmp, &secs);
    assert!(!probe.exists())
}

#[test]
#[ignore]
fn move_to_documents() {
    let tmp = tempdir_in(".").unwrap();
    let secs = Duration::ZERO;

    let test_file = format!("{TEST_FILE}.pdf");
    let probe = tmp.path().join(&test_file);

    File::create_new(&probe).unwrap();

    saving_service(tmp.path(), &secs);
    let destination = document_dir().unwrap().join(&test_file);

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

    saving_service(tmp.path(), &secs);
    let destination = picture_dir().unwrap().join(&test_file);

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

    saving_service(tmp.path(), &secs);
    let destination = audio_dir().unwrap().join(&test_file);

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

    saving_service(tmp.path(), &secs);
    let destination = video_dir().unwrap().join(&test_file);

    assert!(!probe.exists());
    assert!(destination.exists());
    remove_file(destination).unwrap();
}
