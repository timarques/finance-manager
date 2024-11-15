use chrono::{Datelike, NaiveDate};
use gtk::glib::clone::Downgrade;

use crate::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

struct DatePicker {
    label: gtk::Label,
    button: gtk::Button,
    clear_button: gtk::Button,
    calendar: gtk::Calendar,
    popover: gtk::Popover,
    container: gtk::Box,

    callback: RefCell<Option<Box<dyn Fn(&Rc<Self>, NaiveDate) + 'static>>>,
}

impl DatePicker {
    fn new() -> Rc<Self> {
        let (button, label) = Self::build_calendar_button();
        let clear_button = Self::build_clear_button();
        let calendar = Self::build_calendar();

        let calendar_container = Self::build_calendar_container(&calendar, &clear_button);
        let popover = Self::build_popover(&calendar_container);
        let container= Self::build_container(&button, &popover);

        let this = Rc::new(Self {
            label,
            button,
            clear_button,
            calendar,
            popover,
            container,

            callback: RefCell::new(None),
        });

        this.connect_button_clicked();
        this.connect_calendar_day_selected();
        this.connect_clear_button_clicked();
        this.set_default_date();
        this
    }

    fn build_container(button: &gtk::Button, popover: &gtk::Popover) -> gtk::Box {
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        h_box.set_valign(gtk::Align::Center);
        h_box.append(button);
        h_box.append(popover);
        h_box
    }

    fn build_clear_button() -> gtk::Button {
        let icon = gtk::Image::from_icon_name("brush-symbolic");
        let label = gtk::Label::new(Some("Clear"));

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        h_box.set_valign(gtk::Align::Center);
        h_box.append(&icon);
        h_box.append(&label);
        h_box.set_hexpand(true);
        h_box.set_halign(gtk::Align::Center);

        let button = gtk::Button::new();
        button.set_child(Some(&h_box));
        button.set_valign(gtk::Align::Center);
        button.set_halign(gtk::Align::Fill);
        button.set_hexpand(true);
        button.set_focusable(false);
        button
    }

    fn build_calendar_button() -> (gtk::Button, gtk::Label) {
        let icon = gtk::Image::from_icon_name("month-symbolic");
        icon.set_halign(gtk::Align::Start);
        let label = gtk::Label::new(Some("0000-00-00"));
        label.set_halign(gtk::Align::Center);

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
        calendar.set_margin_bottom(5);
        calendar
    }

    fn build_calendar_container(calendar: &gtk::Calendar, clear_button: &gtk::Button) -> gtk::Box {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        v_box.append(calendar);
        v_box.append(clear_button);
        v_box.set_hexpand(false);
        v_box.set_margin_end(10);
        v_box.set_margin_start(10);
        v_box.set_margin_bottom(10);
        v_box.set_margin_top(10);
        v_box
    }

    fn build_popover(calendar: &impl IsA<gtk::Widget>) -> gtk::Popover {
        let popover = gtk::Popover::new();
        popover.set_child(Some(calendar));
        popover.set_has_arrow(true);
        popover.set_autohide(true);
        popover.add_css_class("card");
        popover
    }

    fn update_calendar(&self, date: NaiveDate) {

        if date == chrono::Utc::now().date_naive() {
            self.button.remove_css_class("active");
        } else {
            self.button.add_css_class("active");
        }

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
        self.calendar.set_month(glib_date.month() as i32 - 1);
        self.calendar.set_year(glib_date.year() as i32);
        self.label.set_text(&glib_date.format("%Y-%m-%d").unwrap());
    }

    pub fn get_date(&self) -> NaiveDate {
        let glib_date = self.calendar.date();
        NaiveDate::from_ymd_opt(glib_date.year(), glib_date.month() as u32, glib_date.day_of_month() as u32).unwrap()
    }

    pub fn set_date(&self, date: NaiveDate) {
        self.update_calendar(date);
    }

    pub fn set_default_date(&self) {
        let now = chrono::Utc::now().date_naive();
        self.update_calendar(now);
    }

    pub fn set_sensitive(&self, is_sensitive: bool) {
        self.button.set_sensitive(is_sensitive);
    }

    pub fn is_sensitive(&self) -> bool {
        self.button.is_sensitive()
    }

    pub fn activate(self: &Rc<Self>) {
        self.button.emit_clicked();
    }

