use std::fs::File;
use std::io::{BufReader, Seek, Write};
use std::path::Path;
use thiserror::Error;
use zip::write::FileOptions;

#[derive(Debug, Error)]
pub enum DecompressError {
    #[error("Unsupported compression format: {0}")]
    UnsupportedFormat(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),
    #[error("Zip archive error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("7z error: {0}")]
    SevenZ(#[from] sevenz_rust::Error),
}

fn get_file_type(path: &Path) -> Option<String> {
    let filename = path.file_name()?.to_string_lossy();
    if filename.ends_with(".tar.gz") {
        return Some("tar.gz".to_string());
    }
    if filename.ends_with(".tar.xz") {
        return Some("tar.xz".to_string());
    }
    if filename.ends_with(".tar.bz2") {
        return Some("tar.bz2".to_string());
    }
    if filename.ends_with(".tar.lz4") {
        return Some("tar.lz4".to_string());
    }
    if filename.ends_with(".7z") {
        return Some("7z".to_string());
    }
    path.extension()
        .map(|ext| ext.to_string_lossy().to_string())
}

pub fn decompress(input_path: &str, output_path: &str) -> Result<(), DecompressError> {
    let input_path_obj = Path::new(input_path);

    let file_type = get_file_type(input_path_obj)
        .ok_or_else(|| DecompressError::UnsupportedFormat("No file extension found".to_string()))?;

    match file_type.as_str() {
        "7z" => {
            sevenz_rust::decompress_file(input_path, output_path)?;
            Ok(())
        }
        "zip" => {
            let file = File::open(input_path)?;
            let reader = BufReader::new(file);
            let mut archive = zip::read::ZipArchive::new(reader)?;
            archive.extract(output_path)?;
            Ok(())
        }
        "tar" => {
            let file = File::open(input_path)?;
            let reader = BufReader::new(file);
            let mut archive = tar::Archive::new(reader);
            archive.unpack(output_path)?;
            Ok(())
        }
        "gz" | "tar.gz" => {
            let file = File::open(input_path)?;
            let reader = BufReader::new(file);
            let decoder = flate2::read::GzDecoder::new(reader);
            let mut archive = tar::Archive::new(decoder);
            archive.unpack(output_path)?;
            Ok(())
        }
        "xz" | "tar.xz" => {
            let file = File::open(input_path)?;
            let reader = BufReader::new(file);
            let decoder = xz2::read::XzDecoder::new(reader);
            let mut archive = tar::Archive::new(decoder);
            archive.unpack(output_path)?;
            Ok(())
        }
        "bz2" | "tar.bz2" => {
            let file = File::open(input_path)?;
            let reader = BufReader::new(file);
            let decoder = bzip2::read::BzDecoder::new(reader);
            let mut archive = tar::Archive::new(decoder);
            archive.unpack(output_path)?;
            Ok(())
        }
        "lz4" | "tar.lz4" => {
            let file = File::open(input_path)?;
            let reader = BufReader::new(file);
            let decoder = lz4::Decoder::new(reader)?;
            let mut archive = tar::Archive::new(decoder);
            archive.unpack(output_path)?;
            Ok(())
        }
        _ => Err(DecompressError::UnsupportedFormat(file_type)),
    }
}

fn add_directory_to_zip<W: Write + Seek>(
    zip_writer: &mut zip::write::ZipWriter<W>,
    dir: &Path,
    base: &Path,
    options: FileOptions,
) -> Result<(), std::io::Error> {
    let entries = std::fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .strip_prefix(base)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?
            .to_string_lossy()
            .replace("\\", "/");

        if path.is_file() {
            zip_writer.start_file(name, options)?;
            let mut f = File::open(&path)?;
            std::io::copy(&mut f, zip_writer)?;
        } else if path.is_dir() {
            let dir_name = if name.ends_with('/') {
                name
            } else {
                format!("{}/", name)
            };
            zip_writer.add_directory(dir_name, options)?;
            add_directory_to_zip(zip_writer, &path, base, options)?;
        }
    }
    Ok(())
}

pub fn compress(input_path: &str, output_path: &str, format: &str) -> Result<(), DecompressError> {
    match format {
        "7z" => {
            sevenz_rust::compress_to_path(input_path, output_path)?;
            Ok(())
        }
        "zip" => {
            let output_file = File::create(output_path)?;
            let input_path_obj = Path::new(input_path);
            let mut zip_writer = zip::write::ZipWriter::new(output_file);
            let options = FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .unix_permissions(0o755);

            if input_path_obj.is_dir() {
                add_directory_to_zip(&mut zip_writer, input_path_obj, input_path_obj, options)?;
            } else {
                let name = input_path_obj.file_name().unwrap().to_string_lossy();
                zip_writer.start_file(name, options)?;
                let mut f = File::open(input_path_obj)?;
                std::io::copy(&mut f, &mut zip_writer)?;
            }
            zip_writer.finish()?;
            Ok(())
        }
        // process tar and its compressed variants
        _ => {
            let output_file = File::create(output_path)?;

            fn create_tar_builder<W: Write>(
                writer: W,
                input_path: &str,
            ) -> Result<(), DecompressError> {
                let mut builder = tar::Builder::new(writer);
                builder.append_dir_all(".", input_path)?;
                builder.finish()?;
                Ok(())
            }

            match format {
                "tar" => create_tar_builder(output_file, input_path),
                "gz" | "tar.gz" => {
                    let encoder =
                        flate2::write::GzEncoder::new(output_file, flate2::Compression::default());
                    create_tar_builder(encoder, input_path)
                }
                "xz" | "tar.xz" => {
                    let encoder = xz2::write::XzEncoder::new(output_file, 6);
                    create_tar_builder(encoder, input_path)
                }
                "bz2" | "tar.bz2" => {
                    let encoder =
                        bzip2::write::BzEncoder::new(output_file, bzip2::Compression::default());
                    create_tar_builder(encoder, input_path)
                }
                "lz4" | "tar.lz4" => {
                    let encoder = lz4::EncoderBuilder::new().build(output_file)?;
                    let mut builder = tar::Builder::new(encoder);
                    builder.append_dir_all(".", input_path)?;
                    builder.finish()?;
                    let (_inner, result) = builder.into_inner()?.finish();
                    result.map_err(DecompressError::Io)?;
                    Ok(())
                }
                _ => Err(DecompressError::UnsupportedFormat(format.to_string())),
            }
        }
    }
}
