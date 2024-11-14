mod status;
mod overview;
mod transaction;
mod wallet;
mod navigation_page;

use crate::prelude::*;
use crate::context::*;

use navigation_page::NavigationPage;
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    #[default]
    Status,
    Overview,
    Transaction,
    Wallet
}

pub struct Pages {
    navigation_view: adw::NavigationView,
    pages: HashMap<Page, NavigationPage>,
    history: RefCell<Vec<NavigationAction>>,
    last_context: RefCell<Context>
}

impl Pages {

    pub fn new() -> Rc<Self> {
        let pages = Self::create_pages();
        let navigation_view = Self::build_navigation_view();
        Self::add_pages(&navigation_view, &pages);
        let this = Rc::new(Self {
            pages,
            navigation_view,
            history: RefCell::new(Vec::new()),
            last_context: RefCell::new(Context::default())
        });
        this.connect_events();
        this
    }

    fn create_pages() -> HashMap<Page, NavigationPage> {
        let mut pages = HashMap::new();
        let status_page = NavigationPage::new(status::Status::new());
        let overview = NavigationPage::new(overview::Overview::new());
        let transaction = NavigationPage::new(transaction::TransactionPage::new());
        let wallet = NavigationPage::new(wallet::WalletPage::new());
        pages.insert(Page::Status, status_page);
        pages.insert(Page::Overview, overview);
        pages.insert(Page::Transaction, transaction);
        pages.insert(Page::Wallet, wallet);
        pages
    }

    fn add_pages(navigation_view: &adw::NavigationView, pages: &HashMap<Page, NavigationPage>) {
        let default_page = pages.get(&Page::default()).expect("Failed to get default page");
        navigation_view.add(default_page.widget());
        for (page, navigation_page) in pages.iter() {
            if page == &Page::default() {
                continue
            }
            navigation_view.add(navigation_page.widget());
        }
    }

    fn build_navigation_view() -> adw::NavigationView {
        let navigation_view = adw::NavigationView::new();
        navigation_view.set_pop_on_escape(true);
        navigation_view.set_animate_transitions(true);
        navigation_view
    }

    fn get_navigation_stack(&self) -> Vec<adw::NavigationPage> {
        self.navigation_view
            .navigation_stack()
            .iter()
            .filter_map(Result::ok)
            .collect()
    }

    const fn get_page_from_action(&self, action: &NavigationAction) -> Option<Page> {
        match action {
            NavigationAction::NavigateToStatus => Some(Page::Status),
            NavigationAction::NavigateToOverview => Some(Page::Overview),
            NavigationAction::NavigateToTransaction { wallet: _, transaction: _ } => Some(Page::Transaction),
            NavigationAction::NavigateToWallet { wallet: _ } => Some(Page::Wallet),
            _ => None
        }
    }

    fn get_navigation_page_from_action(&self, action: &NavigationAction) -> Option<&NavigationPage> {
        self.get_page_from_action(action)
            .and_then(|p| self.pages.get(&p))
    }

    fn push_navigation_page(&self, navigation_page: &adw::NavigationPage) {
        let navigation_view = self.navigation_view.clone();
        let navigation_page = navigation_page.clone();
        let navigation_stack = self.get_navigation_stack();
        gtk::glib::idle_add_local_once(move || {
            if navigation_stack.contains(&navigation_page) {
                navigation_view.pop_to_page(&navigation_page);
            } else {
                navigation_view.push(&navigation_page);
            }
        });
    }

    fn pop_navigation_action(&self) {
        let mut history = self.history.borrow_mut();
        if history.len() < 2 {
            panic!("Navigation history is empty");
        }
    
        let last_action = history.pop().unwrap();
        let next_action = history.pop().unwrap();
        let context = self.last_context.borrow();
        
        if self.get_page_from_action(&last_action)
            .and_then(|p| self.pages.get(&p))
            .map(|p| p.deactivate())
            .is_none() {
                panic!("bad history");
            };

        if self.get_page_from_action(&next_action)
            .and_then(|p| self.pages.get(&p))
            .map(|p| p.activate(next_action, &context))
            .is_none() {
                panic!("bad history");
            }

        history.push(next_action);
    }

    fn connect_events(self: &Rc<Self>) {
        let this = self.clone();
        self.navigation_view.connect_popped(move |nv, _| {
            let result = this.history
                .borrow()
                .last()
                .and_then(|a| this.get_page_from_action(a))
                .and_then(|p| this.pages.get(&p))
                .map(|p| Some(p.widget()) == nv.visible_page().as_ref());
            if result == Some(false) {
                this.pop_navigation_action()
            }
        });
    }

}

impl Propagator<NavigationAction> for Rc<Pages> {
    fn propagate(&self, action: NavigationAction, context: &Context) {
        self.last_context.replace(context.clone());

        if action.is_navigation_previous() {
            self.pop_navigation_action();
            self.navigation_view.pop();
            return
        } else if action.is_navigation_current() {
            if let Some(last_action) = self.history.borrow().last() {
                let page = self.get_navigation_page_from_action(last_action).unwrap();
                page.deactivate();
                page.activate(last_action.clone(), context);
            }
            return
        }

        let mut history = self.history.borrow_mut();
        let last_action = history.last();
        let current_page_type = self.get_page_from_action(&action).unwrap();
        let previous_page_type = last_action.and_then(|a| self.get_page_from_action(&a));
        let current_navigation_page = self.pages.get(&current_page_type).unwrap();
        let is_same_action = Some(current_page_type) == previous_page_type;

        if is_same_action {
            current_navigation_page.deactivate();
        } else {
            history.push(action);
        }

        current_navigation_page.activate(action.clone(), context);
        self.push_navigation_page(current_navigation_page.widget());

        if is_same_action {
            return
        }

        if let Some(previous_page_type) = previous_page_type {
            self
                .pages
                .get(&previous_page_type)
                .unwrap()
                .deactivate();
        }
    }
}

impl HasWidget<gtk::Widget> for Rc<Pages> {
    fn widget(&self) -> &gtk::Widget {
        self.navigation_view.upcast_ref()
    }
}
