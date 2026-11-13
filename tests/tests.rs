use dirs::{audio_dir, document_dir, picture_dir, video_dir};
use floord::{SOON_DELETED, UNKNOWN, cleanup_service, saving_service};
use std::fs::{File, create_dir, create_dir_all, read_dir, remove_file};
use std::thread::sleep;
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

    cleanup_service(&vec![path], &span, &Duration::new(10, 0), false);

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

    cleanup_service(&vec![path], &span, &Duration::new(10, 0), false);
    assert!(path.join(SOON_DELETED).exists());

    let read_dir = read_dir(path.join(SOON_DELETED)).unwrap();
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

    cleanup_service(&vec![path], &span, &Duration::new(10, 0), false);
    assert!(!path.join(SOON_DELETED).exists()); // Shouldn't exist
    assert!(!path.join(SOON_DELETED).join(TEST_FILE).exists());

    assert!(path.join(TEST_FILE).exists());
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
    saving_service(path, &duration); // File 0

    create_dir_all(path.join(TEST_FILE)).unwrap();
    saving_service(path, &duration); // File with (1)

    create_dir_all(path.join(TEST_FILE)).unwrap();
    saving_service(path, &duration); // File with (2)

    assert!(unknown_path.join(TEST_FILE).exists());
    let second_file = TEST_FILE.to_owned() + "(1)";
    assert!(unknown_path.join(second_file).exists());
    let third_file = TEST_FILE.to_owned() + "(2)";
    assert!(unknown_path.join(third_file).exists())
}

#[test]
fn correct_renaming_cleanup() {
    let path = tempdir_in(".").unwrap();
    let path = path.path();
    let soon_deleted = path.join(SOON_DELETED);
    create_dir_all(path.join(TEST_FILE)).unwrap();

    let duration = Duration::ZERO;
    cleanup_service(&vec![path], &duration, &duration, false); // File 0

    create_dir_all(path.join(TEST_FILE)).unwrap();
    cleanup_service(&vec![path], &duration, &duration, false); // File 1

    create_dir_all(path.join(TEST_FILE)).unwrap();
    cleanup_service(&vec![path], &duration, &duration, false); // File 2

    assert!(soon_deleted.join(TEST_FILE).exists());
    let second_file = TEST_FILE.to_owned() + "(1)";
    assert!(soon_deleted.join(second_file).exists());
    let third_file = TEST_FILE.to_owned() + "(2)";
    assert!(soon_deleted.join(third_file).exists())
}
// ------------------------------------------------------------------------------------------------
// Deleting cleanup tests
// ------------------------------------------------------------------------------------------------
#[test]
fn dont_delete_by_both_times() {
    let path = tempdir_in(".").unwrap();
    let path = path.path();
    let probe = path.join(SOON_DELETED).join(TEST_FILE);
    create_dir_all(&probe).unwrap();

    assert!(probe.exists());

    cleanup_service(
        &vec![path],
        &Duration::new(5, 0),
        &Duration::new(5, 0),
        true,
    );

    assert!(probe.exists());
}

#[test]
fn delete_by_both_times_zero() {
    let path = tempdir_in(".").unwrap();
    let path = path.path();
    let probe = path.join(SOON_DELETED).join(TEST_FILE);
    create_dir_all(&probe).unwrap();

    assert!(probe.exists());

    cleanup_service(&vec![path], &Duration::ZERO, &Duration::ZERO, true);

    assert!(!probe.exists());
    assert!(probe.parent().unwrap().exists());
}

#[test]
fn delete_by_grace_time_zero() {
    let path = tempdir_in(".").unwrap();
    let path = path.path();
    let probe = path.join(SOON_DELETED).join(TEST_FILE);
    create_dir_all(&probe).unwrap();

    assert!(probe.exists());

    cleanup_service(
        &vec![path],
        &Duration::from_millis(1),
        &Duration::ZERO,
        true,
    );

    assert!(probe.exists());

    sleep(Duration::from_millis(2));
    cleanup_service(
        &vec![path],
        &Duration::from_millis(1),
        &Duration::ZERO,
        true,
    );

    assert!(!probe.exists());
    assert!(probe.parent().unwrap().exists());
}

#[test]
fn delete_by_span_time_zero() {
    let path = tempdir_in(".").unwrap();
    let path = path.path();
    let probe = path.join(SOON_DELETED).join(TEST_FILE);
    create_dir_all(&probe).unwrap();

    assert!(probe.exists());

    cleanup_service(
        &vec![path],
        &Duration::ZERO,
        &Duration::from_millis(1),
        true,
    );

    assert!(probe.exists());

    sleep(Duration::from_millis(2));
    cleanup_service(
        &vec![path],
        &Duration::ZERO,
        &Duration::from_millis(1),
        true,
    );

    assert!(!probe.exists());
    assert!(probe.parent().unwrap().exists());
}

#[test]
fn delete_by_summatory_times() {
    let path = tempdir_in(".").unwrap();
    let path = path.path();
    let probe = path.join(SOON_DELETED).join(TEST_FILE);
    create_dir_all(&probe).unwrap();

    assert!(probe.exists());

    cleanup_service(
        &vec![path],
        &Duration::from_millis(1),
        &Duration::from_millis(1),
        true,
    );
    assert!(probe.exists()); // May exist due to not wait

    sleep(Duration::from_millis(3));
    cleanup_service(
        &vec![path],
        &Duration::from_millis(1),
        &Duration::from_millis(1),
        true,
    );

    assert!(!probe.exists()); // May NOT exist due to the wait
    assert!(probe.parent().unwrap().exists());
}
