use crate::context::Context;
use crate::utils::AlertButton;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationAction {
    NavigateToStatus,
    NavigateToOverview,
    NavigateToWallet { wallet: Option<usize> },
    NavigateToTransaction { wallet: usize, transaction: Option<usize> },
    NavigateToPrevious,
    NavigateToCurrent,
}

impl NavigationAction {

    #[inline]
    pub const fn is_navigation_previous(&self) -> bool {
        matches!(self, NavigationAction::NavigateToPrevious)
    }

    #[inline]
    pub const fn is_navigation_current(&self) -> bool {
        matches!(self, NavigationAction::NavigateToCurrent)
    }

    pub fn navigate_to_new_wallet() -> Self {
        NavigationAction::NavigateToWallet {
            wallet: None
        }
    }

    #[inline]
    pub const fn navigate_to_wallet(wallet: usize) -> Self {
        NavigationAction::NavigateToWallet {
            wallet: Some(wallet)
        }
    }

    pub fn navigate_to_new_transaction(wallet: usize) -> Self {
        NavigationAction::NavigateToTransaction {
            wallet,
            transaction: None
        }
    }

    #[inline]
    pub const fn navigate_to_transaction(wallet: usize, transaction: usize) -> Self {
        NavigationAction::NavigateToTransaction {
            wallet,
            transaction: Some(transaction)
        }
    }
}

pub enum UiAction {
    PushNotification { message: String },
    OpenAlertDialog {
        title: String,
        message: String,
        buttons: Vec<AlertButton>,
        callback: Box<dyn FnOnce(Option<&AlertButton>) + 'static>
    },
    OpenFileChooserDialog { callback: Box<dyn FnOnce(Option<PathBuf>, &Context) + 'static> },
    OpenAboutDialog,
}

impl UiAction {

    pub fn push_notification(message: impl Into<String>) -> Self {
        UiAction::PushNotification { 
            message: message.into() 
        }
    }

    pub fn open_file_chooser(callback: impl Fn(Option<PathBuf>, &Context) + 'static) -> Self {
        UiAction::OpenFileChooserDialog {
            callback: Box::new(callback)
        }
    }


}

pub trait Action {}
impl Action for UiAction {}
impl Action for NavigationAction {}