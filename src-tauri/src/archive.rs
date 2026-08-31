use std::fs::{self, File};
use std::io::{Cursor, Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use zip::{ZipWriter, ZipArchive};
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;

use crate::errors::CryptVaultError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultFileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
}

/// Packs multiple files/directories into an uncompressed ZIP archive in memory.
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
                        .replace('\\', "/");

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

/// Creates a minimal default empty ZIP archive with a Welcome note.
pub fn create_initial_zip(vault_name: &str) -> Result<Vec<u8>, CryptVaultError> {
    let mut buf = Vec::new();
    {
        let mut zip = ZipWriter::new(Cursor::new(&mut buf));
        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Stored);

        let welcome_filename = "Readme.txt";
        zip.start_file(welcome_filename, options)
            .map_err(|e| CryptVaultError::Serialization(e.to_string()))?;

        let note = format!("Welcome to {}\nYour files are safely stored inside this encrypted volume.", vault_name);
        zip.write_all(note.as_bytes())?;

        zip.finish().map_err(|e| CryptVaultError::Serialization(e.to_string()))?;
    }
    Ok(buf)
}

/// Lists all files and folders in an in-memory ZIP archive.
pub fn list_zip_entries(zip_bytes: &[u8]) -> Result<Vec<VaultFileEntry>, CryptVaultError> {
    if zip_bytes.is_empty() {
        return Ok(Vec::new());
    }
    let mut archive = ZipArchive::new(Cursor::new(zip_bytes))
        .map_err(|_e| CryptVaultError::InvalidContainer)?;

    let mut entries = Vec::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i)
            .map_err(|_e| CryptVaultError::InvalidContainer)?;
        
        let path_str = file.name().to_string();
        let is_dir = file.is_dir() || path_str.ends_with('/');
        let trimmed_path = path_str.trim_end_matches('/').to_string();
        let name = Path::new(&trimmed_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&trimmed_path)
            .to_string();

        entries.push(VaultFileEntry {
            name,
            path: trimmed_path,
            is_dir,
            size: file.size(),
        });
    }

    Ok(entries)
}

/// Appends new files/folders to an existing in-memory ZIP archive.
pub fn append_to_zip(existing_zip: &[u8], new_paths: &[String]) -> Result<Vec<u8>, CryptVaultError> {
    let mut entries_map = std::collections::HashMap::new();

    if !existing_zip.is_empty() {
        if let Ok(mut archive) = ZipArchive::new(Cursor::new(existing_zip)) {
            for i in 0..archive.len() {
                if let Ok(mut file) = archive.by_index(i) {
                    let mut data = Vec::new();
                    let _ = file.read_to_end(&mut data);
                    entries_map.insert(file.name().to_string(), (file.is_dir(), data));
                }
            }
        }
    }

    // Pack new files
    let packed_new = pack_paths(new_paths)?;
    if let Ok(mut new_archive) = ZipArchive::new(Cursor::new(packed_new)) {
        for i in 0..new_archive.len() {
            if let Ok(mut file) = new_archive.by_index(i) {
                let mut data = Vec::new();
                let _ = file.read_to_end(&mut data);
                entries_map.insert(file.name().to_string(), (file.is_dir(), data));
            }
        }
    }

    // Re-build consolidated ZIP
    let mut out = Vec::new();
    {
        let mut zip = ZipWriter::new(Cursor::new(&mut out));
        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Stored);

        for (name, (is_dir, data)) in entries_map {
            if is_dir {
                let _ = zip.add_directory(&name, options);
            } else {
                if zip.start_file(&name, options).is_ok() {
                    let _ = zip.write_all(&data);
                }
            }
        }
        zip.finish().map_err(|e| CryptVaultError::Serialization(e.to_string()))?;
    }

    Ok(out)
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
