// Copyright 2025 natsuu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

pub fn check_path(path_str: &str) -> Result<(), String> {
    if path_str.is_empty() {
        return Err("Path is empty".to_string());
    }

    let os_path = OsStr::new(path_str);
    let path = Path::new(os_path);
    #[cfg(windows)]
    {
        let illegal_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
        if path_str.chars().any(|c| illegal_chars.contains(&c)) {
            return Err("Windows path contains illegal characters".to_string());
        }
        let trimmed = path_str.trim_end();
        if trimmed.ends_with('.') || trimmed.ends_with(' ') {
            if !path.is_absolute() || trimmed != path_str {
                return Err("Windows path cannot end with a space or dot".to_string());
            }
        }
        if let Some(parent) = path.parent() {
            if parent == Path::new("") || parent.as_os_str().is_empty() {
                return Err("Cannot access root directory.".to_string());
            }
        } else if path.is_absolute() && path.components().count() == 1 {
            return Err("Cannot access root directory.".to_string());
        }
    }
    #[cfg(unix)]
    {
        // Unix 仅 NUL 字符（\0）非法，其余字符（包括/）均合法（/ 是路径分隔符）
        if path_str.contains('\0') {
            return Err("Unix 路径含 NUL 字符（\\0）".to_string());
        }
        if path == Path::new("/") {
            return false;
        }
    }

    if let Some(_) = path.canonicalize().ok() {
    } else if !path.is_absolute() && !path.is_relative() {
        return Err("Path is neither absolute nor relative".to_string());
    }

    if !is_safe_path(path_str) {
        return Err("Path is not safe! Pulonia can only access files in the current directory or its subdirectories. But don't worry, it's was designed this way to protect your files.".to_string());
    }

    match fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Path is not a valid file or directory: {}", e)),
    }
}

pub fn is_safe_path(path: &str) -> bool {
    let forbidden_patterns = [
        "..", "~", "//", "\\", "%", "$", "{", "}", "<", ">", "|", "\"",
    ];
    for pattern in forbidden_patterns.iter() {
        if path.contains(pattern) {
            return false;
        }
    }

    let self_dir = env::current_dir().unwrap();
    let path = PathBuf::from(path);
    if !path.starts_with(&self_dir) {
        return false;
    }

    true
}
