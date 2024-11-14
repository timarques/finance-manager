use crate::prelude::*;
use crate::utils::{ButtonList, PopoverExtension};
use crate::data::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct HeaderRow {
    horizontal_box: gtk::Box,
    main_button: gtk::Button,
    main_button_label: gtk::Label,
    button_list: ButtonList<Period>,
    popover: gtk::Popover,
    callback: Rc<RefCell<Option<Rc<dyn Fn(Period)>>>>
}

impl HeaderRow {

    pub fn new() -> Self {
        let main_button_label = Self::build_period_label();
        let main_button = Self::build_main_button(&main_button_label);
        let horizontal_box = Self::build_horizontal_box(&main_button);
        let button_list = Self::build_button_list();
        let popover = gtk::Popover::new();
            popover.set_button_list(&button_list);
            popover.set_parent(&main_button);

        let this = Self {
            horizontal_box,
            main_button,
            main_button_label,
            button_list,
            popover,
            callback: Rc::new(RefCell::new(None))
        };

        this.connect_button_list_activated();
        this.connect_main_button_activated();
        this
    }

    fn build_period_label() -> gtk::Label {
        let label = gtk::Label::new(Some(Period::default().as_str()));
            label.set_hexpand(true);
            label.set_halign(gtk::Align::Center);
            label
    }

    fn build_main_button(label: &gtk::Label) -> gtk::Button {
        let icon = gtk::Image::from_icon_name("graph-symbolic");
            icon.set_halign(gtk::Align::Start);

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
            h_box.append(&icon);
            h_box.append(label);
            h_box.set_hexpand(false);
        
        let button = gtk::Button::new();
            button.set_valign(gtk::Align::Center);
            button.set_focusable(false);
            button.set_child(Some(&h_box));
            button.set_tooltip_text(Some("Period"));
            button
    }

    fn build_button_list() -> ButtonList<Period> {
        let button_list = ButtonList::new(false);
        for period in Period::as_slice() {
            button_list.add_with_text(period, period.as_str());
        }
        button_list
    }

    fn build_horizontal_box(child: &impl IsA<gtk::Widget>) -> gtk::Box {
        let title_label = gtk::Label::new(Some("Wallets"));
        title_label.add_css_class("title-3");
        title_label.set_halign(gtk::Align::Start);
        title_label.set_hexpand(true);

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        h_box.append(&title_label);
        h_box.append(child);
        h_box.set_valign(gtk::Align::Center);
        h_box.set_halign(gtk::Align::Fill);
        h_box.set_hexpand(true);
        h_box
    }

    fn connect_button_list_activated(&self) {
        let callback = self.callback.clone();
        let main_button_label_weak = self.main_button_label.downgrade();
        self.button_list.connect_activated(move |period, _, _| {
            let Some(main_button_label) = main_button_label_weak.upgrade() else { return; };
            main_button_label.set_label(period.as_str());
            let callback = callback.borrow().clone();
            if let Some(callback) = callback  {
                callback(period);
            }
        });
    }

    fn connect_main_button_activated(&self) {
        let popover_weak = self.popover.downgrade();
        self.main_button.connect_clicked(move |_| {
            if let Some(popover) = popover_weak.upgrade() {
                if !popover.is_visible() {
                    popover.popup();
                }
            }
        });
    }

    pub fn set_period(&self, period: Period) {
        self.main_button_label.set_label(period.as_str());
        self.button_list.activate_button(&period);
    }

    pub fn connect_activated(&self, callback: impl Fn(Period) + 'static) {
        self.callback.borrow_mut().replace(Rc::new(callback));
    }

    pub fn disconnect_activated(&self) {
        self.callback.borrow_mut().take();
    }

}

impl HasWidget<gtk::Widget> for HeaderRow {
    fn widget(&self) -> &gtk::Widget {
        self.horizontal_box.upcast_ref()
    }
}