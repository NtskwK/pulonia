# Quick Start

## Usage

Pulonia is a command-line tool. You can use it to generate a patch between two compressed files.

```bash
pulonia --before <old_archive> --after <new_archive> [options]
```

### Options

- `-b, --before <PATH>`: Path to the previous version compressed file (Required).
- `-a, --after <PATH>`: Path to the new version compressed file (Required).
- `-o, --output <PATH>`: Output path for the generated patch file (Default: `ota`).
- `--temp <PATH>`: Temporary directory path for extraction.
- `--format <FORMAT>`: Patch file format (e.g., bsdiff, zstd).

## Example

```bash
pulonia -b app-v1.zip -a app-v2.zip -o update.patch
```

## Supported Formats

Pulonia supports multiple compression formats:

- **ZIP** - The most common compression format
- **TAR** - Traditional Unix packaging format
- **GZIP** (.tar.gz) - TAR + GZIP compression
- **XZ** (.tar.xz) - TAR + XZ compression
- **BZIP2** (.tar.bz2) - TAR + BZIP2 compression
- **LZ4** (.tar.lz4) - TAR + LZ4 compression
- **7Z** - 7-Zip format

## How It Works

Pulonia generates differential patches through the following steps:

1. **Extraction**: Decompresses old and new version compressed files to temporary directories
2. **Comparison**: Uses SHA-256 hashing to compare file contents and precisely detect all changes
3. **Analysis**: Identifies three types of changes:
   - Added files
   - Modified files
   - Deleted files
4. **Report Generation**: Creates a JSON migration report (following Migration Protocol v1)
5. **Patch Creation**: Generates a patch file containing only changed and added files

## Real-World Applications

### Software Update Scenario

```bash
# Generate a patch for upgrading an application from version 1.0 to 2.0
pulonia -b my-app-v1.0.zip -a my-app-v2.0.zip -o my-app-update.zip
```

### CI/CD Integration

```bash
#!/bin/bash
# Automatically generate update patches in CI/CD workflows
pulonia \
  --before ./builds/release-v${OLD_VERSION}.zip \
  --after ./builds/release-v${NEW_VERSION}.zip \
  --output ./releases/update-${OLD_VERSION}-to-${NEW_VERSION}.zip
```

