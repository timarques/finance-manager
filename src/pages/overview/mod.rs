mod header_row;
mod balance_row;
mod wallet_group;

use gtk::glib::translate::FromGlib as _;

use crate::data::Wallet;
use crate::prelude::*;
use crate::context::*;
use crate::utils::ScrollablePane;

use std::cell::Cell;

pub struct Overview {
    scrollable_pane: ScrollablePane,
    balance_row: balance_row::BalanceRow,
    header_row: header_row::HeaderRow,
    insert_wallet_row: adw::ButtonRow,
    wallets_box: gtk::Box,

    insert_row_signal_id: Cell<isize>
}

impl Overview {
    pub fn new() -> Self {
        
        let wallets_box = gtk::Box::new(gtk::Orientation::Vertical, 20);
        let header_row = header_row::HeaderRow::new();
        let balance_row = balance_row::BalanceRow::new();
        let insert_wallet_row = Self::build_insert_button_row();

        let mut scrollable_pane = ScrollablePane::new();

        scrollable_pane.add_group(vec![header_row.widget()]);
        scrollable_pane.add_group(vec![&wallets_box]);
        scrollable_pane.add_group(vec![&insert_wallet_row]);
        scrollable_pane.add_separator();
        scrollable_pane.add_group(vec![balance_row.widget()]);

        Self {
            scrollable_pane,
            balance_row,
            header_row,
            insert_wallet_row,
            wallets_box,
            insert_row_signal_id: Cell::new(-1)
        }
    }

    fn build_insert_button_row() -> adw::ButtonRow {
        let button_row = adw::ButtonRow::new();
        button_row.set_activatable(true);
        button_row.set_title("Insert Wallet");
        button_row.set_start_icon_name(Some("list-add-symbolic"));
        button_row
    }

    fn handle_wallet_group_activated(activate_type: wallet_group::ActivateType, context: &Context) {
        match activate_type {
            wallet_group::ActivateType::Wallet(wallet_id) => {
                context
                    .clone()
                    .with_navigation_action(NavigationAction::navigate_to_wallet(wallet_id))
                    .propagate();
            }
            wallet_group::ActivateType::Transaction(wallet_id, transaction_id) => {
                context
                    .clone()
                    .with_navigation_action(NavigationAction::navigate_to_transaction(wallet_id, transaction_id))
                    .propagate();
            }
        }
    }

    fn add_wallet_groups(&self, wallets: Vec<Wallet>, context: &Context) {
        for wallet in wallets {
            let context = context.clone();
            let wallet_group = wallet_group::WalletGroup::new(&wallet);
            wallet_group.connect_activated(move |activate_type| {
                Self::handle_wallet_group_activated(activate_type, &context)
            });
            self.wallets_box.append(wallet_group.widget());
        }
    }

    fn remove_wallet_groups(&self) {
        while let Some(widget) = self.wallets_box.last_child() {
            self.wallets_box.remove(&widget);
        }
    }

    fn connect_insert_wallet_row_activated(&self, context: &Context) {
        let context = context.clone();
        let signal_handler_id = self.insert_wallet_row.connect_activated(move |_| {
            context
                .clone()
                .with_navigation_action(NavigationAction::navigate_to_new_wallet())
                .propagate()
        });

        self.insert_row_signal_id.set(unsafe { signal_handler_id.as_raw() as isize });
    }

    fn connect_header_row_activated(&self, context: &Context) {
        let context = context.clone();
        self.header_row.connect_activated(move |new_period| {
            let mut new_data = context.data().clone();
            new_data.period = new_period;

            context
                .clone()
                .with_data(new_data)
                .with_ui_action(UiAction::push_notification("Period changed"))
                .with_navigation_action(NavigationAction::NavigateToCurrent)
                .propagate()
        });
    }

    fn connect_balance_row_activated(&self, context: &Context) {
        let context = context.clone();
        self.balance_row.connect_activated(move |new_currency| {
            let old_data = context.data();
            if old_data.currency == new_currency {
                return;
            }

            let mut new_data = old_data.clone();
            new_data.currency = new_currency;

            context
                .clone()
                .with_data(new_data)
                .with_ui_action(UiAction::push_notification("Currency changed"))
                .with_navigation_action(NavigationAction::NavigateToCurrent)
                .propagate()
        });
    }

    fn connect_events(&self, context: &Context) {
        self.connect_insert_wallet_row_activated(context);
        self.connect_header_row_activated(context);
        self.connect_balance_row_activated(context);
    }

    fn disconnect_events(&self) {
        let insert_signal_handler_id = unsafe { gtk::glib::SignalHandlerId::from_glib(self.insert_row_signal_id.get() as u64) };
        self.insert_wallet_row.disconnect(insert_signal_handler_id);
        self.header_row.disconnect_activated();
        self.balance_row.disconnect_activated();
    }

}

impl HasWidget<gtk::Widget> for Overview {
    fn widget(&self) -> &gtk::Widget {
        self.scrollable_pane.widget()
    }
}

impl LifeCycle<NavigationAction> for Overview {

    fn activate(&self, action: NavigationAction, context: &Context) {
        if !matches!(action, NavigationAction::NavigateToOverview) { unreachable!() };
        let mut data = context.data().clone();
        data.sort_by_name();
        let wallets = data.wallets_for_period();
        self.balance_row.set_balance(data.total_balance_for_period(), data.currency);
        self.header_row.set_period(data.period);

        self.add_wallet_groups(wallets, context);

        self.connect_events(context);
    }

    fn deactivate(&self) {
        self.remove_wallet_groups();
        self.disconnect_events();
    }
}

impl PageContent for Overview {
    fn title(&self) -> &str {
        "Overview"
    }
}