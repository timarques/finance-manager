use std::{env, fs};
use std::env::VarError;
use std::error::Error;
use std::path::{Path, PathBuf};
use super::metadata::Metadata;

pub struct Linux {
    metadata: Metadata,
    data_dir_path: PathBuf
}

impl Linux {
    pub fn new(metadata: Metadata) -> Result<Self, Box<dyn Error>> {
        let data_dir_path = Self::get_data_dir_path()?;
        Ok(Self {
            metadata,
            data_dir_path
        })
    }

    fn get_data_dir_path() -> Result<PathBuf, VarError> {
        let is_root = unsafe { libc::geteuid() == 0 };

        let data_dir = if let Ok(dir) = env::var("XDG_DATA_HOME") {
            PathBuf::from(dir)
        } else if is_root {
            PathBuf::from("/usr/share")
        } else if let Ok(dir) = env::var("HOME") {
            PathBuf::from(&dir)
                .join(".local")
                .join("share")
        } else {
            return Err(VarError::NotPresent)
        };

        Ok(data_dir)
    }

    fn get_executable_path(&self) -> Result<PathBuf, VarError> {
        let bin_file_name = env::var("CARGO_BIN_FILE")
            .unwrap_or_else(|_| self.metadata.name.to_string());

        let install_dir = env::var("CARGO_INSTALL_ROOT")?;
        let path = PathBuf::from(install_dir)
            .join("bin")
            .join(&bin_file_name);

        Ok(path)
    }

    fn get_icons_dir_path(&self) -> PathBuf {
        self.data_dir_path.join("icons/hicolor/scalable/apps")
    }

    fn get_applications_dir_path(&self) -> PathBuf {
        self.data_dir_path.join("applications")
    }

    fn install_icon(&self) -> Result<PathBuf, std::io::Error> {
        let icons_dir = self.get_icons_dir_path();
        fs::create_dir_all(&icons_dir)?;

        let source_icon_string = format!("resources/{}.svg", self.metadata.icon_name);
        let source_icon = Path::new(&source_icon_string);
        if !source_icon.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Icon not found"
            ))
        }

        let target_icon = icons_dir.join(format!("{}.svg", self.metadata.id));
        fs::copy(source_icon, &target_icon)?;
        Ok(target_icon)
    }

    fn install_desktop_file(
        &self,
        icon_path: &Path,
    ) -> Result<(), std::io::Error> {
        let template_path = Path::new("resources/app.desktop.in");
        if !template_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Desktop file not found"
            ));
        }

        let executable = self.get_executable_path()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or(self.metadata.name.to_string());
        
        let content = fs::read_to_string(&template_path)?
            .replace("@APP_ICON@", icon_path.to_str().unwrap())
            .replace("@APP_TITLE@", &self.metadata.title)
            .replace("@APP_EXEC@", &executable)
            .replace("@APP_VERSION@", env!("CARGO_PKG_VERSION"))
            .replace("@APP_NAME@", env!("CARGO_PKG_NAME"))
            .replace("@APP_DESCRIPTION@", env!("CARGO_PKG_DESCRIPTION"));

        let apps_dir = self.get_applications_dir_path();
        fs::create_dir_all(&apps_dir)?;

        let desktop_path = apps_dir.join(format!("{}.desktop", self.metadata.id));
        fs::write(&desktop_path, content)?;
        Ok(())
    }

    pub fn install(&self) -> Result<(), std::io::Error> {
        let icon_path = self.install_icon()?;
        self.install_desktop_file(&icon_path)?;
        Ok(())
    }

}