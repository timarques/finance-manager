use crate::prelude::*;
use crate::data::*;

fn build_amount_label(amount: f64, currency: Currency) -> gtk::Label {
    let label = gtk::Label::new(Some(&currency.format_amount(amount)));
    label.set_valign(gtk::Align::Center);
    label.set_halign(gtk::Align::Center);
    label.add_css_class("numeric");
    label.add_css_class("caption");
    label
}

pub enum ActivateType {
    Transaction(usize, usize),
    Wallet(usize)
}

pub struct TransactionRow {
    transaction_id: usize,
    wallet_id: usize,
    balance_label: gtk::Label,
    action_row: adw::ActionRow
}

impl TransactionRow {
    pub fn new(transaction: &Transaction, wallet: &Wallet) -> Self {
        let balance_label = build_amount_label(transaction.amount, wallet.currency);
        let action_row = Self::build_action_row(transaction, &balance_label);
        Self {
            transaction_id: transaction.id,
            wallet_id: wallet.id,
            balance_label,
            action_row
        }
    }

    fn build_cycle_label(cycle: Cycle) -> gtk::Label {
        let label = gtk::Label::new(Some(&cycle.to_string()));
        label.add_css_class("dim-label");
        label.add_css_class("caption");
        label.set_valign(gtk::Align::Center);
        label.set_halign(gtk::Align::End);
        label
    }

    fn build_suffix(transaction: &Transaction, balance_label: &gtk::Label) -> gtk::Box {
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        h_box.set_valign(gtk::Align::Center);
        h_box.append(&Self::build_cycle_label(transaction.cycle));
        h_box.append(&gtk::Separator::new(gtk::Orientation::Vertical));
        h_box.append(balance_label);
        h_box
    }

    fn build_action_row(transaction: &Transaction, balance_label: &gtk::Label) -> adw::ActionRow {
        let action_row = adw::ActionRow::new();
        action_row.set_activatable(true);
        action_row.add_suffix(&Self::build_suffix(transaction, balance_label));
        action_row.set_title(&transaction.name);
        if let Some(description) = &transaction.description {
            action_row.set_subtitle(&description);
        }
        action_row
    }

    fn connect_activated(&self, callback: impl Fn(ActivateType) + 'static) {
        let transaction_id = self.transaction_id;
        let wallet_id = self.wallet_id;
        self.action_row.connect_activated(move |_| {
            callback(ActivateType::Transaction(wallet_id, transaction_id));
        });
    }
}

pub struct WalletRow {
    wallet_id: usize,
    balance_label: gtk::Label,
    action_row: adw::ActionRow
}

impl WalletRow {
    pub fn new(wallet: &Wallet) -> Self {
        let balance_label = build_amount_label(wallet.balance().net_balance(), wallet.currency);
        let action_row = Self::build_action_row(wallet, &balance_label);
        Self {
            wallet_id: wallet.id,
            balance_label,
            action_row
        }
    }

    fn build_action_row(wallet: &Wallet, balance_label: &gtk::Label) -> adw::ActionRow {
        let action_row = adw::ActionRow::new();
            action_row.add_css_class("heading");
            action_row.set_activatable(true);
            action_row.set_title(wallet.name.as_str());
            action_row.add_suffix(balance_label);
            if let Some(description) = &wallet.description {
                action_row.set_subtitle(description);
            }
            action_row
    }

    fn connect_activated(&self, callback: impl Fn(ActivateType) + 'static) {
        let wallet_id = self.wallet_id;
        self.action_row.connect_activated(move |_| {
            callback(ActivateType::Wallet(wallet_id));
        });
    }
}

pub struct WalletGroup {
    wallet_row: WalletRow,
    transaction_rows: Vec<TransactionRow>,
    preferences_group: adw::PreferencesGroup
}

impl WalletGroup {

    pub fn new(wallet: &Wallet) -> Self {
        let wallet_row = WalletRow::new(wallet);
        let transaction_rows = Self::create_transaction_rows(wallet);
        let preferences_group = Self::build_preferences_group(&wallet_row, &transaction_rows);

        let this = Self {
            wallet_row,
            transaction_rows,
            preferences_group
        };
        this.resize_balance_labels();
        this
    }

    fn create_transaction_rows(wallet: &Wallet) -> Vec<TransactionRow> {
        wallet.transactions
            .iter()
            .map(|t| TransactionRow::new(t, wallet))
            .collect()
    }

    fn build_expander_action_row(transaction_rows: &Vec<TransactionRow>) -> adw::ActionRow {
        let nested_list_box = gtk::ListBox::new();
            nested_list_box.set_selection_mode(gtk::SelectionMode::None);
            nested_list_box.set_focusable(false);
            nested_list_box.add_css_class("boxed-list");
            nested_list_box.add_css_class("nested");

            for transaction_row in transaction_rows {
                nested_list_box.append(&transaction_row.action_row);
            }

        let expander_action_row = adw::ActionRow::new();
            expander_action_row.add_css_class("expander");
            expander_action_row.set_focusable(false);
            expander_action_row.set_activatable(false);
            expander_action_row.set_child(Some(&nested_list_box));
            expander_action_row
    }

    fn build_preferences_group(wallet_row: &WalletRow, transaction_rows: &Vec<TransactionRow>) -> adw::PreferencesGroup {
        let nested_list_box = Self::build_expander_action_row(transaction_rows);
        let preferences_group = adw::PreferencesGroup::new();
        preferences_group.add(&wallet_row.action_row);
        preferences_group.add(&nested_list_box);
        preferences_group
    }

    fn resize_balance_labels(&self) {
        let labels: Vec<_> = std::iter::once(&self.wallet_row.balance_label)
            .chain(self.transaction_rows.iter().map(|t| &t.balance_label))
            .collect();
            
        let max_width = labels.iter()
            .map(|label| label.measure(gtk::Orientation::Horizontal, -1).1)
            .max()
            .unwrap_or(0);
            
        labels.iter()
            .for_each(|label| label.set_width_request(max_width));
    }

    pub fn connect_activated(&self, callback: impl Fn(ActivateType) + Clone + 'static) {
        for transaction_row in &self.transaction_rows {
            transaction_row.connect_activated(callback.clone());
        }
        self.wallet_row.connect_activated(callback);
    }

}

impl HasWidget<gtk::Widget> for WalletGroup {
    fn widget(&self) -> &gtk::Widget {
        self.preferences_group.upcast_ref()
    }
}