use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_hash(path: PathBuf) -> Value {
    if !path.exists() {
        panic!("Path does not exist");
    }
    if path.is_file() {
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        let hash = get_file_hash(path);
        json!({
            filename: {
                "hash": hash
            }
        })
    } else if path.is_dir() {
        let dirname = path.file_name().unwrap().to_string_lossy().to_string();
        let hash = get_directory_hash(&path);
        let children = get_directory_children(&path);
        json!({
            dirname: {
                "hash": hash,
                "child": children
            }
        })
    } else {
        panic!("Path is neither a file nor a directory");
    }
}

fn get_file_hash(path: PathBuf) -> String {
    let file = File::open(path).expect("Failed to open file for hashing");
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];
    loop {
        let bytes_read = reader
            .read(&mut buffer)
            .expect("Failed to read file for hashing");
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    let hash_result = hasher.finalize();
    format!("{:x}", hash_result)
}

fn get_directory_hash(path: &PathBuf) -> String {
    let mut combined_hash = String::new();
    for entry in WalkDir::new(path) {
        let entry = entry.expect("Failed to read directory entry");
        if entry.path().is_file() {
            combined_hash.push_str(&get_file_hash(entry.path().to_path_buf()));
        }
    }
    format!("{:x}", sha2::Sha256::digest(combined_hash.as_bytes()))
}

fn get_directory_children(path: &PathBuf) -> Vec<Value> {
    let mut children = Vec::new();

    for entry in std::fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");
        let entry_path = entry.path();

        if entry_path.is_file() {
            let filename = entry_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let hash = get_file_hash(entry_path);
            children.push(json!({
                filename: {
                    "hash": hash
                }
            }));
        } else if entry_path.is_dir() {
            let dirname = entry_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let hash = get_directory_hash(&entry_path);
            let sub_children = get_directory_children(&entry_path);
            children.push(json!({
                dirname: {
                    "hash": hash,
                    "child": sub_children
                }
            }));
        }
    }

    children
}
