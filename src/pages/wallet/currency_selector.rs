use crate::prelude::*;
use crate::data::*;
use crate::utils::{ButtonList, PopoverExtension as _};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::LazyLock;

static MAX_CURRENCY_WIDTH: LazyLock<i32> = LazyLock::new(|| {
    let label = gtk::Label::new(None);
    let max_width = Currency::as_slice().iter()
        .map(|currency| {
            label.set_text(&currency.as_long_str());
            label.measure(gtk::Orientation::Horizontal, -1).1
        })
        .max()
        .unwrap_or(50);

    max_width
});

#[derive(Clone)]
pub struct CurrencySelector {
    main_button: gtk::Button,
    button_list: ButtonList<Currency>,
    popover: gtk::Popover,
    action_row: adw::ActionRow,
    callback: Rc<RefCell<Option<Rc<dyn Fn(Currency)>>>>,
}

impl CurrencySelector {

    pub fn new() -> Self {
        let button_list = Self::build_button_list();
        let main_button = Self::build_button(Currency::default());
        let action_row = Self::build_action_row(&main_button);
        let popover = gtk::Popover::new();
            popover.set_button_list(&button_list);
            popover.set_parent(&main_button);

        let this = Self {
            main_button,
            action_row,
            button_list,
            popover,
            callback: Rc::new(RefCell::new(None)),
        };
        this.connect_button_clicked();
        this.connect_action_row_activated();
        this.connect_button_list_activated();
        this.connect_popover_popdown();
        this
    }

    fn build_button_list() -> ButtonList<Currency> {
        let button_list = ButtonList::new(true);

        for currency in Currency::as_slice() {
            button_list.add_with_prefixed_label(currency, currency.as_symbol(), currency.as_long_str());
        }

        button_list
    }

    fn build_action_row(suffix: &impl IsA<gtk::Widget>) -> adw::ActionRow {
        let action_row = adw::ActionRow::new();
        action_row.set_activatable(true);
        action_row.set_title("Currency");
        action_row.add_suffix(suffix);
        action_row
    }

    fn build_button_child(currency: Currency) -> gtk::Box {
        let symbol = gtk::Label::new(Some(currency.as_symbol()));
        symbol.set_halign(gtk::Align::Start);
        let text = gtk::Label::new(Some(currency.as_long_str()));
        text.set_halign(gtk::Align::Center);
        text.set_hexpand(true);
        text.set_width_request(MAX_CURRENCY_WIDTH.to_owned());

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        h_box.set_valign(gtk::Align::Center);
        h_box.append(&symbol);
        h_box.append(&text);
        h_box
    }

    fn build_button(currency: Currency) -> gtk::Button {
        let button = gtk::Button::new();
        button.set_child(Some(&Self::build_button_child(currency)));
        button.set_valign(gtk::Align::Center);
        button.set_hexpand(false);
        button
    }

    fn connect_button_clicked(&self) {
        let popover_weak = self.popover.downgrade();
        self.main_button.connect_clicked(move |button| {
            if let Some(popover) = popover_weak.upgrade() { 
                if !popover.is_visible() {
                    popover.popup();
                    button.add_css_class("active");
                }
            }
        });
    }

    fn connect_action_row_activated(&self) {
        let main_button = self.main_button.downgrade();
        self.action_row.connect_activated(move |_| {
            let Some(main_button) = main_button.upgrade() else { return; };
            main_button.emit_clicked();
        });
    }

    fn connect_button_list_activated(&self) {
        let main_button = self.main_button.downgrade();
        let callback = self.callback.clone();
        self.button_list.connect_activated(move |currency, _, is_active| {
            let Some(main_button) = main_button.upgrade() else { return; };
            let mut currency = currency;
            if is_active {
                if currency.is_default() {
                    main_button.remove_css_class("active");
                } else {
                    main_button.add_css_class("active");
                }
            } else {
                currency = Currency::default();
            }
            main_button.set_child(Some(&Self::build_button_child(currency)));
            if let Some(callback) = callback.borrow().as_ref() {
                callback(currency);
            }
        });
    }

    fn connect_popover_popdown(&self) {
        let main_button = self.main_button.downgrade();
        let button_list = self.button_list.clone();
        self.popover.connect_closed(move |_| {
            let Some(main_button) = main_button.upgrade() else { return; };
            if button_list.active_key().unwrap_or(Currency::default()).is_default() {
                main_button.remove_css_class("active");
            }
        });
    }

    pub fn connect_activate_event(&self, callback: impl Fn(Currency) + Clone + 'static) {
        self.callback.borrow_mut().replace(Rc::new(callback));
    }

    pub fn set_activated(&self, currency: Currency) {
        self.button_list.activate_button(&currency);
        if currency != Currency::default() {
            self.main_button.add_css_class("active");
        } else {
            self.main_button.remove_css_class("active");
        }
        self.main_button.set_child(Some(&Self::build_button_child(currency)));
    }

    pub fn get_activated(&self) -> Currency {
        self.button_list.active_key().unwrap_or(Currency::default())
    }

}

impl HasWidget<gtk::Widget> for CurrencySelector {
    fn widget(&self) -> &gtk::Widget {
        self.action_row.upcast_ref()
    }
}