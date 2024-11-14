use crate::prelude::*;
use super::ButtonList;

pub trait PopoverExtension: IsA<gtk::Popover> {
    fn set_button_list<K: Clone + Eq + std::hash::Hash + 'static>(&self, button_list: &ButtonList<K>) {
        self.set_child(Some(button_list.widget()));
        let this_weak = self.downgrade();
        button_list.connect_activated(move |_, _, _| {
            let this_weak = this_weak.clone();
            gtk::glib::timeout_add_local_once(
                std::time::Duration::from_millis(100), 
                move || {
                    let Some(this) = this_weak.upgrade() else { return };
                    gtk::prelude::PopoverExt::popdown(&this)
                }
            );
        });
    }
}

impl PopoverExtension for gtk::Popover {}