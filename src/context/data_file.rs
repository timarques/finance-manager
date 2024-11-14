use crate::data::Data;
use crate::metadata;

use std::path::PathBuf;
use std::fs;
use std::io;
use std::time::SystemTime;

#[derive(Debug, Clone, Default)]
pub struct DataFile {
    pub path: PathBuf,
}

impl DataFile {

    #[inline]
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load(&self) -> io::Result<Data> {
        let content = fs::read(&self.path)?;
        serde_json::from_slice(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn save(&self, data: &Data) -> io::Result<()> {
        let content = serde_json::to_string(data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&self.path, content)
    }

    pub fn remove(&self) -> io::Result<()> {
        fs::remove_file(&self.path)
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn is_valid(&self) -> bool {
        self.has_valid_extension() &&
        self.has_valid_filename() &&
        self.has_valid_content()
    }

    fn has_valid_content(&self) -> bool {
        fs::read_to_string(&self.path)
            .ok()
            .and_then(|content| serde_json::from_str::<Data>(&content).ok())
            .is_some()
    }

    fn has_valid_extension(&self) -> bool {
        self.path.extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| ext == "json")
    }

    fn has_valid_filename(&self) -> bool {
        self.path.file_name()
            .and_then(|name| name.to_str())
            .map_or(false, |name| name.contains(metadata::APP_NAME))
    }

    pub fn modified_time(&self) -> io::Result<SystemTime> {
        fs::metadata(&self.path)?.modified()
    }
}