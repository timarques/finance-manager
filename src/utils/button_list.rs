use crate::prelude::*;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

const ACTIVE_CLASS: &str = "active";

#[derive(Clone)]
pub struct ButtonList<K: Clone + Eq + std::hash::Hash + 'static> {
    allow_empty: bool,
    map: Rc<RefCell<HashMap<K, gtk::Button>>>,
    container: gtk::Box,
    callbacks: Rc<RefCell<Vec<Rc<dyn Fn(K, &gtk::Button, bool)>>>>
}

impl<K: Clone + Eq + std::hash::Hash + 'static> ButtonList<K> {
    pub fn new(allow_empty: bool) -> Self {
        let v_box = Self::build_box();
        Self {
            allow_empty,
            container: v_box,
            map: Rc::new(RefCell::new(HashMap::new())),
            callbacks: Rc::new(RefCell::new(Vec::new()))
        }
    }

    fn build_box() -> gtk::Box {
        let h_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        h_box.set_valign(gtk::Align::Center);
        h_box.set_margin_top(5);
        h_box.set_margin_bottom(5);
        h_box.set_margin_start(5);
        h_box.set_margin_end(5);
        h_box
    }

    fn build_button(child: &impl IsA<gtk::Widget>) -> gtk::Button {
        let button = gtk::Button::new();
        button.set_child(Some(child));
        button.set_valign(gtk::Align::Center);
        button.set_halign(gtk::Align::Fill);
        button.set_hexpand(true);
        button
    }

    fn build_text_label(text: &str) -> gtk::Label {
        let label = gtk::Label::new(Some(text));
        label.set_halign(gtk::Align::Center);
        label.set_valign(gtk::Align::Center);
        label.set_hexpand(true);
        label
    }

    fn build_button_box(prefix: & impl IsA<gtk::Widget>, text: &str) -> gtk::Box {
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
            h_box.set_valign(gtk::Align::Center);
            h_box.set_halign(gtk::Align::Fill);
            h_box.set_hexpand(true);
            h_box.set_baseline_position(gtk::BaselinePosition::Center);
            h_box.set_tooltip_text(Some(text));
            h_box.append(prefix);
            h_box.append(&Self::build_text_label(text));
            h_box
    }

    fn remove_active_classes(map: &HashMap<K, gtk::Button>) {
        map.values().for_each(|button| button.remove_css_class(ACTIVE_CLASS));
    }

    pub fn active_key(&self) -> Option<K> {
        self.map.borrow().iter()
            .find_map(|(index, button)| {
                if button.has_css_class(ACTIVE_CLASS) {
                    Some(index.clone())
                } else {
                    None
                }
            })
    }

    #[allow(dead_code)]
    pub fn active_button(&self) -> Option<gtk::Button> {
        self.map.borrow().iter()
            .find_map(|(_, button)| {
                if button.has_css_class(ACTIVE_CLASS) {
                    Some(button.clone())
                } else {
                    None
                }
            })
    }
    
    pub fn activate_button(&self, index: &K) {
        let map_ref = self.map.borrow();
        for (button_index, button) in map_ref.iter() {
            if button_index == index {
                button.add_css_class(ACTIVE_CLASS);
            } else {
                button.remove_css_class(ACTIVE_CLASS);
            }
        }
    }
    
    pub fn deactivate_all_buttons(&self) {
        Self::remove_active_classes(&*self.map.borrow());
    }

    fn handle_button_click(
        index: &K,
        button: &gtk::Button,
        map: &Rc<RefCell<HashMap<K, gtk::Button>>>,
        allow_empty: bool,
        callbacks: &Rc<RefCell<Vec<Rc<dyn Fn(K, &gtk::Button, bool)>>>>
    ) {
        let is_active = button.has_css_class(ACTIVE_CLASS) && allow_empty;
        if is_active {
            button.remove_css_class(ACTIVE_CLASS);
        } else {
            Self::remove_active_classes(&*map.borrow());
            button.add_css_class(ACTIVE_CLASS);
        }

        for callback in callbacks.borrow().iter() {
            callback(index.clone(), button, !is_active);
        }
    }

    pub fn add_button(&self, index: K, button: gtk::Button) -> K {
        let map = self.map.clone();
        let callbacks = self.callbacks.clone();
        let allow_empty = self.allow_empty;

        let index_clone = index.clone();
        button.connect_clicked(move |b| Self::handle_button_click(
            &index_clone, 
            b,
            &map,
            allow_empty,
            &callbacks
        ));

        self.map.borrow_mut().insert(index.clone(), button);
        self.container.append(self.map.borrow().get(&index).unwrap());
        
        index
    }

    pub fn add_with_child(&self, index: K, child: &impl IsA<gtk::Widget>) -> K {
        let button = Self::build_button(child);
        self.add_button(index, button)
    }

    pub fn add_with_text(&self, index: K, text: &str) -> K {
        let label = Self::build_text_label(text);
        self.add_with_child(index, &label)
    }

    pub fn add_with_prefixed_icon(&self, index: K, icon_name: &str, text: &str) -> K {
        let icon = gtk::Image::from_icon_name(icon_name);
        icon.set_halign(gtk::Align::Start);
        icon.set_valign(gtk::Align::Center);
        
        let h_box = Self::build_button_box(&icon, text);
        
        self.add_with_child(index, &h_box)
    }

    pub fn add_with_prefixed_label(&self, index: K, prefix_text: &str, text: &str) -> K {        
        let prefix_label = gtk::Label::new(Some(prefix_text));
        prefix_label.set_halign(gtk::Align::Start);
        prefix_label.set_valign(gtk::Align::Center);
        
        let h_box = Self::build_button_box(&prefix_label, text);
        self.add_with_child(index, &h_box)
    }

    pub fn connect_activated<F: Fn(K, &gtk::Button, bool) + 'static>(&self, callback: F) -> usize {
        self.callbacks.borrow_mut().push(Rc::new(callback));
        self.callbacks.borrow().len() -  1
    }

    #[allow(dead_code)]
    pub fn disconnect_activated(&self, callback_index: usize) {
        self.callbacks.borrow_mut().remove(callback_index);
    }

}

impl<K: Clone + Eq + std::hash::Hash + 'static> HasWidget<gtk::Widget> for ButtonList<K> {
    fn widget(&self) -> &gtk::Widget {
        self.container.upcast_ref()
    }
}