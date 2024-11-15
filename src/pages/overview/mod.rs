mod header_row;
mod balance_row;
mod wallet_group;

use crate::data::Wallet;
use crate::prelude::*;
use crate::context::*;
use crate::utils::ScrollablePane;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Overview {
    scrollable_pane: ScrollablePane,
    balance_row: balance_row::BalanceRow,
    header_row: header_row::HeaderRow,
    insert_wallet_row: adw::ButtonRow,
    wallets_box: gtk::Box,

    context: RefCell<Context>
}

impl Overview {
    pub fn new() -> Rc<Self> {
        
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

        let this = Rc::new(Self {
            scrollable_pane,
            balance_row,
            header_row,
            insert_wallet_row,
            wallets_box,
            context: Default::default()
        });
        this.connect_balance_row_activated();
        this.connect_header_row_activated();
        this.connect_insert_wallet_row_activated();
        this
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

    fn connect_insert_wallet_row_activated(self: &Rc<Self>) {
        let this = Rc::downgrade(&self);
        self.insert_wallet_row.connect_activated(move |_| {
            let Some(this) = this.upgrade() else { unreachable!() };
            let context = this.context.borrow().clone();

            context
                .with_navigation_action(NavigationAction::navigate_to_new_wallet())
                .propagate()
        });
    }

    fn connect_header_row_activated(self: &Rc<Self>) {
        let this = Rc::downgrade(&self);
        self.header_row.connect_activated(move |new_period| {
            let Some(this) = this.upgrade() else { unreachable!() };
            let context = this.context.borrow().clone();

            let mut new_data = context.data().clone();
            new_data.period = new_period;

            context
                .with_data(new_data)
                .with_ui_action(UiAction::push_notification("Period changed"))
                .with_navigation_action(NavigationAction::NavigateToCurrent)
                .propagate()
        });
    }

    fn connect_balance_row_activated(self: &Rc<Self>) {
        let this = Rc::downgrade(&self);
        self.balance_row.connect_activated(move |new_currency| {
            let Some(this) = this.upgrade() else { unreachable!() };
            let context = this.context.borrow().clone();

            let data = context.data();
            if data.currency == new_currency {
                return;
            }

            let mut new_data = data.clone();
            new_data.currency = new_currency;

            context
                .with_data(new_data)
                .with_ui_action(UiAction::push_notification("Currency changed"))
                .with_navigation_action(NavigationAction::NavigateToCurrent)
                .propagate()
        });
    }

}

impl HasWidget<gtk::Widget> for Rc<Overview> {
    fn widget(&self) -> &gtk::Widget {
        self.scrollable_pane.widget()
    }
}

impl LifeCycle<NavigationAction> for Rc<Overview> {

    fn activate(&self, action: NavigationAction, context: &Context) {
        if !matches!(action, NavigationAction::NavigateToOverview) { unreachable!() };
        let mut data = context.data().clone();
        data.sort_by_name();

        self.balance_row.set_balance(data.total_balance_for_period(), data.currency);
        self.header_row.set_period(data.period);

        let wallets = data.wallets_for_period();
        self.add_wallet_groups(wallets, context);

        self.context.replace(context.clone());
    }

    fn deactivate(&self) {
        self.remove_wallet_groups();
        self.context.take();
    }
}

impl PageContent for Rc<Overview> {
    fn title(&self) -> &str {
        "Overview"
    }
}