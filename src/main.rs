use std::env::{
    consts::{ARCH, OS},
    current_dir,
};

use chrono::Local;
use tklog::info;

mod logging;
use logging::log_init;

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
    info!("system: ", OS);
    info!("arch: ", ARCH);
    info!("----------------------------");
}
