use crate::prelude::*;
use crate::context::*;
use crate::utils::AlertButton;
use super::WindowComponent;

pub struct AlertDialog {
    window: adw::ApplicationWindow,
    alert_dialog: adw::AlertDialog,
}

impl AlertDialog {
    pub fn new(window: adw::ApplicationWindow) -> Self {
        Self {
            window,
            alert_dialog: adw::AlertDialog::new(None, None),
        }
    }

    fn add_buttons(&self, buttons: &[AlertButton]) {
        for button in buttons {
            self.alert_dialog.add_response(button.tag(), &button.label);

            if button.is_suggested() {
                self.alert_dialog.set_response_appearance(button.tag(), adw::ResponseAppearance::Suggested);
            } else if button.is_destructive() {
                self.alert_dialog.set_response_appearance(button.tag(), adw::ResponseAppearance::Destructive);
            }

            if button.is_default() {
                self.alert_dialog.set_default_response(Some(button.tag()));
            }
        }
    }

    fn show_dialog(&self, buttons: Vec<AlertButton>, callback: Box<dyn FnOnce(Option<&AlertButton>)>) {
        let cloned_dialog = self.alert_dialog.clone();
        self.alert_dialog.clone().choose(&self.window, gtk::gio::Cancellable::NONE, move |id| {

            let mut result = None;
            for button in &buttons {
                let tag = button.tag();
                cloned_dialog.remove_response(tag);
                if tag == id {
                    result = Some(button);
                }
            }

            callback(result);
        });
    }

}

impl LifeCycle<UiAction> for AlertDialog {
    fn activate(&self, action: UiAction, _: &Context) {
        let UiAction::OpenAlertDialog {
            title,
            message,
            buttons,
            callback
        } = action else { unreachable!() };

        self.alert_dialog.set_heading(Some(&title));
        self.alert_dialog.set_body(&message);
        self.add_buttons(&buttons);
        self.show_dialog(buttons, callback);
    }

    fn deactivate(&self) {
        if self.alert_dialog.can_close() {
            self.alert_dialog.close();
        }
    }
}

impl WindowComponent for AlertDialog {
    fn is_active(&self) -> bool {
        self.alert_dialog.can_close()
    }
}