mod notifications;
mod file_dialog;
mod about_dialog;
mod alert_dialog;

use crate::metadata;
use crate::prelude::*;
use crate::context::*;

use std::rc::Rc;
use std::cell::Cell;
use std::collections::HashMap;

trait WindowComponent: LifeCycle<UiAction> {
    fn is_active(&self) -> bool;
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ComponentType {
    #[default]
    None,
    Notifications,
    AlertDialog,
    FileDialog,
    About,
}

pub struct Window {
    components: HashMap<ComponentType, Box<dyn WindowComponent>>,
    previous_component: Cell<ComponentType>,
}

impl Window {
    pub fn new(application: &adw::Application, child: &impl HasWidget<gtk::Widget>) -> Rc<Self> {
        let notifications = notifications::Notifications::new(child);

        let window = Self::build_window(application, &notifications);
        let file_dialog = file_dialog::FileDialog::new(window.clone());
        let about_dialog = about_dialog::AboutDialog::new(window.clone());
        let alert_dialog = alert_dialog::AlertDialog::new(window.clone());

        Rc::new(Self {
            components: HashMap::from([
                (ComponentType::Notifications, Box::new(notifications) as Box<dyn WindowComponent>),
                (ComponentType::FileDialog, Box::new(file_dialog) as Box<dyn WindowComponent>),
                (ComponentType::About, Box::new(about_dialog) as Box<dyn WindowComponent>),
                (ComponentType::AlertDialog, Box::new(alert_dialog) as Box<dyn WindowComponent>),
            ]),
            previous_component: Cell::new(ComponentType::default()),
        })
    }

    fn build_window(application: &adw::Application, child: &impl HasWidget<gtk::Widget>) -> adw::ApplicationWindow {
        let window = adw::ApplicationWindow::new(application);
        window.set_content(Some(child.widget()));
        window.set_default_size(800, 600);
        window.set_icon_name(Some(metadata::APP_ICON_NAME));
        window.present();
        window
    }

    const fn get_component_by_ui_action(&self, action: &UiAction) -> ComponentType {
        match action {
            UiAction::PushNotification { .. } => ComponentType::Notifications,
            UiAction::OpenFileChooserDialog { .. } => ComponentType::FileDialog,
            UiAction::OpenAboutDialog { .. } => ComponentType::About,
            UiAction::OpenAlertDialog { .. } => ComponentType::AlertDialog,
        }
    }

}

impl Propagator<UiAction> for Rc<Window> {
    fn propagate(&self, action: UiAction, context: &Context) {
        let this = self.clone();
        let context = context.clone();

        gtk::glib::idle_add_local_once(move || {
            let component_type = this.get_component_by_ui_action(&action);
            let previous_component_type = this.previous_component.get();

            if let Some(previous) = this.components.get(&previous_component_type) {
                if previous.is_active() {
                    previous.deactivate();
                }
            }

            if let Some(component) = this.components.get(&component_type) {
                component.activate(action, &context);
                this.previous_component.set(component_type);
            }
        });
    }
}