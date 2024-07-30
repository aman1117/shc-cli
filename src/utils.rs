use ignore::WalkBuilder;
use std::fs::{self, File};
use std::io::{self};
use std::path::{Path, PathBuf};
use zip::write::FileOptions;
use zip::{CompressionMethod::Bzip2, ZipWriter};

use crate::consts::SHC_IGNORE_FILE_NAME;
use tempfile::NamedTempFile;

pub fn format_bytes(bytes: u64) -> String {
    let mut bytes = bytes as f64;
    let mut unit = 0;

    while bytes >= 1000.0 {
        bytes /= 1000.0;
        unit += 1;
    }

    let unit = match unit {
        0 => "B",
        1 => "KB",
        2 => "MB",
        3 => "GB",
        4 => "TB",
        5 => "PB",
        6 => "EB",
        7 => "ZB",
        _ => "YB",
    };

    format!("{:.2} {}", bytes, unit)
}

pub fn zip_directory_recursive(src_dir: &Path, size_limit: u64) -> io::Result<PathBuf> {
    let src_dir = fs::canonicalize(src_dir)?;
    let folder_name = src_dir.file_name().unwrap().to_string_lossy();
    let dest_file_path = NamedTempFile::new_in("/tmp")?
        .into_temp_path()
        .with_extension("zip")
        .with_file_name(format!("{}.zip", folder_name));

    let dest_file = File::create(&dest_file_path)?;

    let mut zip = ZipWriter::new(dest_file);

    fn zip_inner(
        path: &Path,
        zip: &mut ZipWriter<File>,
        base_path: &Path,
        size_limit: u64,
        current_size: &mut u64,
    ) -> io::Result<u64> {
        let mut total_size = 0;

        if path.is_file() {
            let relative_path = path.strip_prefix(base_path).unwrap();
            let zip_path = relative_path.to_string_lossy();
            let options = FileOptions::default()
                .compression_method(Bzip2)
                .unix_permissions(0o755);

            zip.start_file(zip_path, options)?;
            let mut file = File::open(path)?;

            let file_size = io::copy(&mut file, zip)?;
            total_size += file_size;
            *current_size += file_size;

            if *current_size > size_limit {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Exceeded size limit for zip file",
                ));
            }
        } else if path.is_dir() {
            let walker = WalkBuilder::new(path)
                .git_ignore(false)
                .add_custom_ignore_filename(SHC_IGNORE_FILE_NAME)
                .build();

            for result in walker {
                // TODO: Handle errors
                let entry_path = result.unwrap().into_path();

                // Skip the base path
                if path == entry_path {
                    continue;
                }

                total_size += zip_inner(&entry_path, zip, base_path, size_limit, current_size)?;
            }
        }

        Ok(total_size)
    }

    let mut current_size = 0;

    let _total_size = zip_inner(&src_dir, &mut zip, &src_dir, size_limit, &mut current_size)?;
    Ok(dest_file_path)
}
