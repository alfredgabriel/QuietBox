use std::fs::{self, File};
use std::io::{Cursor, Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::{ZipWriter, ZipArchive};
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;

use crate::errors::CryptVaultError;

/// Packs multiple files/directories into a single, uncompressed ZIP archive in memory.
/// Using Stored (no compression) is critical because compression reduces entropy,
/// which can make the file distinguishable from random noise or leak structural information.
pub fn pack_paths(paths: &[String]) -> Result<Vec<u8>, CryptVaultError> {
    let mut buf = Vec::new();
    {
        let mut zip = ZipWriter::new(Cursor::new(&mut buf));
        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Stored);

        for p_str in paths {
            let path = Path::new(p_str);
            if !path.exists() {
                continue;
            }

            if path.is_file() {
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| CryptVaultError::Serialization("Invalid filename".into()))?;
                
                zip.start_file(name, options)
                    .map_err(|e| CryptVaultError::Serialization(e.to_string()))?;
                
                let mut f = File::open(path)?;
                let mut data = Vec::new();
                f.read_to_end(&mut data)?;
                zip.write_all(&data)?;
            } else if path.is_dir() {
                let base_dir = path.parent().unwrap_or(path);
                for entry in WalkDir::new(path) {
                    let entry = entry.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                    let entry_path = entry.path();
                    
                    let relative_path = entry_path.strip_prefix(base_dir)
                        .map_err(|_| CryptVaultError::Serialization("Strip prefix failed".into()))?;
                    
                    let path_str = relative_path.to_str()
                        .ok_or_else(|| CryptVaultError::Serialization("Non-UTF8 path".into()))?
                        .replace('\\', "/"); // standard zip forward slashes

                    if entry_path.is_dir() {
                        zip.add_directory(&path_str, options)
                            .map_err(|e| CryptVaultError::Serialization(e.to_string()))?;
                    } else if entry_path.is_file() {
                        zip.start_file(&path_str, options)
                            .map_err(|e| CryptVaultError::Serialization(e.to_string()))?;
                        
                        let mut f = File::open(entry_path)?;
                        let mut data = Vec::new();
                        f.read_to_end(&mut data)?;
                        zip.write_all(&data)?;
                    }
                }
            }
        }
        zip.finish().map_err(|e| CryptVaultError::Serialization(e.to_string()))?;
    }
    Ok(buf)
}

/// Unpacks an in-memory ZIP archive to a destination directory.
pub fn unpack_to(zip_bytes: &[u8], output_dir: &str) -> Result<(), CryptVaultError> {
    let mut archive = ZipArchive::new(Cursor::new(zip_bytes))
        .map_err(|_e| CryptVaultError::InvalidContainer)?;

    let out_path = Path::new(output_dir);
    fs::create_dir_all(out_path)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|_e| CryptVaultError::InvalidContainer)?;
        
        // Sanitize path to prevent Zip Slip vulnerability
        let file_path = match file.enclosed_name() {
            Some(p) => p.to_owned(),
            None => continue,
        };

        let dest_path = out_path.join(file_path);

        if file.name().ends_with('/') {
            fs::create_dir_all(&dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&dest_path)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
