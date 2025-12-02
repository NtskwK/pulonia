use std::{
    env::{
        consts::{ARCH, OS},
        current_dir,
    },
    path::Path,
};

use chrono::Local;
use clap::Parser;
use tempfile::TempDir;
use tklog::{error, info, warn};

mod logging;
use logging::log_init;

mod cli;
use cli::Cli;
mod compress;
use compress::decompress;

mod diff;
mod migration;
mod path;

use path::check_path;

use crate::diff::get_hash;
use crate::migration::generate_migration;

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

    let cli = Cli::parse();

    if cli.after_path.is_empty() || cli.before_path.is_empty() {
        eprintln!("Error: Both current and previous version paths must be provided.");
        return;
    }

    let temp_dir = match cli.temp_dir_path {
        Some(path) => {
            check_path(&path).unwrap_or_else(|err| {
                error!("Invalid temporary directory path:", err);
                std::process::exit(1);
            });
            TempDir::new_in(path).unwrap()
        }
        None => TempDir::new().unwrap(),
    };

    check_path(&cli.after_path).unwrap_or_else(|err| {
        error!("Invalid current version path:", err);
        std::process::exit(1);
    });

    check_path(&cli.before_path).unwrap_or_else(|err| {
        error!("Invalid previous version path:", err);
        std::process::exit(1);
    });

    let output_path = cli.output_path.unwrap_or_else(|| "ota".to_string());

    // 先检查用户是否指定了格式
    let format_specified = cli.format.is_some();

    let format = cli.format.unwrap_or_else(|| {
        Path::new(&output_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "zip".to_string())
    });

    // 确保输出路径包含正确的扩展名
    // 如果用户指定了 format，则移除 output_path 中的扩展名（如果有），然后添加正确的扩展名
    let output_path = if format_specified {
        // 用户明确指定了格式，移除原有扩展名并使用新格式
        let path_without_ext = Path::new(&output_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&output_path);

        // 如果原路径包含目录部分，需要保留
        if let Some(parent) = Path::new(&output_path).parent() {
            if parent.as_os_str().is_empty() {
                format!("{}.{}", path_without_ext, format)
            } else {
                format!("{}/{}.{}", parent.display(), path_without_ext, format)
            }
        } else {
            format!("{}.{}", path_without_ext, format)
        }
    } else if Path::new(&output_path).extension().is_none() {
        // 用户没有指定格式，且输出路径没有扩展名，添加默认扩展名
        format!("{}.{}", output_path, format)
    } else {
        // 用户没有指定格式，但输出路径有扩展名，直接使用
        output_path
    };

    info!("after path:", cli.after_path);
    info!("before path:", cli.before_path);
    info!("Temporary directory path:", temp_dir.path().display());
    info!("Output path:", output_path);
    info!("Patch file format:", format);

    let decompressed_after_path = Path::join(temp_dir.path(), "after_decompressed");
    let decompressed_before_path = Path::join(temp_dir.path(), "before_decompressed");
    decompress(&cli.after_path, decompressed_after_path.to_str().unwrap()).unwrap();
    decompress(&cli.before_path, decompressed_before_path.to_str().unwrap()).unwrap();

    let before_hash = get_hash(decompressed_before_path);
    let after_hash = get_hash(decompressed_after_path.clone());

    let before_inner = before_hash.as_object().unwrap().values().next().unwrap();
    let after_inner = after_hash.as_object().unwrap().values().next().unwrap();

    if before_inner == after_inner {
        info!("The two files are identical.");
        return;
    }
    warn!("The hash of the two files is different.");
    warn!("before hash:", before_inner);
    warn!("after hash:", after_inner);

    // 生成迁移记录文件
    let changes = generate_migration(before_inner, after_inner);

    // 保存迁移记录文件
    let migration_file_path = format!("migration_{}.json", Local::now().format("%y%m%d_%H%M"));
    let json_string = serde_json::to_string_pretty(&changes).unwrap();
    match std::fs::write(&migration_file_path, json_string) {
        Ok(_) => {
            info!("Migration report saved to:", migration_file_path);
        }
        Err(e) => {
            error!("Failed to save migration report:", e);
        }
    }

    // 获取更新的文件列表并打包
    let updated_files = migration::get_updated_files(before_inner, after_inner);
    if !updated_files.is_empty() {
        let patch_temp_dir = temp_dir.path().join("patch_temp");
        if let Err(e) = std::fs::create_dir_all(&patch_temp_dir) {
            error!("Failed to create patch temp directory:", e);
            return;
        }

        for file_path in updated_files {
            let src_path = decompressed_after_path.join(&file_path);
            let dest_path = patch_temp_dir.join(&file_path);

            if let Some(parent) = dest_path.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    error!("Failed to create directory:", parent.display(), e);
                    continue;
                }
            }

            if let Err(e) = std::fs::copy(&src_path, &dest_path) {
                error!(
                    "Failed to copy file:",
                    src_path.display(),
                    "to",
                    dest_path.display(),
                    e
                );
            }
        }

        match compress::compress(patch_temp_dir.to_str().unwrap(), &output_path, &format) {
            Ok(_) => {
                info!("Patch file created successfully at:", output_path);
            }
            Err(e) => {
                error!("Failed to create patch file:", e);
            }
        }
    } else {
        info!("No files updated, skipping patch generation.");
    }
}
