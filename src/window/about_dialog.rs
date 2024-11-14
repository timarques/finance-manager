use crate::prelude::*;
use crate::metadata;
use crate::context::*;

use super::WindowComponent;
use super::UiAction;

pub struct AboutDialog {
    window: gtk::Window,
    about_dialog: adw::AboutDialog
}

impl AboutDialog {

    pub fn new(window: impl IsA<gtk::Window>) -> Self {
        let about_dialog = Self::build_about_dialog();
        Self {
            window: window.upcast(),
            about_dialog,
        }
    }

    fn build_about_dialog() -> adw::AboutDialog {
        let dialog = adw::AboutDialog::new();
            dialog.set_can_close(true);
            dialog.set_presentation_mode(adw::DialogPresentationMode::Floating);
            dialog.set_application_name(&metadata::APP_TITLE);
            dialog.set_title(&metadata::APP_DESCRIPTION);
            dialog.set_application_icon(&metadata::APP_ICON_NAME);
            dialog.set_version(metadata::APP_VERSION);
            dialog.set_developers(&[metadata::APP_DEVELOPERS]);
            dialog.set_website(metadata::APP_HOMEPAGE);
            dialog.set_license_type(match metadata::APP_LICENSE {
                "GPL-3.0" => gtk::License::Gpl30,
                "MIT" => gtk::License::MitX11,
                _ => gtk::License::Custom
            });

        dialog
    }

}

impl LifeCycle<UiAction> for AboutDialog {
    fn activate(&self, action: UiAction, _: &Context) {
        if !matches!(action, UiAction::OpenAboutDialog) { unreachable!() };
        self.about_dialog.present(Some(&self.window));
    }

    fn deactivate(&self) {
        self.about_dialog.close();
    }
}

impl WindowComponent for AboutDialog {
    fn is_active(&self) -> bool {
        self.about_dialog.can_close()
    }
}