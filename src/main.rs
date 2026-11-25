use clap::{CommandFactory, FromArgMatches, Parser};
use dirs::{audio_dir, document_dir, download_dir, picture_dir, video_dir};
use floord::services::{cleanup_service, saving_service};
use log::{error, info, warn};
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

const O_LOOP_INTERVAL: Duration = Duration::from_secs(5);
const C_LOOP_INTERVAL: Duration = Duration::from_mins(5);

#[derive(Parser, Deserialize)]
#[command(version, about, long_about = None, group(
        clap::ArgGroup::new("services")
            .args(["organizer", "cleanup", "config"])
            .required(true)
            .multiple(true)
    ))]
#[allow(clippy::struct_excessive_bools)]
struct Args {
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

    /// Time in days until a file goes to the `SoonDeleted` folder
    #[arg(
        long,
        help_heading = "Cleanup",
        requires = "cleanup",
        default_value_t = 30
    )]
    span: u64,

    /// Time in days until a file gets permanently DELETED from the `SoonDeleted` folder
    #[arg(
        long,
        help_heading = "Cleanup",
        requires = "cleanup",
        default_value_t = 7
    )]
    grace: u64,

    /// Allow daemon to execute deletion actions over the `SoonDeleted` folders according to the grace and span times
    #[arg(
        long,
        help_heading = "Cleanup",
        requires = "cleanup",
        default_value_t = false
    )]
    allow_deletion: bool,

    /// Executes in loop/daemon-mode.
    /// Needed for the execution as a service
    #[arg(long, help_heading = "Daemon", default_value_t = false)]
    daemon: bool,

    /// Enable notifications
    #[arg(long, help_heading = "Daemon", default_value_t = false)]
    notify: bool,

    /// Path to configuration file
    #[arg(long, help_heading = "Configuration", conflicts_with_all ([
        "organizer",
        "inbox",
        "delay",
        "cleanup",
        "span",
        "grace",
        "allow_deletion",
        "notify",
        "paths",
        "daemon"
    ]))]
    config: Option<PathBuf>,
}

impl Default for Args {
    fn default() -> Self {
        warn!("Running default parameters");
        Self {
            organizer: true,
            inbox: None,
            delay: 300,
            cleanup: true,
            paths: vec![],
            span: 30,
            grace: 5,
            allow_deletion: false,
            daemon: true,
            notify: true,
            config: None,
        }
    }
}

fn run_organizer(cli_args: &Args) {
    info!("Executing organizer!");
    let inbox_path = cli_args
        .inbox
        .clone()
        .unwrap_or(if let Some(dir) = download_dir() {
            dir
        } else {
            warn!("Default path not found (default: Downloads folder)");
            return;
        });

    if !inbox_path.is_dir() {
        error!("Inbox path does not exist nor is a directory");
        return;
    }

    let delay = Duration::new(cli_args.delay, 0);

    if cli_args.daemon {
        loop {
            match saving_service(&inbox_path, &delay, cli_args.notify) {
                Ok(report) => {
                    info!(
                        "Organized: {} saved, {} skipped, {} failed",
                        report.saved,
                        report.skipped,
                        report.failed.len()
                    );
                    for failure in &report.failed {
                        error!(
                            "failure in: {}, error: {}",
                            failure.path.display(),
                            failure.error
                        );
                    }
                }
                Err(e) => error!("organizer fatal error: {e}"),
            }
            sleep(O_LOOP_INTERVAL);
        }
    }
    match saving_service(&inbox_path, &delay, cli_args.notify) {
        Ok(report) => {
            info!(
                "Organized: {} saved, {} skipped, {} failed",
                report.saved,
                report.skipped,
                report.failed.len()
            );
        }
        Err(e) => error!("Organizer returned an error: {e}"),
    }
}

fn run_cleanup(cli_args: &Args) {
    info!("Executing cleanup!");
    let mut paths = cli_args.paths.clone();
    if paths.is_empty() {
        if let Some(got) = document_dir() {
            paths.push(got);
        }
        if let Some(got) = audio_dir() {
            paths.push(got);
        }
        if let Some(got) = video_dir() {
            paths.push(got);
        }
        if let Some(got) = picture_dir() {
            paths.push(got);
        }
    }
    if paths.is_empty() {
        error!("Paths empty, couldn't get any default path for cleanup service");
        return;
    }
    let paths = paths.iter().map(PathBuf::as_path).collect();
    let span = Duration::from_hours(24 * cli_args.span);
    let grace = Duration::from_hours(24 * cli_args.grace);
    if cli_args.daemon {
        loop {
            match cleanup_service(
                &paths,
                &span,
                &grace,
                cli_args.allow_deletion,
                cli_args.notify,
            ) {
                Ok(report) => {
                    info!(
                        "Cleaned: {} cleaned, {} deleted, {} skipped, {} failed",
                        report.cleaned,
                        report.deleted,
                        report.skipped,
                        report.failed.len()
                    );
                    for failure in &report.failed {
                        error!(
                            "failure in: {}, error: {}",
                            failure.path.display(),
                            failure.error
                        );
                    }
                }
                Err(e) => error!("cleanup fatal error: {e}"),
            }
            sleep(C_LOOP_INTERVAL);
        }
    }
    match cleanup_service(
        &paths,
        &span,
        &grace,
        cli_args.allow_deletion,
        cli_args.notify,
    ) {
        Ok(report) => {
            info!(
                "Cleaned: {} cleaned, {} deleted, {} skipped, {} failed",
                report.cleaned,
                report.deleted,
                report.skipped,
                report.failed.len()
            );
        }
        Err(e) => error!("Cleanup returned an error: {e}"),
    }
    info!("Cleanup done!");
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let mut cmd = Args::command();
    if let Some(dir) = dirs::config_dir() {
        cmd = cmd.about(format!(
            "Floor daemon - File organizer and cleaner.\n\
             Modify its values in '{}' and reload.",
            dir.join("floord/config.toml").display()
        ));
    }
    let Ok(arguments) = Args::from_arg_matches(&cmd.get_matches()) else {
        error!("Couldn't get arguments");
        return;
    };

    let cli_args: Arc<Args> = match arguments.config {
        Some(file_path) => {
            let content = std::fs::read_to_string(file_path).unwrap_or_default();
            let toml_args: Args = toml::from_str(&content).unwrap_or_default();
            Arc::new(toml_args)
        }
        None => Arc::new(arguments),
    };

    let organizer_args = Arc::clone(&cli_args);
    let cleanup_args = Arc::clone(&cli_args);

    let organizer = thread::spawn(move || {
        if organizer_args.organizer {
            run_organizer(&organizer_args);
        }
    });

    let cleanup = thread::spawn(move || {
        if cleanup_args.cleanup {
            run_cleanup(&cleanup_args);
        }
    });

    organizer.join().expect("organized crashed");
    cleanup.join().expect("cleanup crashed");
}
