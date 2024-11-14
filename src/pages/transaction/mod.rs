mod cycle_selector;
mod date_range_picker;

use crate::prelude::*;
use crate::context::*;
use crate::data::*;
use crate::utils::{AlertButton, AlertButtonType, ScrollablePane};

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
struct State {
    context: Context,
    wallet: Wallet,
    transaction: Transaction,
}

pub struct TransactionPage {
    scrollable_pane: ScrollablePane,
    cycle_selector_row: cycle_selector::CycleSelector,
    dates_pickers_row: date_range_picker::DateRangePicker,
    name_entry_row: adw::EntryRow,
    description_entry_row: adw::EntryRow,
    amount_spin_row: adw::SpinRow,
    save_button_row: adw::ButtonRow,
    remove_button_row: adw::ButtonRow,

    state: RefCell<State>,
}

impl TransactionPage {
    pub fn new() -> Rc<Self> {
        let dates_pickers_row = date_range_picker::DateRangePicker::new();
        let cycle_selector_row = cycle_selector::CycleSelector::new(dates_pickers_row.clone());

        let name_entry_row = Self::build_name_entry_row();
        let description_entry_row = Self::build_description_entry_row();
        let amount_spin_row = Self::build_amount_spin_row();
        let save_button_row = Self::build_save_button_row();
        let remove_button_row = Self::build_remove_button_row();

        let mut scrollable_pane = ScrollablePane::new();
        scrollable_pane.add_group(vec![&name_entry_row, &description_entry_row]);
        scrollable_pane.add_group(vec![&amount_spin_row]);
        scrollable_pane.add_group(vec![dates_pickers_row.widget()]);
        scrollable_pane.add_group(vec![cycle_selector_row.widget()]);
        scrollable_pane.add_separator();
        scrollable_pane.add_group(vec![&remove_button_row]);
        scrollable_pane.add_group(vec![&save_button_row]);
        
        let this = Rc::new(Self {
            cycle_selector_row,
            dates_pickers_row,
            name_entry_row,
            description_entry_row,
            amount_spin_row,
            save_button_row,
            remove_button_row,
            scrollable_pane,

            state: RefCell::new(State::default()),
        });
        this.connect_entries_changed();
        this.connect_save_button_activated();
        this.connect_remove_button_activated();
        this
    }

    fn build_name_entry_row() -> adw::EntryRow {
        let entry_row = adw::EntryRow::new();
        entry_row.set_enable_emoji_completion(true);
        entry_row.set_activates_default(true);
        entry_row.set_show_apply_button(false);
        entry_row.set_title("Name");
        entry_row
    }

    fn build_description_entry_row() -> adw::EntryRow {
        let description_row = adw::EntryRow::new();
        description_row.set_enable_emoji_completion(true);
        description_row.set_show_apply_button(false);
        description_row.set_title("Description");
        description_row
    }

    fn build_amount_spin_row() -> adw::SpinRow {
        let adjustment = gtk::Adjustment::new(
            0.0,
            i32::MIN as f64,
            u32::MAX as f64,
            1.0,
            1.0 * 10.0,
            0.0,
        );
        let spin_row = adw::SpinRow::new(Some(&adjustment), 10.0, 2);
        spin_row.set_value(0.0);
        spin_row.set_numeric(true);
        spin_row.set_title("Amount");
        spin_row
    }

    fn build_save_button_row() -> adw::ButtonRow {
        let button_row = adw::ButtonRow::new();
        button_row.set_title("Save");
        button_row.set_start_icon_name(Some("document-save-symbolic"));
        button_row.add_css_class("suggested-action");
        button_row.add_css_class("pill");
        button_row.set_sensitive(false);
        button_row
    }

    fn build_remove_button_row() -> adw::ButtonRow {
        let button_row = adw::ButtonRow::new();
        button_row.set_title("Remove");
        button_row.set_start_icon_name(Some("user-trash-symbolic"));
        button_row.add_css_class("destructive-action");
        button_row.add_css_class("pill");
        button_row.set_sensitive(false);
        button_row
    }

    fn get_data(&self) -> Transaction {
        let end_date = self.dates_pickers_row.get_end_date();
        let start_date = self.dates_pickers_row.get_start_date();
        let description = self.description_entry_row.text().to_string();
        let cycle = self.cycle_selector_row.get_selected_cycle();
        let end_date = if cycle == Cycle::OneTime {
            Some(chrono::Utc::now().date_naive())
        } else if start_date > end_date {
            None
        } else {
            Some(end_date)
        };
        Transaction {
            name: self.name_entry_row.text().to_string(),
            description: (!description.is_empty()).then(|| description),
            amount: self.amount_spin_row.text().parse().unwrap_or(0.0),
            start_date,
            end_date,
            cycle,
            id: self.state.borrow().transaction.id,
        }
    }

    fn set_data(&self, transaction: &Transaction) {
        if let Some(description) = &transaction.description {
            self.description_entry_row.set_text(description);
        }
        if let Some(end_time) = transaction.end_date {
            self.dates_pickers_row.set_end_date(end_time);
        }
        self.name_entry_row.set_text(&transaction.name);
        self.amount_spin_row.set_value(transaction.amount as f64);
        self.dates_pickers_row.set_start_date(transaction.start_date);
        self.cycle_selector_row.set_selected_cycle(transaction.cycle.clone());
        self.save_button_row.set_sensitive(false);
        self.remove_button_row.set_sensitive(transaction.is_created());
    }

