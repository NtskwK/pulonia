# Pulonia

Pulonia is a high-performance differential patch generator for software updates. It compares two compressed archives, identifies changes (additions, modifications, deletions), generates a structured migration report, and creates a patch file containing only the updated files.

## Features

- **Multi-format Support**: Supports ZIP, TAR, GZIP (.tar.gz), XZ (.tar.xz), BZIP2 (.tar.bz2), LZ4 (.tar.lz4), and 7Z archives.
- **High Performance**: Built with Rust for speed and memory safety.
- **Smart Detection**: Uses SHA-256 hashing to precisely detect file changes at any directory depth.
- **Structured Output**:
  - Generates a JSON migration report following Migration Protocol v1
  - Creates a patch archive containing only changed/added files
- **Easy CLI**: Simple command-line interface for integration into CI/CD pipelines.

## Installation

### From Source

```bash
git clone https://github.com/NtskwK/pulonia.git
cd pulonia
cargo build --release
```

The binary will be available at `target/release/pulonia`.

## Usage

### Basic Usage

```bash
pulonia -b <old_archive> -a <new_archive>
```

### With Custom Output

```bash
pulonia -b old_version.zip -a new_version.zip -o update.zip
```

### Specify Output Format

```bash
pulonia -b old_version.7z -a new_version.7z -o update -f zip
```

### Options

- `-b, --before <PATH>`: Path to the previous version compressed file (Required)
- `-a, --after <PATH>`: Path to the new version compressed file (Required)
- `-o, --output <PATH>`: Output path for the generated patch file (Default: `ota`)
- `-f, --format <FORMAT>`: Output patch format: `zip`, `tar`, `gz`, `xz`, `bz2`, `lz4`, or `7z` (Default: inferred from output path or `zip`)
- `--temp <PATH>`: Custom temporary directory path for extraction

### Example

```bash
# Compare two versions and generate a patch
pulonia -b app_v1.0.zip -a app_v1.1.zip -o patch_v1.1.zip

# This will create:
# - patch_v1.1.zip: Contains only the changed/added files
# - migration_YYMMDD_HHMM.json: Detailed change report
```

## Output

Pulonia generates two outputs:

1. **Patch Archive** (e.g., `ota.zip`): Contains only the files that were added or modified
2. **Migration Report** (e.g., `migration_251202_1751.json`): A JSON file with detailed change information:
   ```json
   {
     "version": "1.0",
     "update": {
       "path/to/modified_file.txt": {
         "hash": "abc123..."
       }
     },
     "deleted": ["path/to/deleted_file.txt"]
   }
   ```

## Supported Formats

| Format | Extensions        | Compression |
| ------ | ----------------- | ----------- |
| ZIP    | `.zip`            | Deflate     |
| TAR    | `.tar`            | None        |
| GZIP   | `.tar.gz`, `.tgz` | Gzip        |
| XZ     | `.tar.xz`         | XZ          |
| BZIP2  | `.tar.bz2`        | Bzip2       |
| LZ4    | `.tar.lz4`        | LZ4         |
| 7Z     | `.7z`             | LZMA        |

## How It Works

1. **Extraction**: Both archives are extracted to temporary directories
2. **Hashing**: All files are recursively hashed using SHA-256
3. **Comparison**: File trees are compared to identify changes
4. **Report Generation**: A migration report is created in JSON format
5. **Patch Creation**: Changed/added files are collected into a new archive

## Building from Source

```bash
cargo build --release
```

## Running Tests

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
