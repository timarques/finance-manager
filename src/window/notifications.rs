use crate::prelude::*;
use crate::context::*;
use std::cell::RefCell;
use super::WindowComponent;

#[derive(Debug, Clone)]
pub struct Notifications {
    toast_overlay: adw::ToastOverlay,
    last_toast: RefCell<Option<adw::Toast>>
}

impl Notifications {

    pub fn new(child: &impl HasWidget<gtk::Widget>) -> Self {
        let toast_overlay = Self::build_toast_overlay(child.widget());
        Self {
            toast_overlay,
            last_toast: RefCell::new(None)
        }
    }

    fn build_toast_overlay(child: &impl IsA<gtk::Widget>) -> adw::ToastOverlay {
        let toast_overlay = adw::ToastOverlay::new();
        toast_overlay.set_child(Some(child));
        toast_overlay
    }

}

impl HasWidget<gtk::Widget> for Notifications {
    fn widget(&self) -> &gtk::Widget {
        self.toast_overlay.upcast_ref()
    }
}

impl LifeCycle<UiAction> for Notifications {
    fn activate(&self, action: UiAction, _: &Context) {
        let UiAction::PushNotification { message } = action else { unreachable!() };
        let toast = adw::Toast::new(&message);
        toast.set_timeout(2);
        self.toast_overlay.add_toast(toast.clone());
        self.last_toast.borrow_mut().replace(toast);
    }

    fn deactivate(&self) {
        if let Some(toast) = self.last_toast.borrow_mut().take() {
            toast.dismiss();
        }
    }
}

impl WindowComponent for Notifications {
    fn is_active(&self) -> bool {
        self.last_toast.borrow().is_some()
    }
}