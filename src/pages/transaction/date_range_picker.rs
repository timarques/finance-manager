use chrono::{Datelike, NaiveDate};

use crate::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct DatePicker {
    label: gtk::Label,
    button: gtk::Button,
    calendar: gtk::Calendar,
    popover: gtk::Popover,
    container: gtk::Box,
}

impl DatePicker {
    fn new() -> Self {
        let (button, label) = Self::build_calendar_button();
        let calendar = Self::build_calendar();
        let popover = Self::build_popover(&calendar);
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        let date_picker = Self {
            label,
            button,
            calendar,
            popover,
            container,
        };

        date_picker.connect_signals();
        date_picker.update_date();
        date_picker.setup_container();
        date_picker
    }

    fn build_calendar_button() -> (gtk::Button, gtk::Label) {
        let label = gtk::Label::new(Some("Date"));
        label.set_halign(gtk::Align::Center);
        let icon = gtk::Image::from_icon_name("month-symbolic");
        icon.set_halign(gtk::Align::Start);

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        h_box.set_valign(gtk::Align::Center);
        h_box.append(&icon);
        h_box.append(&label);

        let button = gtk::Button::new();
        button.set_child(Some(&h_box));
        button.set_valign(gtk::Align::Center);
        button.set_focusable(false);
        (button, label)
    }

    fn build_calendar() -> gtk::Calendar {
        let calendar = gtk::Calendar::new();
        calendar.select_day(&gtk::glib::DateTime::now_local().unwrap());
        calendar
    }

    fn build_popover(calendar: &gtk::Calendar) -> gtk::Popover {
        let popover = gtk::Popover::new();
        popover.set_child(Some(calendar));
        popover.set_has_arrow(true);
        popover.set_autohide(true);
        popover
    }

    fn connect_button_signals(&self) {
        let popover_weak = self.popover.downgrade();
        self.button.connect_clicked(move |button| {
            let Some(popover) = popover_weak.upgrade() else { return };
            if !popover.is_visible() {
                popover.popup();
                button.add_css_class("active");
            }
        });
    }

    fn connect_popover_signals(&self) {
        let button_weak = self.button.downgrade();
        let calendar_weak = self.calendar.downgrade();
        self.popover.connect_closed(move |_| {
            let Some(button) = button_weak.upgrade() else { return };
            let Some(calendar) = calendar_weak.upgrade() else { return };
            
            let now = gtk::glib::DateTime::now_local().expect("Failed to get local time");
            let calendar_date = calendar.date();
            
            if calendar_date.year() == now.year() && 
               calendar_date.month() == now.month() && 
               calendar_date.day_of_month() == now.day_of_month() {
                button.remove_css_class("active");
            }
        });
    }

    fn connect_calendar_signals(&self) {
        let label_weak = self.label.downgrade();
        let popover_weak = self.popover.downgrade();
        self.calendar.connect_day_selected(move |calendar| {
            let Some(label) = label_weak.upgrade() else { return };
            let Some(popover) = popover_weak.upgrade() else { return };
            Self::update_label_date(calendar, &label);
            gtk::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                popover.popdown();
                gtk::glib::ControlFlow::Break
            });
        });
    }

    fn connect_signals(&self) {
        self.connect_button_signals();
        self.connect_popover_signals();
        self.connect_calendar_signals();
    }

    fn update_label_date(calendar: &gtk::Calendar, label: &gtk::Label) {
        let date = calendar.date();
        label.set_text(&date.format("%Y-%m-%d").unwrap());
    }

    fn update_date(&self) {
        let now = gtk::glib::DateTime::now_local().expect("Failed to get local time");
        let date= self.calendar.date();
        if date.year() != now.year() ||
            date.month() != now.month() ||
            date.day_of_month() != now.day_of_month() {
            self.button.add_css_class("active");
        }
        Self::update_label_date(&self.calendar, &self.label);
    }

    fn setup_container(&self) {
        self.container.set_valign(gtk::Align::Center);
        self.container.append(&self.button);
        self.container.append(&self.popover);
    }

    fn get_date_from_calendar(calendar: &gtk::Calendar) -> NaiveDate {
        let date = calendar.date();
        NaiveDate::from_ymd_opt(date.year(), date.month() as u32, date.day_of_month() as u32).unwrap()
    }

    pub fn get_date(&self) -> NaiveDate {
        Self::get_date_from_calendar(&self.calendar)
    }

    pub fn set_date(&self, date: NaiveDate) {
        let glib_date = gtk::glib::DateTime::new(
            &gtk::glib::TimeZone::local(),
            date.year() as i32,
            date.month() as i32,
            date.day() as i32,
            0,
            0,
            0.0
        ).expect("Failed to create glib DateTime");
        self.calendar.select_day(&glib_date);
        self.update_date();
    }

    pub fn set_default_date(&self) {
        let now = gtk::glib::DateTime::now_local().expect("Failed to get local time");
        self.button.remove_css_class("active");
        self.calendar.select_day(&now);
    }

}

