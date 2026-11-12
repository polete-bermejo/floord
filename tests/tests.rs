use dirs::{audio_dir, document_dir, picture_dir, video_dir};
use floord::{SOON_DELETED, cleanup_service, saving_service};
use std::fs::{File, create_dir, read_dir, remove_file};
use std::time::Duration;
use tempfile::{tempdir, tempdir_in};

const TEST_FILE: &str = "test_c_a_r_a_c_u_l_o";

//-------------------------------------------------------------------------------------------------
//      CLEANUP TESTS
//-------------------------------------------------------------------------------------------------

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

//-------------------------------------------------------------------------------------------------
//      CLEANUP TESTS
//-------------------------------------------------------------------------------------------------

#[test]
fn shorts_directories_to_one() {
    let tmp = tempdir_in(".").unwrap();
    let span = Duration::ZERO;

    // Create 2 directories
    let path = tmp.path();
    create_dir(path.join(TEST_FILE)).unwrap();
    create_dir(path.join("asdf")).unwrap();

    let read_dir = read_dir(path).unwrap();
    let size = read_dir.count();

    cleanup_service(vec![path.to_path_buf()], &span, &Duration::new(10, 0));

    let read_dir = std::fs::read_dir(path).unwrap();
    let size2 = read_dir.count();
    assert_ne!(size, size2);
    assert_eq!(1, size2);
    assert!(path.join(SOON_DELETED).exists());
}

#[test]
fn move_directory_to_soon_deleted() {
    let tmp = tempdir_in(".").unwrap();
    let span = Duration::ZERO;

    let path = tmp.path();
    create_dir(path.join(TEST_FILE)).unwrap();

    cleanup_service(vec![path.to_path_buf()], &span, &Duration::new(10, 0));
    assert!(path.join(SOON_DELETED).exists());

    let read_dir = std::fs::read_dir(path.join(SOON_DELETED)).unwrap();
    let size = read_dir.count();
    assert_eq!(1, size); // Assert that cleanup doesn't create duplicates

    assert!(path.join(SOON_DELETED).join(TEST_FILE).exists());
}

#[test]
fn dont_move_directory_to_soon_deleted() {
    let tmp = tempdir_in(".").unwrap();
    let span = Duration::new(10, 0);

    let path = tmp.path();
    create_dir(path.join(TEST_FILE)).unwrap();

    cleanup_service(vec![path.to_path_buf()], &span, &Duration::new(10, 0));
    assert!(!path.join(SOON_DELETED).exists()); // Shouldn't exist
    assert!(!path.join(SOON_DELETED).join(TEST_FILE).exists());

    assert!(path.join(TEST_FILE).exists());
}
