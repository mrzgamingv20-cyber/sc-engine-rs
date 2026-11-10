use std::path::{Path, PathBuf};
use std::fs;

/// Storage path prefixes
pub const APP_PREFIX: &str = "app:";
pub const DATA_PREFIX: &str = "data:";
pub const ANDROID_PREFIX: &str = "android:";
pub const CONFIG_PREFIX: &str = "config:";
pub const SYSTEM_PREFIX: &str = "system:";

pub struct Storage;

impl Storage {
    /// Resolve a path with prefix to an absolute filesystem path.
    pub fn normalize_path(path: &str) -> PathBuf {
        if let Some(rest) = path.strip_prefix(APP_PREFIX) {
            // App internal storage
            Self::app_dir().join(rest)
        } else if let Some(rest) = path.strip_prefix(DATA_PREFIX) {
            // App data directory
            Self::data_dir().join(rest)
        } else if let Some(rest) = path.strip_prefix(ANDROID_PREFIX) {
            // Android external storage
            Self::android_dir().join(rest)
        } else if let Some(rest) = path.strip_prefix(CONFIG_PREFIX) {
            Self::config_dir().join(rest)
        } else if let Some(rest) = path.strip_prefix(SYSTEM_PREFIX) {
            PathBuf::from(rest)
        } else {
            PathBuf::from(path)
        }
    }

    /// App internal files directory (filesDir)
    pub fn app_dir() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            // On Android, use the app's files directory
            // This is typically /data/data/<package>/files
            PathBuf::from("/data/data/com.scengine.core/files")
        }
        #[cfg(not(target_os = "android"))]
        {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        }
    }

    /// App data directory (dataDir)
    pub fn data_dir() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            PathBuf::from("/data/data/com.scengine.core/files")
        }
        #[cfg(not(target_os = "android"))]
        {
            Self::app_dir().join("data")
        }
    }

    /// Android external storage
    pub fn android_dir() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            PathBuf::from("/sdcard")
        }
        #[cfg(not(target_os = "android"))]
        {
            PathBuf::from("/sdcard")
        }
    }

    /// Config directory
    pub fn config_dir() -> PathBuf {
        Self::app_dir().join("config")
    }

    /// Ensure a directory exists
    pub fn ensure_directory(path: &Path) -> std::io::Result<()> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    /// Check if a file exists
    pub fn file_exists(path: &str) -> bool {
        let p = Self::normalize_path(path);
        p.exists() && p.is_file()
    }

    /// Check if a directory exists
    pub fn directory_exists(path: &str) -> bool {
        let p = Self::normalize_path(path);
        p.exists() && p.is_dir()
    }

    /// Delete a file
    pub fn delete_file(path: &str) -> std::io::Result<()> {
        let p = Self::normalize_path(path);
        fs::remove_file(p)
    }

    /// Delete a directory recursively
    pub fn delete_directory(path: &str) -> std::io::Result<()> {
        let p = Self::normalize_path(path);
        fs::remove_dir_all(p)
    }

    /// List files in a directory
    pub fn list_files(path: &str) -> std::io::Result<Vec<PathBuf>> {
        let p = Self::normalize_path(path);
        let mut files = Vec::new();
        for entry in fs::read_dir(p)? {
            let entry = entry?;
            files.push(entry.path());
        }
        Ok(files)
    }

    /// Get file size in bytes
    pub fn file_size(path: &str) -> std::io::Result<u64> {
        let p = Self::normalize_path(path);
        Ok(fs::metadata(p)?.len())
    }

    /// Copy a file
    pub fn copy_file(from: &str, to: &str) -> std::io::Result<()> {
        let src = Self::normalize_path(from);
        let dst = Self::normalize_path(to);
        if let Some(parent) = dst.parent() {
            Self::ensure_directory(parent)?;
        }
        fs::copy(src, dst)?;
        Ok(())
    }

    /// Sanitize a filename (remove invalid characters)
    pub fn sanitize_filename(name: &str) -> String {
        name.chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                c => c,
            })
            .collect()
    }

    /// Get the base name of a path (without extension)
    pub fn get_base_name(path: &str) -> &str {
        Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(path)
    }

    /// Get the extension of a path
    pub fn get_extension(path: &str) -> Option<&str> {
        Path::new(path).extension().and_then(|s| s.to_str())
    }

    /// Get the directory part of a path
    pub fn get_directory(path: &str) -> &str {
        Path::new(path)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
    }

    /// Join path segments
    pub fn join_path(base: &str, relative: &str) -> PathBuf {
        Path::new(base).join(relative)
    }
}
