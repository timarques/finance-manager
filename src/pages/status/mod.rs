mod buttons;

use crate::context::*;
use crate::data::Data;
use crate::prelude::*;
use crate::metadata;

use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

use gtk::{gdk, gio};

pub struct Status {
    status_page: adw::StatusPage,
    drop_target: gtk::DropTarget,
    buttons: buttons::Buttons,
    context: Rc<RefCell<Context>>,
}

impl Status {

    pub fn new() -> Self {
        let buttons = buttons::Buttons::new();
        let status_page = Self::build_status_page(&buttons);
        let drop_target = gtk::DropTarget::new(gio::File::static_type(), gdk::DragAction::COPY);
        status_page.add_controller(drop_target.clone());

        let this = Self {
            status_page,
            buttons,
            drop_target,
            context: Rc::new(RefCell::new(Default::default())),
        };
        this.connect_drag_and_drop();
        this.connect_buttons_events();
        this
    }

    fn build_status_child(buttons: &buttons::Buttons) -> adw::Clamp {
        let clamp = adw::Clamp::new();
        clamp.set_child(Some(buttons.widget()));
        clamp.set_maximum_size(300);
        clamp
    }

    fn build_status_page(buttons: &buttons::Buttons) -> adw::StatusPage {
        let status_page = adw::StatusPage::new();
        // status_page.set_title(metadata::APP_TITLE);
        status_page.set_description(Some("Select a data document or create a new one"));
        status_page.set_icon_name(Some(metadata::APP_ICON_NAME));
        status_page.set_child(Some(&Self::build_status_child(buttons)));
        status_page
    }

    fn load_previous_file(context: Context) {
        let mut context = context;
        if !context.file().exists() {
            let Some(file) = context
                .directory()
                .find_most_recent_data_file()
                .ok()
                .flatten() else {
                    return context
                        .with_ui_action(UiAction::push_notification("No data files found"))
                        .propagate();
                };
            context = context.with_file(file);
        }

        if let Ok(data) = context.file().load() {
            context
                .with_data(data)
                .with_navigation_action(NavigationAction::NavigateToOverview)
                .propagate();
        } else {
            context
                .with_ui_action(UiAction::push_notification("Failed to load data"))
                .propagate();
        }
    }

    fn handle_file_loaded(path: Option<PathBuf>, context: &Context) {
        let Some(path) = path else { return };
        let data_file = DataFile::new(path);
        if let Ok(data) = data_file.load() {
            context
                .clone()
                .with_file(data_file)
                .with_data(data)
                .with_navigation_action(NavigationAction::NavigateToOverview)
                .propagate();
        } else {
            context
                .clone()
                .with_ui_action(UiAction::push_notification("Failed to load data"))
                .propagate();
        }
    }

    fn load_file(context: Context) {
        context
            .with_ui_action(UiAction::open_file_chooser(Self::handle_file_loaded))
            .propagate();
    }

    fn create_file(context: Context) {
        let data = Data::default();
        let Ok(data_file) = context
            .directory()
            .create_new_data_file() else {
                return context
                    .with_ui_action(UiAction::push_notification("Failed to create new data file"))
                    .propagate();
            };

        context
            .with_file(data_file)
            .with_data(data)
            .with_navigation_action(NavigationAction::NavigateToOverview)
            .propagate();
    }

    fn show_about(context: Context) {
        context.with_ui_action(UiAction::OpenAboutDialog).propagate();
    }

    fn connect_buttons_events(&self) {
        let context = self.context.clone();
        self.buttons.connect_events(move |event| {
            let context = context.borrow().clone();
            match event {
                buttons::ButtonClick::LoadPrevious => Self::load_previous_file(context),
                buttons::ButtonClick::Create => Self::create_file(context),
                buttons::ButtonClick::Load => Self::load_file(context),
                buttons::ButtonClick::About => Self::show_about(context),
            }
        });
    }

    fn connect_drag_and_drop(&self) {
        let context = self.context.clone();
        self.drop_target.connect_accept(move |_, drop| {
            let formats = drop.formats();
            for mime_type in formats.mime_types() {
                if mime_type.contains("application/json") || mime_type.contains("text/plain") {
                    return true
                }
            }
            false
        });
        self.drop_target.connect_drop(move |_, value, _, _| {
            if let Some(file_path) = value.get::<gio::File>().ok().and_then(|f| f.path()) {
                let context = context.borrow();
                Self::handle_file_loaded(Some(file_path), &context);
                true
            } else {
                false
            }
        });
    }

    fn has_recent_file(&self, context: &Context) -> bool {
        context
            .file()
            .exists() 
        || context
            .directory()
            .find_most_recent_data_file()
            .ok()
            .flatten()
            .is_some()
    }

}

impl HasWidget<gtk::Widget> for Status {
    fn widget(&self) -> &gtk::Widget {
        self.status_page.upcast_ref()
    }
}

impl LifeCycle<NavigationAction> for Status {

    fn activate(&self, action: NavigationAction, context: &Context) {
        if !matches!(action, NavigationAction::NavigateToStatus) { unreachable!() };

        self.buttons.set_load_previous_button_sensitive(self.has_recent_file(&context));
        self.context.replace(context.clone());
    }

    fn deactivate(&self) {
        self.context.replace(Context::default());
    }

}

impl PageContent for Status {
    fn title(&self) -> &str {
        &metadata::APP_TITLE
    }
}