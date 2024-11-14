use crate::metadata;

use super::data_file::DataFile;
use std::path::PathBuf;
use std::io;
use std::fs;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct DataDirectory {
    pub path: PathBuf,
}

impl Default for DataDirectory {
    fn default() -> Self {
        Self { path: PathBuf::new() }
    }
}

impl DataDirectory {

    pub fn from_user_data_dir() -> Self {
        let path = gtk::glib::user_data_dir().join(metadata::APP_ID);
        Self { path }
    }

    pub fn ensure_exists(&self) -> io::Result<()> {
        if !self.path.exists() {
            fs::create_dir_all(&self.path)?;
        }
        Ok(())
    }

    pub fn list_valid(&self) -> io::Result<Vec<DataFile>> {
        let mut valid_files = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let data_file = DataFile::new(entry.path());
            if data_file.is_valid() {
                valid_files.push(data_file);
            }
        }
        Ok(valid_files)
    }

    pub fn find_most_recent_data_file(&self) -> io::Result<Option<DataFile>> {
        let valid_files = self.list_valid()?;
        let mut latest: Option<(DataFile, SystemTime)> = None;

        for file in valid_files {
            let modified = file.modified_time()?;
            match &latest {
                None => latest = Some((file, modified)),
                Some((_, last_mod)) if modified > *last_mod => {
                    latest = Some((file, modified));
                }
                _ => {}
            }
        }

        Ok(latest.map(|(file, _)| file))
    }

    pub fn generate_unique_file_path(&self) -> io::Result<PathBuf> {
        const MAX_ATTEMPTS: usize = 1000;
        let mut count = 0;
        
        while count < MAX_ATTEMPTS {
            let file_path = self.build_data_path(count);
            if !file_path.exists() {
                return Ok(file_path);
            }
            count += 1;
        }
        
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not find unique filename after maximum attempts"
        ))
    }

    pub fn generate_unique_file_path_or_default(&self) -> PathBuf {
        self.generate_unique_file_path()
            .unwrap_or(self.path.join(format!("data.{}.json", metadata::APP_NAME)))
    }

    pub fn create_new_data_file(&self) -> io::Result<DataFile> {
        let file_path = self.generate_unique_file_path()?;
        let data_file = DataFile::new(file_path);
        return Ok(data_file);
    }

    fn build_data_path(&self, count: usize) -> PathBuf {
        let filename = if count == 0 {
            format!("data.{}.json", metadata::APP_NAME)
        } else {
            format!("data.{}.{}.json", count, metadata::APP_NAME)
        };
        self.path.join(filename)
    }
}