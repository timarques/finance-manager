use crate::prelude::*;

pub enum ButtonClick {
    LoadPrevious,
    Load,
    Create,
    About
}

pub struct Buttons {
    container: gtk::Box,
    load_previous_button: gtk::Button,
    about_button: gtk::Button,
    create_button: gtk::Button,
    load_button: gtk::Button,
}

impl Buttons {

    pub fn new() -> Self {
        let load_previous_button = Self::build_button("document-open-recent-symbolic", "Load Previous document");
            load_previous_button.add_css_class("suggested-action");
        let create_button = Self::build_button("document-new-symbolic", "Create new document");
        let load_button = Self::build_button("document-open-symbolic", "Load document");
        let about_button = Self::build_about_button();
        let container = Self::build_container(&load_previous_button, &about_button, &load_button, &create_button);
        
        Self {
            container,
            load_previous_button,
            about_button,
            create_button,
            load_button,
        }
    }

    fn build_button(icon_name: &str, text: &str) -> gtk::Button {
        let button = gtk::Button::new();
        let image = gtk::Image::from_icon_name(icon_name);
        let label = gtk::Label::new(Some(text));
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        h_box.append(&image);
        h_box.append(&label);
        h_box.set_halign(gtk::Align::Center);
        button.set_child(Some(&h_box));
        button.add_css_class("pill");
        button
    }

    fn build_about_button() -> gtk::Button {
        let button = Self::build_button("help-about-symbolic", "About");
        button.add_css_class("link");
        button.set_focusable(false);
        button
    }

    fn build_alternative_container(create_button: &gtk::Button, load_button: &gtk::Button) -> gtk::Box {
        let alternative_buttons = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        alternative_buttons.set_homogeneous(true);
        alternative_buttons.append(create_button);
        alternative_buttons.append(load_button);
        alternative_buttons
    }

    fn build_container(
        load_previous_button: &gtk::Button,
        about_button: &gtk::Button,
        load_button: &gtk::Button,
        create_button: &gtk::Button
    ) -> gtk::Box {
        let buttons = gtk::Box::new(gtk::Orientation::Vertical, 10);
        buttons.set_homogeneous(true);
        buttons.append(load_previous_button);
        buttons.append(&Self::build_alternative_container(create_button, load_button));
        buttons.append(about_button);
        buttons
    }

    pub fn connect_events(&self, callback: impl Fn(ButtonClick) + Clone + 'static) {
        let callback_clone = callback.clone();
        self.about_button.connect_clicked(move |_| callback_clone(ButtonClick::About));

        let callback_clone = callback.clone();
        self.load_previous_button.connect_clicked(move |_| callback_clone(ButtonClick::LoadPrevious));

        let callback_clone = callback.clone();
        self.create_button.connect_clicked(move |_| callback_clone(ButtonClick::Create));

        let callback_clone = callback.clone();
        self.load_button.connect_clicked(move |_| callback_clone(ButtonClick::Load));
    }

    pub fn set_load_previous_button_sensitive(&self, is_sensitive: bool) {
        self.load_previous_button.set_sensitive(is_sensitive);
    }

}

impl HasWidget<gtk::Widget> for Buttons {
    fn widget(&self) -> &gtk::Widget {
        self.container.upcast_ref()
    }
}