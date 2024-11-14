use std::path::PathBuf;

use crate::prelude::*;
use crate::context::*;
use gtk::gio::File;

use super::WindowComponent;

#[derive(Clone)]
pub struct FileDialog {
    dialog: gtk::FileDialog,
    window: gtk::Window,
    cancellable: gtk::gio::Cancellable,
}

impl FileDialog {
    pub fn new(window: impl IsA<gtk::Window>) -> Self {
        let this = Self {
            dialog: gtk::FileDialog::new(),
            window: window.upcast(),
            cancellable: gtk::gio::Cancellable::new(),
        };
        this.configure_filters();
        this
    }

    fn configure_filters(&self) {
        let filter = gtk::FileFilter::new();
        filter.set_name(Some("Documents"));
        filter.add_mime_type("application/json");

        let filter_list = gtk::gio::ListStore::new::<gtk::FileFilter>();
        filter_list.append(&filter);

        self.dialog.set_default_filter(Some(&filter));
        self.dialog.set_filters(Some(&filter_list));
    }

    fn configure_dialog(&self, context: &Context) {
        self.set_initial_path(context);
        self.set_dialog_properties();
    }

    fn set_initial_path(&self, context: &Context) {
        let initial_file_path = context.directory().generate_unique_file_path_or_default();
        
        if let Some(file_name) = initial_file_path
            .file_name()
            .and_then(|f| f.to_str()) {
                self.dialog.set_initial_name(Some(file_name));
            }

        let initial_folder = File::for_path(&context.directory().path);
        self.dialog.set_initial_folder(Some(&initial_folder));
    }

    fn set_dialog_properties(&self) {
        self.dialog.set_accept_label(Some("Select"));
        self.dialog.set_title("Select document file");
        self.dialog.set_modal(false);
    }

    fn choose_file(&self, context: &Context, callback: Box<dyn FnOnce(Option<PathBuf>, &Context) + 'static>) {
        let context = context.clone();
        self.dialog.open(
            Some(&self.window),
            Some(&self.cancellable),
            move |result| {
                let result = result.ok().and_then(|file| file.path());
                callback(result, &context);
            },
        );
    }

}

impl LifeCycle<UiAction> for FileDialog {

    fn activate(&self, action: UiAction, context: &Context) {
        let UiAction::OpenFileChooserDialog {
            callback
        } = action else { return };
        self.configure_dialog(context);
        self.choose_file(context, callback);
    }

    fn deactivate(&self) {
        if !self.cancellable.is_cancelled() {
            self.cancellable.cancel();
        }
    }

}

impl WindowComponent for FileDialog {
    fn is_active(&self) -> bool {
        !self.cancellable.is_cancelled()
    }
}