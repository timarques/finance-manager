use crate::prelude::*;
use crate::context::*;
use crate::metadata;

pub struct NavigationPage {
    title_label: gtk::Label,
    navigation_page: adw::NavigationPage,
    revealer: gtk::Revealer,
    spinner: gtk::Spinner,
    page_content: Box<dyn PageContent>,
}

impl NavigationPage {
    pub fn new(page_content: impl PageContent + 'static) -> Self {
        let page_content = Box::new(page_content);
        let spinner = Self::build_spinner();
        let revealer = Self::build_revealer(page_content.widget());
        let overlay = Self::build_overlay(&spinner, &revealer);
        let title_label = Self::build_title_label();
        let header_bar = Self::build_header_bar(&title_label);
        let toolbar_view = Self::build_toolbar_view(&header_bar, &overlay);
        let navigation_page = Self::build_navigation_page(&toolbar_view);

        Self {
            title_label,
            navigation_page,
            revealer,
            spinner,
            page_content
        }
    }

    fn build_spinner() -> gtk::Spinner {
        let spinner = gtk::Spinner::new();
        spinner.stop();
        spinner.set_valign(gtk::Align::Center);
        spinner.set_halign(gtk::Align::Center);
        spinner
    }

    fn build_revealer(child: &impl IsA<gtk::Widget>) -> gtk::Revealer {
        let revealer = gtk::Revealer::new();
        revealer.set_child(Some(child));
        revealer.set_reveal_child(false);
        revealer.set_transition_type(gtk::RevealerTransitionType::SlideUp);
        revealer
    }

    fn build_title_label() -> gtk::Label {
        let label = gtk::Label::new(Some("Loading..."));
        label.set_halign(gtk::Align::Center);
        label.set_valign(gtk::Align::Center);
        label.add_css_class("title-3");
        label.set_margin_top(5);
        label.set_margin_bottom(5);
        label
    }

    fn build_header_bar(child: &impl IsA<gtk::Widget>) -> adw::HeaderBar {
        let header_bar = adw::HeaderBar::new();
        header_bar.set_show_back_button(true);
        header_bar.set_show_title(true);
        header_bar.set_title_widget(Some(child));
        header_bar
    }

    fn build_overlay(spinner: &gtk::Spinner, revealer: &gtk::Revealer) -> gtk::Overlay {
        let overlay = gtk::Overlay::new();
        overlay.add_overlay(spinner);
        overlay.set_child(Some(revealer));
        overlay
    }

    fn build_toolbar_view(header_bar: &adw::HeaderBar, overlay: &gtk::Overlay) -> adw::ToolbarView {
        let window_handle = gtk::WindowHandle::new();
        window_handle.set_child(Some(overlay));

        let toolbar_view = adw::ToolbarView::new();
        toolbar_view.set_content(Some(&window_handle));
        toolbar_view.add_top_bar(header_bar);
        toolbar_view
    }

    fn build_navigation_page(toolbar_view: &adw::ToolbarView) -> adw::NavigationPage {
        let navigation_page = adw::NavigationPage::new(
            toolbar_view,
            &metadata::APP_TITLE,
        );
        navigation_page.set_can_pop(true);
        navigation_page
    }

}

impl HasWidget<adw::NavigationPage> for NavigationPage {
    fn widget(&self) -> &adw::NavigationPage {
        &self.navigation_page
    }
}

impl LifeCycle<NavigationAction> for NavigationPage {

    fn activate(&self, action: NavigationAction, context: &Context) {
        self.spinner.start();
        self.page_content.activate(action, context);
        self.spinner.stop();
        self.revealer.set_reveal_child(true);
        self.title_label.set_text(&self.page_content.title());
    }

    fn deactivate(&self) {
        self.revealer.set_reveal_child(false);
        self.title_label.set_text("Loading...");
        self.page_content.deactivate();
    }
}