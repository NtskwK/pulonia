use predicates::prelude::*;
use std::fs::{self, File};
use std::path::Path;
use tempfile::TempDir;
use zip::write::FileOptions;

fn create_zip(src_dir: &Path, dst_file: &Path) -> std::io::Result<()> {
    let file = File::create(dst_file)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for entry in walkdir::WalkDir::new(src_dir) {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .strip_prefix(src_dir)
            .unwrap()
            .to_str()
            .unwrap()
            .replace("\\", "/");

        if path.is_file() {
            zip.start_file(&name, options)?;
            let mut f = File::open(path)?;
            std::io::copy(&mut f, &mut zip)?;
        } else if !name.is_empty() {
            let dir_name = if name.ends_with('/') {
                name
            } else {
                format!("{}/", name)
            };
            zip.add_directory(&dir_name, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

#[test]
fn test_generate_migration_report() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let root = temp_dir.path();

    // Create "before" directory
    let before_dir = root.join("before");
    fs::create_dir(&before_dir)?;
    fs::write(before_dir.join("file1.txt"), "content A")?;
    fs::write(before_dir.join("file2.txt"), "content B")?;

    // Create "after" directory
    let after_dir = root.join("after");
    fs::create_dir(&after_dir)?;
    fs::write(after_dir.join("file1.txt"), "content A")?; // Unchanged
    fs::write(after_dir.join("file2.txt"), "content C")?; // Modified
    fs::write(after_dir.join("file3.txt"), "content D")?; // New

    // Compress
    let before_zip = root.join("before.zip");
    create_zip(&before_dir, &before_zip)?;

    let after_zip = root.join("after.zip");
    create_zip(&after_dir, &after_zip)?;

    // Run pulonia
    let cmd = std::process::Command::new(assert_cmd::cargo::cargo_bin!("pulonia"));
    let mut assert = assert_cmd::Command::from_std(cmd);

    assert
        .arg("--before")
        .arg(&before_zip)
        .arg("--after")
        .arg(&after_zip)
        .arg("--output")
        .arg("ota")
        .assert()
        .success()
        .stdout(predicate::str::contains("Migration report saved to:"));

    // Find migration file
    let current_dir = std::env::current_dir()?;
    let entries = fs::read_dir(&current_dir)?;
    let mut migration_file = None;
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("migration_") && name.ends_with(".json") {
            migration_file = Some(entry.path());
            break;
        }
    }

    assert!(migration_file.is_some(), "Migration file not found");
    let migration_file = migration_file.unwrap();

    // Verify content
    let content = fs::read_to_string(&migration_file)?;
    let json: serde_json::Value = serde_json::from_str(&content)?;

    assert_eq!(json["version"], "1.0");

    let update = &json["update"];
    assert!(update.get("file2.txt").is_some());
    assert!(update.get("file3.txt").is_some());
    assert!(update.get("file1.txt").is_none());

    let deleted = &json["deleted"];
    assert!(deleted.as_array().unwrap().is_empty());

    // Clean up migration file
    fs::remove_file(migration_file)?;

    Ok(())
}
