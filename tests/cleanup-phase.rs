use std::{
    fs::{create_dir, create_dir_all, read_dir},
    time::Duration,
};

use floord::{
    SOON_DELETED,
    services::{cleanup_service, reports::Report},
};
use tempfile::tempdir_in;

const TEST_FILE: &str = "CLEANUP_TEST";

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

    let report = cleanup_service(&vec![path], &span, &Duration::new(10, 0), false, false).unwrap();

    let read_dir = std::fs::read_dir(path).unwrap();
    let size2 = read_dir.count();
    assert_ne!(size, size2);
    assert_eq!(1, size2);
    assert!(path.join(SOON_DELETED).exists());

    assert_eq!(
        report,
        Report {
            saved: Default::default(),
            cleaned: 2,
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
}

#[test]
fn move_directory_to_soon_deleted() {
    let tmp = tempdir_in(".").unwrap();
    let span = Duration::ZERO;

    let path = tmp.path();
    create_dir(path.join(TEST_FILE)).unwrap();
    let report = cleanup_service(&vec![path], &span, &Duration::new(10, 0), false, false).unwrap();
    assert!(path.join(SOON_DELETED).exists());

    let read_dir = read_dir(path.join(SOON_DELETED)).unwrap();
    let size = read_dir.count();
    assert_eq!(1, size); // Assert that cleanup doesn't create duplicates

    assert!(path.join(SOON_DELETED).join(TEST_FILE).exists());

    assert_eq!(
        report,
        Report {
            saved: Default::default(),
            cleaned: 1,
            deleted: Default::default(),
            skipped: Default::default(),
            failed: Default::default()
        }
    );
}

#[test]
fn dont_move_directory_to_soon_deleted() {
    let tmp = tempdir_in(".").unwrap();
    let span = Duration::new(10, 0);

    let path = tmp.path();
    create_dir(path.join(TEST_FILE)).unwrap();
    let report = cleanup_service(&vec![path], &span, &Duration::new(10, 0), false, false).unwrap();
    assert!(!path.join(SOON_DELETED).exists()); // Shouldn't exist
    assert!(!path.join(SOON_DELETED).join(TEST_FILE).exists());

    assert!(path.join(TEST_FILE).exists());

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
}

//-------------------------------------------------------------------------------------------------
//      PATH RENAMING TESTS
//-------------------------------------------------------------------------------------------------

#[test]
fn correct_renaming_cleanup() {
    let path = tempdir_in(".").unwrap();
    let path = path.path();
    let soon_deleted = path.join(SOON_DELETED);
    create_dir_all(path.join(TEST_FILE)).unwrap();

    let duration = Duration::ZERO;

    cleanup_service(&vec![path], &duration, &duration, false, false).unwrap(); // File 0

    create_dir_all(path.join(TEST_FILE)).unwrap();
    cleanup_service(&vec![path], &duration, &duration, false, false).unwrap(); // File 1

    create_dir_all(path.join(TEST_FILE)).unwrap();
    cleanup_service(&vec![path], &duration, &duration, false, false).unwrap(); // File 2

    assert!(soon_deleted.join(TEST_FILE).exists());
    let second_file = TEST_FILE.to_owned() + "(1)";
    assert!(soon_deleted.join(second_file).exists());
    let third_file = TEST_FILE.to_owned() + "(2)";
    assert!(soon_deleted.join(third_file).exists())
}
