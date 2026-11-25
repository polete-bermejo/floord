use std::process::{Command, ExitStatus};

use tempfile::tempdir_in;

const ORGANIZER: &str = "--organizer";
const INBOX: &str = "--inbox";
const DELAY: &str = "--delay";

const CLEANUP: &str = "--cleanup";
const PATHS: &str = "--paths";
const SPAN: &str = "--span";
const GRACE: &str = "--grace";
const ALLOW_DELETION: &str = "--allow-deletion";

const DAEMON: &str = "--daemon";
const NOTIFY: &str = "--notify";

const CONFIG: &str = "--config";

// ==============================================
//      General
// ==============================================

#[test]
#[ignore = "CI does not have any default folders so this would fail"]
fn organizer_solo() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();

    let status = Command::new(executable).arg(ORGANIZER).status().unwrap();
    assert!(ExitStatus::success(&status));
}

#[test]
#[ignore = "CI does not have any default folders so this would fail"]
fn cleanup_solo() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();

    let status = Command::new(executable).arg(CLEANUP).status().unwrap();
    assert!(ExitStatus::success(&status));
}

#[test]
#[ignore = "CI does not have any default folders so this would fail"]
fn config_solo() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();

    let status = Command::new(executable)
        .arg(CONFIG)
        .arg("config_template.toml")
        .status()
        .unwrap();
    assert!(ExitStatus::success(&status));
}

// ==============================================
//      Organizer
// ==============================================

#[test]
fn organizer_inbox() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();
    let tmp = tempdir_in(".").unwrap();

    let status = Command::new(executable)
        .arg(ORGANIZER)
        .arg(INBOX)
        .arg(tmp.path())
        .status()
        .unwrap();
    assert!(ExitStatus::success(&status));
}

#[test]
fn organizer_delay() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();
    let tmp = tempdir_in(".").unwrap();

    let status = Command::new(executable)
        .arg(ORGANIZER)
        .arg(INBOX)
        .arg(tmp.path())
        .arg(DELAY)
        .arg("99999999")
        .status()
        .unwrap();
    assert!(ExitStatus::success(&status));
}

#[test]
#[ignore = "CI does not have any default folders so this would fail"]
fn organizer_inbox_error() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();
    let tmp = tempdir_in(".").unwrap();
    let wrong_path = tmp.path().to_path_buf().join("something");

    let output = Command::new(executable)
        .arg(ORGANIZER)
        .arg(INBOX)
        .arg(wrong_path)
        .output()
        .unwrap();
    let output = String::from_utf8_lossy_owned(output.stdout);
    assert_eq!(
        output.trim(),
        "Inbox path does not exist nor is a directory"
    );
}

#[test]
fn organizer_incompatible_flag() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();

    let status = Command::new(executable)
        .arg(ORGANIZER)
        .arg(PATHS)
        .status()
        .unwrap();
    assert!(!ExitStatus::success(&status));
}

#[test]
fn organizer_incompatible_config_flag() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();

    let status = Command::new(executable)
        .arg(ORGANIZER)
        .arg(CONFIG)
        .status()
        .unwrap();
    assert!(!ExitStatus::success(&status));
}

#[test]
fn organizer_incompatible_paths_flag() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();

    let status = Command::new(executable)
        .arg(ORGANIZER)
        .arg(PATHS)
        .status()
        .unwrap();
    assert!(!ExitStatus::success(&status));
}

// ==============================================
//      Cleanup
// ==============================================

#[test]
fn cleanup_paths() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();
    let tmp = tempdir_in(".").unwrap();

    let status = Command::new(executable)
        .arg(CLEANUP)
        .arg(PATHS)
        .arg(tmp.path())
        .status()
        .unwrap();
    assert!(ExitStatus::success(&status));
}

#[test]
fn cleanup_span() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();
    let tmp = tempdir_in(".").unwrap();

    let status = Command::new(executable)
        .arg(CLEANUP)
        .arg(PATHS)
        .arg(tmp.path())
        .arg(SPAN)
        .arg("0")
        .status()
        .unwrap();
    assert!(ExitStatus::success(&status));
}

#[test]
fn cleanup_grace() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();
    let tmp = tempdir_in(".").unwrap();

    let status = Command::new(executable)
        .arg(CLEANUP)
        .arg(PATHS)
        .arg(tmp.path())
        .arg(GRACE)
        .arg("0")
        .status()
        .unwrap();
    assert!(ExitStatus::success(&status));
}

#[test]
fn cleanup_span_grace() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();
    let tmp = tempdir_in(".").unwrap();

    let status = Command::new(executable)
        .arg(CLEANUP)
        .arg(PATHS)
        .arg(tmp.path())
        .arg(GRACE)
        .arg("0")
        .arg(SPAN)
        .arg("0")
        .status()
        .unwrap();
    assert!(ExitStatus::success(&status));
}

#[test]
#[cfg(unix)]
fn cleanup_paths_error() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();

    let output = Command::new(executable)
        .arg(CLEANUP)
        .arg(PATHS)
        .arg("tmp")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy_owned(output.stderr);
    assert_eq!(
        output.trim(),
        "Cleanup returned an error: No such file or directory (os error 2)"
    );
}

#[test]
fn cleanup_config_error() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();

    let status = Command::new(executable)
        .arg(CLEANUP)
        .arg(CONFIG)
        .status()
        .unwrap();
    assert!(!ExitStatus::success(&status));
}

#[test]
fn cleanup_incompatible_flag() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();

    let status = Command::new(executable)
        .arg(CLEANUP)
        .arg(DELAY)
        .status()
        .unwrap();
    assert!(!ExitStatus::success(&status));
}

// ==============================================
//      Config
// ==============================================

#[test]
fn config_flags_error() {
    let executable = std::env::var("CARGO_BIN_EXE_floord").unwrap();

    let get_status = |arg: &str| {
        Command::new(&executable)
            .arg(CONFIG)
            .arg("config_template.toml")
            .arg(arg)
            .status()
            .unwrap()
    };

    let failures = [
        DAEMON,
        NOTIFY,
        ORGANIZER,
        INBOX,
        DELAY,
        CLEANUP,
        PATHS,
        SPAN,
        GRACE,
        ALLOW_DELETION,
    ];

    for arg in failures {
        assert!(
            !get_status(arg).success(),
            "Expected failure for flag: {arg}"
        );
    }
}
