use crate::prelude::*;
use crate::data::Cycle;
use crate::utils::{ButtonList, PopoverExtension};

use super::date_range_picker::DateRangePicker;

#[derive(Clone)]
pub struct CycleSelector {
    popover: gtk::Popover,
    button_list: ButtonList<Cycle>,
    repeat_button: gtk::Button,
    action_row: adw::ActionRow,
    date_range_picker: DateRangePicker,
}

impl CycleSelector {

    pub fn new(date_range_picker: DateRangePicker) -> Self {
        date_range_picker.set_enable_end_date(false);

        let repeat_button = Self::build_repeat_button();

        let button_list = ButtonList::new(true);
            button_list.add_with_prefixed_icon(Cycle::Daily, Cycle::Daily.icon_name(), Cycle::Daily.as_str());
            button_list.add_with_prefixed_icon(Cycle::Weekly, Cycle::Weekly.icon_name(), Cycle::Weekly.as_str());
            button_list.add_with_prefixed_icon(Cycle::Monthly, Cycle::Monthly.icon_name(), Cycle::Monthly.as_str());
            button_list.add_with_prefixed_icon(Cycle::Yearly, Cycle::Yearly.icon_name(), Cycle::Yearly.as_str());

        let popover = gtk::Popover::new();
            popover.set_button_list(&button_list);
            popover.set_parent(&repeat_button);

        let action_row = Self::build_action_row(&repeat_button);
        let this = Self {
            popover,
            button_list,
            repeat_button,
            action_row,
            date_range_picker,
        };
        this.connect_action_row_activated();
        this.connect_button_list_activated();
        this.connect_repeat_button_clicked();
        this.connect_popover_popdown();
        this
    }

    fn build_repeat_button() -> gtk::Button {
        let icon = gtk::Image::from_icon_name("media-playlist-repeat-symbolic");
            icon.set_halign(gtk::Align::Start);

        let label = gtk::Label::new(Some("Repeat"));
            label.set_hexpand(true);
            label.set_halign(gtk::Align::Center);

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
            h_box.append(&icon);
            h_box.append(&label);
            h_box.set_hexpand(false);
        
        let button = gtk::Button::new();
            button.set_valign(gtk::Align::Center);
            button.set_focusable(false);
            button.set_child(Some(&h_box));
            button.set_tooltip_text(Some("Repeat"));
            button
    }

    fn build_action_row(repeat_button: &gtk::Button) -> adw::ActionRow {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        container.append(repeat_button);

        let action_row = adw::ActionRow::new();
        action_row.set_title("Cycle");
        action_row.add_suffix(&container);
        action_row.set_activatable(true);
        action_row
    }

    fn connect_button_list_activated(&self) {
        let repeat_button_weak = self.repeat_button.downgrade();
        let date_range_picker_clone = self.date_range_picker.clone();
        self.button_list.connect_activated(move |_, _, is_active| {
            let Some(repeat_button) = repeat_button_weak.upgrade() else { return };
            if is_active {
                repeat_button.add_css_class("active");
                date_range_picker_clone.set_enable_end_date(true);
            } else {
                repeat_button.remove_css_class("active");
                date_range_picker_clone.set_enable_end_date(false);
            }
        });
    }

    fn connect_popover_popdown(&self) {
        let button_list = self.button_list.clone();
        let repeat_button_weak = self.repeat_button.downgrade();
        self.popover.connect_closed(move |_| {
            let Some(repeat_button) = repeat_button_weak.upgrade() else { return };
            if button_list.active_key().unwrap_or(Cycle::OneTime) == Cycle::OneTime {
                repeat_button.remove_css_class("active");
            }
        });
    }

    fn connect_repeat_button_clicked(&self) {
        let popover = self.popover.downgrade();
        self.repeat_button.connect_clicked(move |repeat_button| {
            if let Some(popover) = popover.upgrade() {
                if !popover.is_visible() {
                    popover.popup();
                    repeat_button.add_css_class("active");
                }
            }
        });
    }

    fn connect_action_row_activated(&self) {
        let weak_repeat_button = self.repeat_button.downgrade();
        self.action_row.connect_activated(move |_| {
            let Some(repeat_button) = weak_repeat_button.upgrade() else { return };
            repeat_button.emit_clicked();
        });
    }

    pub fn get_selected_cycle(&self) -> Cycle {
        self
            .button_list
            .active_key()
            .unwrap_or(Cycle::OneTime)
    }

    pub fn set_selected_cycle(&self, cycle: Cycle) {
        if cycle == Cycle::OneTime {
            self.button_list.deactivate_all_buttons();
            self.repeat_button.remove_css_class("active");
            self.date_range_picker.set_enable_end_date(false);
        } else {
            self.button_list.activate_button(&cycle);
            self.repeat_button.add_css_class("active");
            self.date_range_picker.set_enable_end_date(true);
        }
    }

    pub fn connect_selected(&self, callback: impl Fn(Cycle) + 'static) {
        self.button_list.connect_activated(move |cycle, _, _| {
            callback(cycle);
        });
    }

}

impl HasWidget<gtk::Widget> for CycleSelector {
    fn widget(&self) -> &gtk::Widget {
        self.action_row.upcast_ref()
    }
}