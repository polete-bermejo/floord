use clap::Parser;
use dirs::download_dir;
use floord::saving_service;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

/// Floor daemon - File organizer.
/// Modify its args in '.config/systemd/user/floor-daemon.service'
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory used as inbox [default: Downloads]
    #[arg(short, long)]
    inbox: Option<PathBuf>,

    /// Amount of time in seconds that floord waits for a file to be moved
    /// (watches the modified value)
    #[arg(short, long, default_value_t = 300)]
    delay: u64,

    /// Executes in loop/daemon-mode.
    /// Needed for the execution as a service
    #[arg(long, default_value_t = false)]
    daemon: bool,
}

const DELAY: Duration = Duration::from_millis(25);

fn main() {
    let cli_args = Args::parse();

    let inbox_path = cli_args.inbox.unwrap_or(if let Some(dir) = download_dir() {
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
            sleep(DELAY);
        }
    }
    saving_service(&inbox_path, &delay);
}