    fn clear_data(&self) {
        self.name_entry_row.set_text("");
        self.description_entry_row.set_text("");
        self.amount_spin_row.set_value(0.0);
        self.dates_pickers_row.set_default_dates();
        self.cycle_selector_row.set_selected_cycle(Cycle::default());
    }

    fn handle_changes(&self) {
        let previous_data = &self.state.borrow().transaction;
        let data = self.get_data();
        if data.is_valid() && data.is_different(previous_data) {
            self.save_button_row.set_sensitive(true);
        } else {
            self.save_button_row.set_sensitive(false);
        }
    }

    fn connect_entries_changed(self: &Rc<Self>) {
        let this = self.clone();
        self.name_entry_row.connect_changed(move |_| this.handle_changes());

        let this = self.clone();
        self.description_entry_row.connect_changed(move |_| this.handle_changes());

        let this = self.clone();
        self.amount_spin_row.connect_changed(move |_| this.handle_changes());

        let this = self.clone();
        self.dates_pickers_row.connect_changed(move |_, _| this.handle_changes());
        
        let this = self.clone();
        self.cycle_selector_row.connect_selected(move |_| this.handle_changes());
    }

    fn handle_remove_confirmation(self: &Rc<Self>) {
        let state = self.state.borrow();
        let context = state.context.clone();
        let transaction_id = state.transaction.id;
        let mut wallet_data = state.wallet.clone();
        drop(state);

        wallet_data.remove_transaction_by_id(transaction_id);

        let mut data = context.data().clone();
        data.add_or_update_wallet(wallet_data);

        context
            .with_data(data)
            .with_ui_action(UiAction::push_notification("Transaction removed"))
            .with_navigation_action(NavigationAction::NavigateToPrevious)
            .propagate();
    }

    fn connect_remove_button_activated(self: &Rc<Self>) {
        let this = self.clone();
        self.remove_button_row.connect_activated(move |_| {
            let context = this.state.borrow().context.clone();
            let this_clone = this.clone();
            context.with_ui_action(UiAction::OpenAlertDialog {
                title: "Remove transaction".into(),
                message: "Are you sure you want to remove this transaction?".into(),
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

    fn connect_save_button_activated(self: &Rc<Self>) {
        let this = self.clone();

        self.save_button_row.connect_activated(move |_| {
            let state = this.state.borrow();
            let context = state.context.clone();
            let mut wallet_data = state.wallet.clone();
            drop(state);

            let mut new_data = context.data().clone();
            let transaction_data = this.get_data().assign_global_id();

            let wallet_id = wallet_data.id;
            let transaction_id = transaction_data.id;
            let transaction_name_lowercase = transaction_data.name.to_lowercase();

            if wallet_data.transactions.iter().any(|t|
                t.id != transaction_id &&
                t.name.to_lowercase() == transaction_name_lowercase &&
                (
                    t.start_date == transaction_data.start_date ||
                    (
                        t.start_date > transaction_data.start_date &&
                        t.cycle != Cycle::OneTime &&
                        transaction_data.cycle != Cycle::OneTime &&
                        (
                            Some(t.start_date) <= transaction_data.end_date ||
                            transaction_data.end_date.is_none()
                        )
                    )
                )
            ) {
                return context
                    .with_ui_action(UiAction::push_notification(
                        "A transaction with the same name already exists within the specified date range."
                    )).propagate();
            }

            wallet_data.add_or_update_transaction(transaction_data.clone());
            new_data.add_or_update_wallet(wallet_data);

            context
                .with_data(new_data)
                .with_navigation_action(NavigationAction::navigate_to_transaction(wallet_id, transaction_id))
                .with_ui_action(UiAction::push_notification("Transaction saved"))
                .propagate()
        });
    }

}

impl HasWidget<gtk::Widget> for Rc<TransactionPage> {
    fn widget(&self) -> &gtk::Widget {
        self.scrollable_pane.widget()
    }
}

impl LifeCycle<NavigationAction> for Rc<TransactionPage> {

    fn activate(&self, action: NavigationAction, context: &Context) {
        let NavigationAction::NavigateToTransaction {
            wallet: wallet_id,
            transaction: transaction_id,
        } = action else {
            unreachable!();
        };

        let wallet = context
            .data()
            .find_wallet_by_id(wallet_id)
            .cloned()
            .unwrap();

        let transaction = transaction_id
            .and_then(|id| wallet.find_transaction_by_id(id))
            .cloned()
            .unwrap_or_default();

        self.set_data(&transaction);
        *self.state.borrow_mut() = State {
            context: context.clone(),
            wallet,
            transaction
        };
    }

    fn deactivate(&self) {
        *self.state.borrow_mut() = State::default();
        self.clear_data();
    }
}

impl PageContent for Rc<TransactionPage> {
    fn title(&self) -> &str {
        "Transaction"
    }
}