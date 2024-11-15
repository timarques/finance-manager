use crate::metadata;
use crate::pages::Pages;
use crate::window::Window;
use crate::context::*;
use crate::prelude::*;

#[derive(Clone)]
pub struct App {
    application: adw::Application,
}

impl App {

    pub fn new() -> Self {
        let application = adw::Application::new(Some(&metadata::APP_ID), Default::default());
        Self {
            application,
        }
    }

    fn setup_context(&self, application: &adw::Application) {
        let pages = Pages::new();
        let window = Window::new(application, &pages);
        let directory = DataDirectory::from_user_data_dir();
        directory.ensure_exists().expect("Failed to ensure that the data directory exists");
        let context = Context::new(directory, window, pages);
        context.with_navigation_action(NavigationAction::NavigateToStatus).propagate();
    }

    fn setup_resources() {
        gtk::glib::set_application_name(metadata::APP_TITLE);
        gtk::glib::set_prgname(Some(metadata::APP_NAME));
        gtk::gio::resources_register_include!("compiled.gresource").expect("failed to register resources");

        let icon_theme = gtk::IconTheme::default();
            icon_theme.add_resource_path(&format!("{}/icons", metadata::APP_RESOURCE_PATH));
            icon_theme.add_resource_path(&format!("{}/icons/scalable/actions", metadata::APP_RESOURCE_PATH));

        let css_provider = gtk::CssProvider::new();
            css_provider.load_from_resource(&format!("{}/styles.css", metadata::APP_RESOURCE_PATH));

        let style_manager = adw::StyleManager::default();
            style_manager.set_color_scheme(adw::ColorScheme::PreferDark);

        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not connect to a display."),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    pub fn init(&self) {
        let this = self.clone();
        self.application.connect_activate(move |app| {
            Self::setup_context(&this, app);
        });
        self.application.connect_startup(|_| {
            Self::setup_resources();
        });
        self.application.run();
    }

}