mod action;
mod data_file;
mod data_directory;

pub use action::{NavigationAction, UiAction, Action};
pub use data_file::DataFile;
pub use data_directory::DataDirectory;

use crate::prelude::*;
use crate::data::*;

use std::rc::Rc;

pub struct Context {
    directory: DataDirectory,
    file: DataFile,
    data: Rc<Data>,
    changed_data: bool,

    ui_propagator: Option<Rc<dyn Propagator<UiAction>>>,
    navigation_propagator: Option<Rc<dyn Propagator<NavigationAction>>>,
    navigation_action: Option<NavigationAction>,
    ui_action: Option<UiAction>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            directory: DataDirectory::default(),
            file: DataFile::default(),
            data: Rc::new(Data::default()),
            changed_data: false,
            ui_propagator: None,
            navigation_propagator: None,
            navigation_action: None,
            ui_action: None
        }
    }
}

impl Context {
    pub fn new(
        directory: DataDirectory,
        ui_propagator: impl Propagator<UiAction> + 'static,
        navigation_propagator: impl Propagator<NavigationAction> + 'static
    ) -> Self {
        Self {
            directory,
            file: DataFile::default(),
            data: Rc::new(Data::default()),
            changed_data: false,
            ui_propagator: Some(Rc::new(ui_propagator)),
            navigation_propagator: Some(Rc::new(navigation_propagator)),
            navigation_action: None,
            ui_action: None
        }
    }

    pub fn data(&self) -> &Data {
        &self.data
    }

    pub const fn directory(&self) -> &DataDirectory {
        &self.directory
    }

    pub const fn file(&self) -> &DataFile {
        &self.file
    }

    pub fn with_file(mut self, file: DataFile) -> Self {
        self.file = file;
        self
    }

    pub fn with_data(mut self, data: Data) -> Self {
        self.data = Rc::new(data);
        self.changed_data = true;
        self
    }

    pub fn with_navigation_action(mut self, action: NavigationAction) -> Self {
        self.navigation_action = Some(action);
        self
    }

    pub fn with_ui_action(mut self, action: UiAction) -> Self {
        self.ui_action = Some(action);
        self
    }

    pub fn propagate(mut self) {
        if let Some(navigation_action) = self.navigation_action.take() {
            if let Some(navigation_propagator) = &self.navigation_propagator {
                navigation_propagator.propagate(navigation_action, &self);
            }
        }
    
        if let Some(ui_action) = self.ui_action.take() {
            if let Some(ui_propagator) = &self.ui_propagator {
                ui_propagator.propagate(ui_action, &self);
            }
        }
    }

    fn handle_save_error(
        &self,
        err: std::io::Error,
        ui_propagator: &Option<Rc<dyn Propagator<UiAction>>>,
        default_context: &Context,
    ) {
        if let Some(propagator) = ui_propagator {
            propagator.propagate(
                UiAction::push_notification("Failed to save data"),
                default_context,
            );
        } else {
            eprintln!("Failed to save data: {}", err);
        }
    }

    fn handle_load_error(
        &self,
        _: std::io::Error,
        ui_propagator: &Option<Rc<dyn Propagator<UiAction>>>,
        default_context: &Context,
    ) {
        let remove_result = self.file.remove();
        if let Some(propagator) = ui_propagator {
            if remove_result.is_err() {
                propagator.propagate(
                    UiAction::push_notification("Failed to remove invalid file"),
                    default_context,
                );
            } else {
                propagator.propagate(
                    UiAction::push_notification("Removed invalid file"),
                    default_context,
                );
            }
        } else {
            if let Err(e) = remove_result {
                eprintln!("Failed to remove invalid file: {}", e);
            } else {
                eprintln!("Removed invalid file");
            }
        }
    }

}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            directory: self.directory.clone(),
            file: self.file.clone(),
            data: self.data.clone(),
            ui_propagator: self.ui_propagator.clone(),
            navigation_propagator: self.navigation_propagator.clone(),

            changed_data: false,
            ui_action: None,
            navigation_action: None
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if !self.changed_data || self.data.is_empty() {
            return
        }

        let ui_propagator = self.ui_propagator.take();
        if self.data.is_valid() {
            if let Err(e) = self.file.save(&self.data) {
                self.handle_save_error(e, &ui_propagator, &self);
            }
        } else {
            if let Err(e) = self.file.load() {
                self.handle_load_error(e, &ui_propagator, &self);
            }
        }
    }
}