use crate::prelude::*;
use crate::data::*;

use std::sync::LazyLock;

static MAX_CYCLE_WIDTH: LazyLock<i32> = LazyLock::new(|| {
    let label = gtk::Label::new(None);
    Cycle::as_slice().iter()
        .map(|cycle| {
            label.set_text(&cycle.to_string());
            label.measure(gtk::Orientation::Horizontal, -1).1
        })
        .max()
        .unwrap_or(60)
});

pub struct TransactionRow {
    transaction_id: usize,
    action_row: adw::ActionRow
}

impl TransactionRow {

    pub fn new(transaction: &Transaction, wallet: &Wallet) -> Self {
        let transaction_id = transaction.id;
        let action_row = Self::build_action_row(transaction, wallet);
        Self {
            transaction_id,
            action_row
        }
    }

    fn build_amount_label(transaction: &Transaction, currency: Currency) -> gtk::Label {
        let amount_label = gtk::Label::new(Some(&currency.format_amount(transaction.amount)));
            amount_label.set_halign(gtk::Align::End);
            amount_label.set_valign(gtk::Align::Center);
            amount_label.set_hexpand(true);
            amount_label.add_css_class("numeric");
            amount_label.add_css_class("caption");
        amount_label
    }

    fn build_cycle_label(transaction: &Transaction) -> gtk::Label {
        let cycle_label = gtk::Label::new(Some(&transaction.cycle.to_string()));
            cycle_label.set_halign(gtk::Align::End);
            cycle_label.set_valign(gtk::Align::Center);
            cycle_label.set_width_request(MAX_CYCLE_WIDTH.to_owned());
            cycle_label.add_css_class("caption");
        cycle_label
    }

    fn build_dates_box(transaction: &Transaction) -> gtk::Box {
        let start_date_label = gtk::Label::new(Some(&transaction.start_date.to_string()));
        start_date_label.set_halign(gtk::Align::End);
        start_date_label.add_css_class("dim-label");
        start_date_label.add_css_class("caption");
        start_date_label.add_css_class("numeric");

        let end_date_label = transaction.end_date.map(|end_date| {
            let label = gtk::Label::new(Some(&end_date.to_string()));
            label.set_halign(gtk::Align::End);
            label.add_css_class("dim-label");
            label.add_css_class("caption");
            label.add_css_class("numeric");
            label
        });

        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        v_box.set_valign(gtk::Align::Center);
        v_box.set_halign(gtk::Align::End);
        v_box.append(&start_date_label);
        if let Some(end_date_label) = end_date_label {
            v_box.append(&end_date_label);
        }
        v_box
    }

    fn build_build_action_row_suffix(transaction: &Transaction, wallet: &Wallet) -> gtk::Box {
        let amount_label = Self::build_amount_label(transaction, wallet.currency);
        let cycle_label = Self::build_cycle_label(transaction);
        let dates_box = Self::build_dates_box(transaction);

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
            h_box.set_valign(gtk::Align::Center);
            h_box.set_halign(gtk::Align::End);
            h_box.append(&amount_label);
            h_box.append(&gtk::Separator::builder().halign(gtk::Align::Center).build());
            h_box.append(&cycle_label);
            h_box.append(&gtk::Separator::builder().halign(gtk::Align::Center).build());
            h_box.append(&dates_box);

        h_box
    }

    fn build_action_row(transaction: &Transaction, wallet: &Wallet) -> adw::ActionRow {
        let action_row = adw::ActionRow::new();
            action_row.set_activatable(true);
            action_row.set_title(&transaction.name);
            action_row.add_suffix(&Self::build_build_action_row_suffix(transaction, wallet));
        if let Some(description) = &transaction.description {
            action_row.set_subtitle(&description);
        }

        let now = chrono::Utc::now().date_naive();
        if now > transaction.start_date && now < transaction.end_date.unwrap_or(now) {
            action_row.add_prefix(&gtk::Image::from_icon_name("diamond-filled-symbolic"));
        } else {
            action_row.add_prefix(&gtk::Image::from_icon_name("diamond-outline-thick-symbolic"));
        }

        action_row
    }

    pub fn connect_activate_event(&self, callback: impl Fn(usize) + 'static) {
        let transaction_id = self.transaction_id;
        self.action_row.connect_activated(move |_| {
            callback(transaction_id);
        });
    }

}

impl HasWidget<gtk::Widget> for TransactionRow {
    fn widget(&self) -> &gtk::Widget {
        self.action_row.upcast_ref()
    }
}