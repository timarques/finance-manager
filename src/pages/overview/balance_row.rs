use crate::prelude::*;
use crate::utils::{ButtonList, PopoverExtension};
use crate::data::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct BalanceRow {
    income_label: gtk::Label,
    expense_label: gtk::Label,
    total_label: gtk::Label,
    popover: gtk::Popover,
    button_list: ButtonList<Currency>,
    action_row: adw::ActionRow,
    
    callback: Rc<RefCell<Option<Rc<dyn Fn(Currency) + 'static>>>>,
}

impl BalanceRow {

    pub fn new() -> Self {
        let (income_box, income_label) = Self::build_balance_box("Income", false);
        let (expense_box, expense_label) = Self::build_balance_box("Expense", false);
        let (total_box, total_label) = Self::build_balance_box("Total", true);
        let v_box = Self::build_box(vec![
            &income_box,
            &expense_box,
            &total_box
        ]);
        
        let button_list = Self::build_button_list();
        let popover = gtk::Popover::new();
            popover.set_button_list(&button_list);
            popover.set_parent(&v_box);
        let action_row = Self::build_action_row(&v_box);
        
        let this = Self {
            income_label,
            expense_label,
            total_label,
            popover,
            button_list,
            action_row,
            callback: Rc::new(RefCell::new(None)),
        };
        this.connect_action_row_activated();
        this.connect_button_list_activated();
        this
    }

    fn build_button_list() -> ButtonList<Currency> {
        let button_list = ButtonList::new(false);
        for currency in Currency::as_slice() {
            button_list.add_with_prefixed_label(currency, currency.as_symbol(), currency.as_long_str());
        }
        button_list
    }

    fn build_action_row(child: &impl IsA<gtk::Widget>) -> adw::ActionRow {
        let action_row = adw::ActionRow::new();
        action_row.set_title("Balance");
        action_row.add_suffix(child);
        action_row.set_activatable(true);
        action_row
    }

    fn build_box(labels: Vec<&impl IsA<gtk::Widget>>) -> gtk::Box {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        v_box.set_valign(gtk::Align::Center);
        v_box.set_hexpand(false);
        v_box.set_margin_top(10);
        v_box.set_margin_bottom(10);

        for label in labels {
            v_box.append(label);
        }

        v_box
    }

    fn build_label(text: &str, align_start: bool, is_heading: bool) -> gtk::Label {
        let label = gtk::Label::new(Some(text));
        label.set_halign(if align_start { gtk::Align::Start } else { gtk::Align::End });
        if is_heading {
            label.add_css_class("heading");
        } else {
            label.add_css_class("dim-label");
            label.add_css_class("caption");
        }
        label
    }
    
    fn build_balance_box(text: &str, is_total: bool) -> (gtk::Box, gtk::Label) {
        let description_label = Self::build_label(text, true, is_total);
        let amount_label = Self::build_label(&Currency::default().format_amount(0.0), false, is_total);
    
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        h_box.set_valign(gtk::Align::Center);
        h_box.append(&description_label);
        h_box.append(&amount_label);
        h_box.set_halign(gtk::Align::Fill);
        h_box.set_hexpand(true);
    
        if is_total {
            h_box.set_margin_top(5);
        }
    
        (h_box, amount_label)
    }

    fn connect_action_row_activated(&self) {
        let popover_weak = self.popover.downgrade();
        self.action_row.connect_activated(move |_| {
            if let Some(popover) = popover_weak.upgrade() {
                if !popover.is_visible() {
                    popover.popup();
                }
            }
        });
    }

    fn connect_button_list_activated(&self) {
        let callback = self.callback.clone();
        self.button_list.connect_activated(move |currency, _, is_active| {
            let callback = callback.borrow().clone();
            if let Some(callback) = callback {
                let new_currency = if is_active { currency } else { Currency::default() };
                callback(new_currency);
            }
        });
    }

    pub fn set_balance(&self, balance: Balance, currency: Currency) {
        self.button_list.activate_button(&currency);
        self.income_label.set_text(&currency.format_amount(balance.income));
        self.expense_label.set_text(&currency.format_amount(balance.expense));
        self.total_label.set_text(&currency.format_amount(balance.net_balance()));
    }

    pub fn connect_activated(&self, callback: impl Fn(Currency) + 'static) {
        self.callback.borrow_mut().replace(Rc::new(callback));
    }

}

impl HasWidget<gtk::Widget> for BalanceRow {
    fn widget(&self) -> &gtk::Widget {
        self.action_row.upcast_ref()
    }
}