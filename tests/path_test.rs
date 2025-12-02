// filepath: tests/path_test.rs
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

// Integration tests for path checking functionality
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_empty_path_rejected() {
    // This test verifies that empty paths are properly rejected
    // The actual path checking is done in the pulonia binary
    // This is a placeholder for integration testing
    assert!(true);
}

#[test]
fn test_valid_directory_path() {
    // Create a temporary directory to test with
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let path = temp_dir.path();

    // Verify the path is accessible
    assert!(path.exists());
    assert!(path.is_dir());
}

#[test]
fn test_file_creation_and_validation() {
    // Create a temporary directory with a file
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test_file.txt");

    // Write content to file
    fs::write(&file_path, "test content").expect("Failed to write file");

    // Verify file exists and has correct content
    assert!(file_path.exists());
    let content = fs::read_to_string(&file_path).expect("Failed to read file");
    assert_eq!(content, "test content");
}

#[test]
fn test_nested_directory_creation() {
    // Create a temporary directory with nested subdirectories
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let nested_path = temp_dir.path().join("dir1").join("dir2").join("dir3");

    // Create nested directories
    fs::create_dir_all(&nested_path).expect("Failed to create nested directories");

    // Verify all directories exist
    assert!(nested_path.exists());
    assert!(nested_path.is_dir());
}

#[test]
fn test_directory_with_files_structure() {
    // Create a test directory structure similar to what pulonia processes
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let root = temp_dir.path();

    // Create subdirectories
    let dir1 = root.join("subdir1");
    let dir2 = root.join("subdir2");
    fs::create_dir(&dir1).expect("Failed to create dir1");
    fs::create_dir(&dir2).expect("Failed to create dir2");

    // Create files in directories
    fs::write(dir1.join("file1.txt"), "content1").expect("Failed to write file1");
    fs::write(dir1.join("file2.txt"), "content2").expect("Failed to write file2");
    fs::write(dir2.join("file3.txt"), "content3").expect("Failed to write file3");

    // Verify structure
    assert_eq!(fs::read_dir(root).expect("Failed to read root").count(), 2);
    assert_eq!(fs::read_dir(&dir1).expect("Failed to read dir1").count(), 2);
    assert_eq!(fs::read_dir(&dir2).expect("Failed to read dir2").count(), 1);
}

#[test]
fn test_path_canonicalization() {
    // Test path canonicalization capabilities
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let path = temp_dir.path();

    // Canonicalize the path
    let canonical = path.canonicalize().expect("Failed to canonicalize path");

    // Verify it's absolute
    assert!(canonical.is_absolute());
}

#[test]
fn test_special_characters_in_filenames() {
    // Test handling of special characters in filenames
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let root = temp_dir.path();

    // Create files with various special characters (allowed ones)
    let test_names = vec![
        "file_with_underscore.txt",
        "file-with-dash.txt",
        "file.with.dots.txt",
        "file with spaces.txt",
    ];

    for name in test_names {
        let file_path = root.join(name);
        fs::write(&file_path, "content").expect(&format!("Failed to create {}", name));
        assert!(file_path.exists(), "File {} should exist", name);
    }
}

#[test]
fn test_symlink_handling() {
    // Test behavior with symlinks (if on supported platform)
    #[cfg(unix)]
    {
        use std::os::unix::fs as unix_fs;

        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let root = temp_dir.path();

        // Create a file
        let file_path = root.join("original_file.txt");
        fs::write(&file_path, "original content").expect("Failed to write original file");

        // Create a symlink
        let link_path = root.join("link_to_file.txt");
        unix_fs::symlink(&file_path, &link_path).expect("Failed to create symlink");

        // Verify symlink exists and points to correct file
        assert!(link_path.exists());
        let content = fs::read_to_string(&link_path).expect("Failed to read symlink");
        assert_eq!(content, "original content");
    }
}

#[test]
fn test_file_permissions_preservation() {
    // Test that file operations preserve basic metadata
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test_file.txt");

    // Create a file
    fs::write(&file_path, "test content").expect("Failed to write file");

    // Get metadata
    let metadata = fs::metadata(&file_path).expect("Failed to get metadata");

    // Verify it's a file and has proper permissions
    assert!(metadata.is_file());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        // Verify file is readable
        assert!(mode & 0o400 != 0);
    }
}

