use clap::Parser;
use dirs::{audio_dir, document_dir, download_dir, picture_dir, video_dir};
use floord::{cleanup_service, saving_service};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

const O_LOOP_INTERVAL: Duration = Duration::from_millis(25);
const C_LOOP_INTERVAL: Duration = Duration::from_mins(5);

/// Floor daemon - File organizer and cleaner.
/// Modify its args in '~/.config/systemd/user/floor-daemon.service' and reload.
#[derive(Parser)]
#[command(version, about, long_about = None, group(
        clap::ArgGroup::new("services")
            .args(["organizer", "cleanup"])
            .required(true)
            .multiple(true)
    ))]
pub struct Args {
    /// Enables the inbox organizer function
    #[arg(long, help_heading = "Inbox organizer", default_value_t = false)]
    organizer: bool,

    /// Directory used as inbox [default: Downloads]
    #[arg(long, help_heading = "Inbox organizer", requires = "organizer")]
    inbox: Option<PathBuf>,

    /// Amount of time in seconds that floord waits for a file to be moved
    /// (watches the modified value)
    #[arg(
        long,
        help_heading = "Inbox organizer",
        requires = "organizer",
        default_value_t = 300
    )]
    delay: u64,

    /// Enables the cleanup function
    #[arg(long, help_heading = "Cleanup", default_value_t = false)]
    cleanup: bool,

    /// Paths to execute cleanup
    #[arg(long, help_heading = "Cleanup", requires = "cleanup")]
    paths: Vec<PathBuf>,

    /// Time in days till a file goes to the SoonDeleted folder
    #[arg(
        long,
        help_heading = "Cleanup",
        requires = "cleanup",
        default_value_t = 30
    )]
    span: u64,

    /// Time in days until a file gets permanently DELETED from the SoonDeleted folder (currently not working)
    #[arg(
        long,
        help_heading = "Cleanup",
        requires = "cleanup",
        default_value_t = 7
    )]
    grace: u64,

    /// Executes in loop/daemon-mode.
    /// Needed for the execution as a service
    #[arg(long, help_heading = "Daemon", default_value_t = false)]
    daemon: bool,
}

fn run_organizer(cli_args: &Args) {
    let inbox_path = cli_args
        .inbox
        .clone()
        .unwrap_or(if let Some(dir) = download_dir() {
            dir
        } else {
            println!("Default path not found (default: Downloads folder)");
            return;
        });

    if !inbox_path.is_dir() {
        println!("Inbox path does not exist nor is a directory");
        return;
    }

    let delay = Duration::new(cli_args.delay, 0);

    if cli_args.daemon {
        loop {
            saving_service(&inbox_path, &delay);
            sleep(O_LOOP_INTERVAL);
        }
    }
    saving_service(&inbox_path, &delay);
    println!("Organization done!")
}

fn run_cleanup(cli_args: &Args) {
    let mut paths = cli_args.paths.clone();
    if paths.is_empty() {
        if let Some(got) = document_dir() {
            paths.push(got);
        };
        if let Some(got) = audio_dir() {
            paths.push(got);
        };
        if let Some(got) = video_dir() {
            paths.push(got);
        };
        if let Some(got) = picture_dir() {
            paths.push(got);
        };
    }
    if paths.is_empty() {
        eprintln!("Paths empty");
        return;
    }
    let span = Duration::from_hours(24 * cli_args.span);
    let grace = Duration::from_hours(24 * cli_args.grace);
    if cli_args.daemon {
        loop {
            cleanup_service(&paths, &span, &grace);
            sleep(C_LOOP_INTERVAL);
        }
    }
    cleanup_service(&paths, &span, &grace);
    println!("Cleanup done!")
}

fn main() {
    let cli_args = Arc::new(Args::parse());

    let organizer_args = Arc::clone(&cli_args);
    let cleanup_args = Arc::clone(&cli_args);

    let organizer = thread::spawn(move || {
        if organizer_args.organizer {
            run_organizer(&organizer_args);
        }
    });

    let cleanup = thread::spawn(move || {
        if cleanup_args.cleanup {
            run_cleanup(&cleanup_args)
        }
    });

    organizer.join().expect("organized crashed");
    cleanup.join().expect("cleanup crashed");
}
