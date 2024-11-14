mod currency_selector;
mod transaction_row;

use crate::prelude::*;
use crate::context::*;
use crate::data::*;
use crate::utils::{ScrollablePane, AlertButton, AlertButtonType};

use std::cell::RefCell;
use std::rc::Rc;

enum ButtonType {
    Suggested,
    Destructive,
    None
}

#[derive(Default)]
struct State {
    context: Context,
    wallet: Wallet,
    rows: Vec<transaction_row::TransactionRow>,
}

pub struct WalletPage {
    scrollable_pane: ScrollablePane,
    name_entry_row: adw::EntryRow,
    description_entry_row: adw::EntryRow,
    currency_row: currency_selector::CurrencySelector,
    save_button_row: adw::ButtonRow,
    remove_button_row: adw::ButtonRow,
    insert_transaction_button_row: adw::ButtonRow,
    transactions_list_box: gtk::ListBox,

    state: RefCell<State>,
}

impl WalletPage {
    pub fn new() -> Rc<Self> {
        let name_entry_row = Self::build_name_entry_row();
        let description_entry_row = Self::build_description_entry_row();
        let transactions_list_box = Self::build_list_box();
        let currency_row = currency_selector::CurrencySelector::new();

        let save_button_row = Self::build_button_row("Save", "document-save-symbolic", ButtonType::Suggested);
        let remove_button_row = Self::build_button_row("Remove", "user-trash-symbolic", ButtonType::Destructive);
        let insert_transaction_button_row = Self::build_button_row("Insert", "list-add-symbolic", ButtonType::None);

        let mut scrollable_pane = ScrollablePane::new();
            scrollable_pane.add_header("Wallet");
            scrollable_pane.add_group(vec![&name_entry_row, &description_entry_row]);
            scrollable_pane.add_group(vec![currency_row.widget()]);
            scrollable_pane.add_separator();
            scrollable_pane.add_header("Transactions");
            scrollable_pane.add_group(vec![&transactions_list_box]);
            scrollable_pane.add_group(vec![&insert_transaction_button_row]);
            scrollable_pane.add_separator();
            scrollable_pane.add_group(vec![&remove_button_row]);
            scrollable_pane.add_group(vec![&save_button_row]);

        let this = Rc::new(Self {
            name_entry_row,
            description_entry_row,
            insert_transaction_button_row,
            save_button_row,
            remove_button_row,
            currency_row,
            transactions_list_box,
            scrollable_pane,

            state: RefCell::new(State::default()),
        });
        this.connect_insert_transaction_row_event();
        this.connect_entries_change_event();
        this.connect_save_event();
        this.connect_remove_event();
        this
    }

    fn build_list_box() -> gtk::ListBox {
        let list_box = gtk::ListBox::new();
        list_box.set_selection_mode(gtk::SelectionMode::None);
        list_box.add_css_class("boxed-list");
        list_box.set_focusable(false);
        list_box
    }

    fn build_name_entry_row() -> adw::EntryRow {
        let entry_row = adw::EntryRow::new();
        entry_row.set_title("Name");
        entry_row
    }

    fn build_description_entry_row() -> adw::EntryRow {
        let entry_row = adw::EntryRow::new();
        entry_row.set_enable_emoji_completion(true);
        entry_row.set_show_apply_button(false);
        entry_row.set_title("Description");
        entry_row
    }

    fn build_button_row(title: &str, icon: &str, button_type: ButtonType) -> adw::ButtonRow {
        let button_row = adw::ButtonRow::new();
        button_row.set_title(title);
        button_row.set_start_icon_name(Some(icon));
        match button_type {
            ButtonType::Suggested => button_row.add_css_class("suggested-action"),
            ButtonType::Destructive => button_row.add_css_class("destructive-action"),
            ButtonType::None => {}
        }
        button_row
    }

    fn create_transaction_rows(&self, data: &Wallet) {
        let mut state_mut_ref = self.state.borrow_mut();
        for transaction in data.transactions.iter() {
            let transaction_row = transaction_row::TransactionRow::new(transaction, data);
            self.transactions_list_box.append(transaction_row.widget());
            state_mut_ref.rows.push(transaction_row);
        }
    }

    fn remove_transaction_rows(&self) {
        self.transactions_list_box.remove_all();
        self.state.borrow_mut().rows.clear();
    }

    fn set_data(&self, data: &Wallet) {
        if let Some(description) = &data.description {
            self.description_entry_row.set_text(description);
        }

        let scroll_page_children = self.scrollable_pane.get_children();
        for (index, child) in scroll_page_children.iter().enumerate() {
            if !data.is_created() && [3, 4, 5, 6, 8].contains(&index) {
                child.set_visible(false);
            } else if data.transactions.is_empty() && index == 5 {
                child.set_visible(false);
            } else {
                child.set_visible(true);
            }
        }

        if !data.transactions.is_empty() {
            self.create_transaction_rows(&data);
        }

        self.name_entry_row.set_text(&data.name);
        self.currency_row.set_activated(data.currency);
        self.save_button_row.set_sensitive(false);
    }

