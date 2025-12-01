# Pulonia

Pulonia is a high-performance open-source tool designed to generate differential patches for software updates. It compares two compressed files (archives), identifies changes (additions, modifications, deletions), and generates a structured migration report.

## Features

- **Multi-format Support**: Supports ZIP, TAR, GZIP, XZ, BZIP2, LZ4, 7Z, and more.
- **High Performance**: Built with Rust for speed and memory safety.
- **Smart Detection**: Uses SHA-256 hashing to precisely detect file changes.
- **Structured Output**: Generates a JSON migration report following Migration Protocol v1.
- **Easy CLI**: Simple command-line interface for integration into CI/CD pipelines.

## Usage

```bash
pulonia --before <old_archive> --after <new_archive> [options]
```

### Options

- `-b, --before <PATH>`: Path to the previous version compressed file (Required).
- `-a, --after <PATH>`: Path to the new version compressed file (Required).
- `-o, --output <PATH>`: Output path for the generated patch file (Default: `ota`).
- `--temp <PATH>`: Temporary directory path for extraction.
- `--format <FORMAT>`: Patch file format (e.g., bsdiff, zstd).

## Building from Source

```bash
cargo build --release
```

## License

Apache-2.0
