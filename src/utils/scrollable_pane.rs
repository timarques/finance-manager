use std::collections::HashMap;

use crate::prelude::*;

#[derive(Clone)]
pub struct ScrollablePane {
    vertical_box: gtk::Box,
    scrolled_window: gtk::ScrolledWindow,
    groups_map: HashMap<gtk::Widget, adw::PreferencesGroup>
}

impl ScrollablePane {

    pub fn new() -> Self {
        let vertical_box = Self::build_box();

        let clamp = Self::build_clamp(&vertical_box);
        let scrolled_window = Self::build_scrolled_window(&clamp);

        Self {
            vertical_box,
            scrolled_window,
            groups_map: HashMap::new(),
        }
    }

    fn build_box() -> gtk::Box {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 20);
        v_box.set_hexpand(true);
        v_box
    }

    fn build_group(widgets: &Vec<&impl IsA<gtk::Widget>>) -> adw::PreferencesGroup {
        let group = adw::PreferencesGroup::new();
        for widget in widgets {
            group.add(*widget);
        }
        group
    }

    fn build_separator() -> gtk::Separator {
        let separator = gtk::Separator::new(gtk::Orientation::Vertical);
        separator.set_hexpand(true);
        separator
    }

    fn build_title_label(title: &str) -> gtk::Label {
        let title_label = gtk::Label::new(Some(title));
        title_label.add_css_class("title-3");
        title_label.set_halign(gtk::Align::Start);
        title_label
    }

    fn build_clamp(child: &impl IsA<gtk::Widget>) -> adw::Clamp {
        let clamp = adw::Clamp::new();
        clamp.set_maximum_size(800);
        clamp.set_child(Some(child));
        clamp.set_margin_bottom(24);
        clamp.set_margin_top(24);
        clamp.set_margin_start(24);
        clamp.set_margin_end(24);
        clamp
    }

    fn build_scrolled_window(child: &impl IsA<gtk::Widget>) -> gtk::ScrolledWindow {
        let scrolled_window = gtk::ScrolledWindow::new();
        scrolled_window.set_hexpand(true);
        scrolled_window.set_vexpand(true);
        scrolled_window.set_child(Some(child));
        scrolled_window
    }

    pub fn add_group(&mut self, widgets: Vec<&impl IsA<gtk::Widget>>) {
        let group = Self::build_group(&widgets);
        for widget in widgets {
            self.groups_map.insert(widget.clone().upcast(), group.clone());
        }
        self.vertical_box.append(&group);
    }

    pub fn add_header(&self, title: &str) {
        self.vertical_box.append(&Self::build_title_label(title));
    }

    pub fn add_separator(&self) {
        self.vertical_box.append(&Self::build_separator());
    }

    pub fn get_children(&self) -> Vec<gtk::Widget> {
        let mut children: Vec<gtk::Widget> = Vec::new();
        for child in self.vertical_box.observe_children().snapshot().iter() {
            if let Some(child) = child.downcast_ref().cloned() {
                children.push(child);
            }
        }
        children
    }

}

impl HasWidget<gtk::Widget> for ScrollablePane {
    fn widget(&self) -> &gtk::Widget {
        self.scrolled_window.upcast_ref()
    }
}