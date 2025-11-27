use std::env::{
    consts::{ARCH, OS},
    current_dir,
};

use chrono::Local;
use clap::Parser;
use tempfile::TempDir;
use tklog::info;

mod logging;
use logging::log_init;

use crate::path::check_path;

mod cli;
mod path;

fn main() {
    pulonia_init();
}

fn pulonia_init() {
    const LOG_DIR_NAME: &str = "updater_logs";

    let work_dir = current_dir().unwrap();
    let log_dir = work_dir.join(LOG_DIR_NAME);
    let log_file_available = match std::fs::create_dir_all(&log_dir) {
        Ok(_) => true,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => true,
        Err(e) => {
            eprintln!("Failed to create log directory: {}", e);
            false
        }
    };

    if log_file_available {
        log_init(Some(&log_dir));
    } else {
        log_init(None);
    }

    info!("----------------------------");
    info!("Pulonia started");
    info!("version: ", env!("CARGO_PKG_VERSION"));
    info!("----------------------------");
    info!("time: ", Local::now().format("%Y-%m-%d %H:%M:%S"));
    info!("os: ", OS);
    info!("arch: ", ARCH);
    info!("----------------------------");

    let cli = cli::Cli::parse();

    if cli.after_path.is_empty()
        || cli.before_path.is_empty()
        || cli.temp_dir_path.is_none()
        || cli.output_path.is_none()
    {
        eprintln!("Error: Both current and previous version paths must be provided.");
        return;
    }

    let temp_dir = match cli.temp_dir_path {
        Some(path) => {
            check_path(&path).unwrap_or_else(|err| {
                eprintln!("Invalid temporary directory path: {}", err);
                std::process::exit(1);
            });
            TempDir::new_in(path).unwrap()
        }
        None => TempDir::new().unwrap(),
    };

    check_path(&cli.after_path).unwrap_or_else(|err| {
        eprintln!("Invalid current version path: {}", err);
        std::process::exit(1);
    });

    check_path(&cli.before_path).unwrap_or_else(|err| {
        eprintln!("Invalid previous version path: {}", err);
        std::process::exit(1);
    });

    let format = cli.format.unwrap_or_else(|| "zip".to_string());
    let output_path = cli.output_path.unwrap_or_else(|| "ota".to_string());

    info!("after path: {}", cli.after_path);
    info!("before path: {}", cli.before_path);
    info!("Temporary directory path: {}", temp_dir.path().display());
    info!("Output path: {}", output_path);
    info!("Patch file format: {}", format);
}