    fn get_data(&self) -> Wallet {
        let name = self.name_entry_row.text().to_string();
        let description_text = self.description_entry_row.text().to_string();
        let description = if description_text.is_empty() { None } else { Some(description_text) };
        let currency = self.currency_row.get_activated();
        Wallet {
            name,
            description,
            currency,
            ..self.state.borrow().wallet.clone()
        }
    }

    fn clear_data(&self) {
        self.name_entry_row.set_text("");
        self.description_entry_row.set_text("");
        self.currency_row.set_activated(Currency::default());
        self.remove_transaction_rows();
    }

    fn handle_changes(&self) {
        let previous_data = &self.state.borrow().wallet;
        let data = self.get_data();
        if data.is_valid() && data.is_different(&previous_data) {
            self.save_button_row.set_sensitive(true);
        } else {
            self.save_button_row.set_sensitive(false);
        }
    }

    fn connect_entries_change_event(self: &Rc<Self>) {
        let this_clone = self.clone();
        self.name_entry_row.connect_changed(move |_| {
            this_clone.handle_changes();
        });

        let this_clone = self.clone();
        self.description_entry_row.connect_changed(move |_| {
            this_clone.handle_changes();
        });

        let this_clone = self.clone();
        self.currency_row.connect_activate_event(move |_| {
            this_clone.handle_changes();
        });
    }

    fn connect_insert_transaction_row_event(self: &Rc<Self>) {
        let this = self.clone();
        self.insert_transaction_button_row.connect_activated(move |_| {
            let state = this.state.borrow();
            let context = state.context.clone();
            let wallet_id = state.wallet.id;
            drop(state);

            context
                .with_navigation_action(NavigationAction::navigate_to_new_transaction(wallet_id))
                .propagate();
        });
    }

    fn connect_transaction_row_activate_event(self: &Rc<Self>) {
        for transaction_row in self.state.borrow().rows.iter() {
            let this = self.clone();
            transaction_row.connect_activate_event(move |transaction| {
                let state = this.state.borrow();
                let context = state.context.clone();
                let wallet_id = state.wallet.id;
                drop(state);

                context
                    .with_navigation_action(NavigationAction::navigate_to_transaction(wallet_id, transaction))
                    .propagate();
            });
        }
    }

    fn connect_save_event(self: &Rc<Self>) {
        let this = self.clone();
        self.save_button_row.connect_activated(move |_| {

            let context = this.state.borrow().context.clone();
            let mut data = context.data().clone();
            let wallet = this.get_data().assign_global_id();
            let wallet_id = wallet.id;

            data.add_or_update_wallet(wallet);

            context
                .with_data(data)
                .with_navigation_action(NavigationAction::navigate_to_wallet(wallet_id))
                .with_ui_action(UiAction::push_notification("Wallet saved"))
                .propagate()
        });
    }

    fn handle_remove_confirmation(self: &Rc<Self>) {
        let state = self.state.borrow();
        let context = state.context.clone();
        let wallet_id = state.wallet.id;
        drop(state);

        let mut data = context.data().clone();
        data.remove_wallet_by_id(wallet_id);

        context
            .with_data(data)
            .with_ui_action(UiAction::push_notification("Wallet removed"))
            .with_navigation_action(NavigationAction::NavigateToPrevious)
            .propagate();
    }

    fn connect_remove_event(self: &Rc<Self>) {
        let this = self.clone();
        self.remove_button_row.connect_activated(move |_| {
            let context = this.state.borrow().context.clone();
            let this_clone = this.clone();
            context.with_ui_action(UiAction::OpenAlertDialog {
                title: "Remove wallet".into(),
                message: "Are you sure you want to remove this wallet?".into(),
                buttons: vec![
                    AlertButton::cancel(),
                    AlertButton::remove().destructive(),
                ],
                callback: Box::new(move |button| {
                    if let Some(button) = button {
                        if button.button_type == AlertButtonType::Remove {
                            this_clone.handle_remove_confirmation();
                        }
                    }
                })
            }).propagate();
        });
    }

}

impl HasWidget<gtk::Widget> for Rc<WalletPage> {
    fn widget(&self) -> &gtk::Widget {
        self.scrollable_pane.widget()
    }
}

impl LifeCycle<NavigationAction> for Rc<WalletPage> {

    fn activate(&self, action: NavigationAction, context: &Context) {
        let NavigationAction::NavigateToWallet {
            wallet: wallet_id
        } = action else {
            unreachable!();
        };

        let wallet = match wallet_id {
            Some(id) => context
                .data()
                .find_wallet_by_id(id)
                .cloned()
                .unwrap(),
            None => {
                let mut w = Wallet::default();
                w.currency = context.data().currency;
                w
            },
        };

        self.set_data(&wallet);
        self.connect_transaction_row_activate_event();
        *self.state.borrow_mut() = State {
            context: context.clone(),
            rows: Vec::new(),
            wallet
        };
    }

    fn deactivate(&self) {
        self.clear_data();
        *self.state.borrow_mut() = State::default();
    }

}

impl PageContent for Rc<WalletPage> {
    fn title(&self) -> &str {
        "Wallet"
    }
}