#[derive(Clone)]
pub struct DateRangePicker {
    start_date_picker: DatePicker,
    end_date_picker: DatePicker,
    action_row: adw::ActionRow,
    callback: Rc<RefCell<Option<Rc<dyn Fn(NaiveDate, NaiveDate) + 'static>>>>
}

impl DateRangePicker {

    pub fn new() -> Self {
        let start_date_picker = DatePicker::new();
        let end_date_picker = DatePicker::new();
        let this = Self {
            start_date_picker,
            end_date_picker,
            action_row: adw::ActionRow::new(),
            callback: Rc::new(RefCell::new(None))
        };

        this.setup_action_row();
        this.syncronize_dates();
        this
    }

    fn setup_action_row(&self) {
        let start_date_button_weak = self.start_date_picker.button.downgrade();
        let end_date_button_weak = self.end_date_picker.button.downgrade();
        self.action_row.set_title("Dates");
        self.action_row.add_suffix(&self.start_date_picker.container);
        self.action_row.add_suffix(&gtk::Label::new(Some(" - ")));
        self.action_row.add_suffix(&self.end_date_picker.container);
        self.action_row.set_activatable(true);
        self.action_row.connect_activated(move |_| {
            let Some(start_button) = start_date_button_weak.upgrade() else { return };
            let Some(end_button) = end_date_button_weak.upgrade() else { return };

            if !end_button.is_sensitive() {
                start_button.emit_clicked();
            }
        });
    }

    fn syncronize_dates(&self) {
        let callback = self.callback.clone();
        let end_date_picker = self.end_date_picker.clone();
        self.start_date_picker.calendar.connect_day_selected(move |calendar| {
            if calendar.date() > end_date_picker.calendar.date() {
                end_date_picker.button.remove_css_class("active");
                DatePicker::update_label_date(calendar, &end_date_picker.label);
            } else {
                end_date_picker.update_date();
            }

            let callback = callback.borrow().clone();
            if let Some(callback) = callback {
                callback(DatePicker::get_date_from_calendar(calendar), end_date_picker.get_date());
            }
        });

        let callback = self.callback.clone();
        let start_date_picker = self.start_date_picker.clone();
        self.end_date_picker.calendar.connect_day_selected(move |calendar| {
            if start_date_picker.calendar.date() > calendar.date() {
                start_date_picker.button.remove_css_class("active");
                DatePicker::update_label_date(calendar, &start_date_picker.label);
            } else {
                start_date_picker.update_date();
            }

            let callback = callback.borrow().clone();
            if let Some(callback) = callback {
                callback(start_date_picker.get_date(), DatePicker::get_date_from_calendar(calendar));
            }
        });
    }

    pub fn connect_changed(&self, callback: impl Fn(NaiveDate, NaiveDate) + 'static) {
        self.callback.borrow_mut().replace(Rc::new(callback));
    }

    pub fn set_enable_end_date(&self, enable: bool) {
        self.end_date_picker.button.set_sensitive(enable);
    }

    pub fn get_start_date(&self) -> NaiveDate {
        self.start_date_picker.get_date()
    }

    pub fn get_end_date(&self) -> NaiveDate {
        self.end_date_picker.get_date()
    }

    pub fn set_start_date(&self, date: NaiveDate) {
        self.start_date_picker.set_date(date);
        if date > self.end_date_picker.get_date() {
            DatePicker::update_label_date(&self.start_date_picker.calendar, &self.end_date_picker.label);
            self.end_date_picker.button.remove_css_class("active");
        }
    }

    pub fn set_end_date(&self, date: NaiveDate) {
        if date < self.start_date_picker.get_date() {
            DatePicker::update_label_date(&self.end_date_picker.calendar, &self.start_date_picker.label);
            self.start_date_picker.button.remove_css_class("active");
        }
        self.end_date_picker.set_date(date);
    }

    pub fn set_default_dates(&self) {
        self.start_date_picker.set_default_date();
        self.end_date_picker.set_default_date();
    }

}

impl HasWidget<gtk::Widget> for DateRangePicker {
    fn widget(&self) -> &gtk::Widget {
        &self.action_row.upcast_ref()
    }
}