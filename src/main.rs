use clap::Parser;
use dirs::download_dir;
use floord::service;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

/// To set-up execute: floord_setup
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory used as inbox (Default: Downloads)
    #[arg(short, long)]
    inbox: Option<PathBuf>,

    /// Seconds to wait after a file appears at inbox path
    #[arg(short, long, default_value_t = 300)]
    delay: u64,

    /// Executes in loop
    #[arg(long, default_value_t = false)]
    daemon: bool,
}

const DELAY: Duration = Duration::from_millis(25);

fn main() {
    let cli_args = Args::parse();
    let inbox_path = cli_args.inbox.unwrap_or(match download_dir() {
        Some(dir) => dir,
        None => {
            println!("Default path not found (default: Downloads folder)");
            return;
        }
    });

    if !inbox_path.is_dir() {
        println!("Inbox path does not exist nor is a directory");
        return;
    }

    let delay = Duration::new(cli_args.delay, 0);

    if cli_args.daemon {
        loop {
            if let Err(error) = service(&inbox_path, &delay) {
                eprintln!("Error: {error}")
            }
            sleep(DELAY)
        }
    } else {
        if let Err(error) = service(&inbox_path, &delay) {
            eprintln!("Error: {error}")
        }
    }
}