    pub fn connect_changed<F: Fn(&Rc<Self>, NaiveDate) + 'static>(&self, callback: F) {
        self.callback.replace(Some(Box::new(callback)));
    }

    fn connect_button_clicked(self: &Rc<Self>) {
        let this = self.downgrade();
        self.button.connect_clicked(move |button| {
            let Some(this) = this.upgrade() else { return };
            if !this.popover.is_visible() {
                this.popover.popup();
                button.add_css_class("active");
            }
        });
    }

    fn connect_calendar_day_selected(self: &Rc<Self>) {
        let this = self.downgrade();
        self.calendar.connect_day_selected(move |_| {
            let Some(this) = this.upgrade() else { return };

            let date = this.get_date();
            if date == chrono::Utc::now().date_naive() {
                this.button.remove_css_class("active");
            }

            this.label.set_text(&date.format("%Y-%m-%d").to_string());

            let callback_ref = this.callback.borrow();
            if let Some(callback) = callback_ref.as_ref() {
                callback(&this, date);
            }

            let popover = this.popover.clone();
            gtk::glib::timeout_add_local_once(std::time::Duration::from_millis(100), move || {
                popover.popdown();
            });
        });
    }

    fn connect_clear_button_clicked(self: &Rc<Self>) {
        let this = self.downgrade();
        self.clear_button.connect_clicked(move |_| {
            let Some(this) = this.upgrade() else { return };
            this.set_default_date();
            this.popover.popdown();
        });
    }

}

impl HasWidget<gtk::Widget> for Rc<DatePicker> {
    fn widget(&self) -> &gtk::Widget {
        self.container.upcast_ref()
    }
}

#[derive(Clone)]
pub struct DateRangePicker {
    start_date_picker: Rc<DatePicker>,
    end_date_picker: Rc<DatePicker>,
    action_row: adw::ActionRow,
    callback: Rc<RefCell<Option<Rc<dyn Fn(NaiveDate, NaiveDate) + 'static>>>>,
}

impl DateRangePicker {

    pub fn new() -> Self {
        let start_date_picker = DatePicker::new();
        let end_date_picker = DatePicker::new();
        let action_row = Self::build_action_row(
            &start_date_picker, 
            &end_date_picker
        );
        let this = Self {
            start_date_picker,
            end_date_picker,
            action_row,
            callback: Rc::new(RefCell::new(None)),
        };

        
        this.set_default_dates();
        this.connect_action_row_activated();
        this.connect_start_date_changed();
        this.connect_end_date_changed();
        this
    }

    fn build_action_row(start_date_picker: &impl HasWidget<gtk::Widget>, end_date_picker: &impl HasWidget<gtk::Widget>) -> adw::ActionRow {
        let action_row = adw::ActionRow::new();
        action_row.set_title("Dates");
        action_row.set_activatable(true);
        action_row.add_suffix(start_date_picker.widget());
        action_row.add_suffix(&gtk::Label::new(Some(" - ")));
        action_row.add_suffix(end_date_picker.widget());
        action_row
    }

    fn connect_action_row_activated(&self) {
        let start_date_weak = self.start_date_picker.downgrade();
        let end_date_weak = self.end_date_picker.downgrade();
        self.action_row.connect_activated(move |_| {
            let Some(start_date_picker) = start_date_weak.upgrade() else { return };
            let Some(end_date_picker) = end_date_weak.upgrade() else { return };
            if !end_date_picker.is_sensitive() {
                start_date_picker.activate();
            }
        });
    }

    fn connect_start_date_changed(&self) {
        let callback = self.callback.clone();
        let end_date_picker_weak = self.end_date_picker.downgrade();
        self.start_date_picker.connect_changed(move |_, start_date| {
            let Some(end_date_picker) = end_date_picker_weak.upgrade() else { return };

            let end_date = end_date_picker.get_date();
            let end_date = if start_date >= end_date {
                end_date_picker.set_default_date();
                end_date_picker.get_date()
            } else {
                end_date
            };

            let callback = callback.borrow().clone();
            if let Some(callback) = callback {
                callback(
                    start_date,
                    end_date
                );
            }
        });
    }

    fn connect_end_date_changed(&self) {
        let callback = self.callback.clone();
        let start_date_picker_weak = self.start_date_picker.downgrade();
        self.end_date_picker.connect_changed(move |end_date_picker, end_date| {
            let Some(start_date_picker) = start_date_picker_weak.upgrade() else { return };

            let start_date = start_date_picker.get_date();

            let end_date = if end_date <= start_date {
                end_date_picker.set_default_date();
                end_date_picker.get_date()
            } else {
                end_date
            };

            let callback = callback.borrow().clone();
            if let Some(callback) = callback {
                callback(
                    start_date,
                    end_date
                );
            }
        });
    }

    pub fn connect_changed(&self, callback: impl Fn(NaiveDate, NaiveDate) + 'static) {
        self.callback.borrow_mut().replace(Rc::new(callback));
    }

    pub fn set_enable_end_date(&self, enable: bool) {
        self.end_date_picker.set_sensitive(enable);
    }

    pub fn get_start_date(&self) -> NaiveDate {
        self.start_date_picker.get_date()
    }

    pub fn get_end_date(&self) -> NaiveDate {
        self.end_date_picker.get_date()
    }

    pub fn set_start_date(&self, date: NaiveDate) {
        self.start_date_picker.set_date(date);
    }

    pub fn set_end_date(&self, date: NaiveDate) {
        if date < self.get_start_date() {
            self.end_date_picker.set_default_date();
        } else {
            self.end_date_picker.set_date(date);
        }
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