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
