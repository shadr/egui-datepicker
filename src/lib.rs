//! egui-datepicker adds a simple date picker widget.
//! Checkout the [example][ex]
//!
//!
//! ```no_run
//! use egui_datepicker::DatePicker;
//!
//! let mut date: chrono::Date<T>;
//! ui.add(DatePicker::new("super_unique_id", &mut date);
//! ```
//!
//! [ex]: ./examples/simple.rs

use std::{fmt::Display, hash::Hash};

use chrono::{prelude::*, Duration};
use eframe::{
    egui,
    egui::{Area, DragValue, Frame, Id, Key, Order, Response, Ui, Widget},
};
use num_traits::FromPrimitive;

/// Default values of fields are:
/// - sunday_first: `false`
/// - movable: `false`
/// - format_string: `"%Y-%m-%d"`
pub struct DatePicker<'a, Tz>
where
    Tz: TimeZone,
    Tz::Offset: Display,
{
    id: Id,
    date: &'a mut Date<Tz>,
    sunday_first: bool,
    movable: bool,
    format_string: String,
}

impl<'a, Tz> DatePicker<'a, Tz>
where
    Tz: TimeZone,
    Tz::Offset: Display,
{
    /// Create new date picker with unique id and mutable reference to date.
    pub fn new<T: Hash>(id: T, date: &'a mut Date<Tz>) -> Self {
        Self {
            id: Id::new(id),
            date,
            sunday_first: false,
            movable: false,
            format_string: String::from("%Y-%m-%d"),
        }
    }

    /// If flag is set to true then first day in calendar will be sunday otherwise monday.
    /// Default is false
    #[must_use]
    pub fn sunday_first(mut self, flag: bool) -> Self {
        self.sunday_first = flag;
        self
    }

    /// If flag is set to true then date picker popup will be movable.
    /// Default is false
    #[must_use]
    pub fn movable(mut self, flag: bool) -> Self {
        self.movable = flag;
        self
    }

    ///Set date format.
    ///See the [chrono::format::strftime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) for the specification.
    #[must_use]
    pub fn date_format(mut self, new_format: impl ToString) -> Self {
        self.format_string = new_format.to_string();
        self
    }

    /// Draw names of week days as 7 columns of grid without calling Ui::end_row
    fn show_grid_header(&mut self, ui: &mut Ui) {
        let day_indexes = if self.sunday_first {
            [6, 0, 1, 2, 3, 4, 5]
        } else {
            [0, 1, 2, 3, 4, 5, 6]
        };
        for i in day_indexes {
            let b = Weekday::from_u8(i).unwrap();
            ui.label(b.to_string());
        }
    }

    fn show_calendar_grid(&mut self, ui: &mut Ui) {
        egui::Grid::new("calendar").show(ui, |ui| {
            self.show_grid_header(ui);
            let first_day_of_month = self.date.with_day(1).unwrap();
            let start_offset = if self.sunday_first {
                first_day_of_month.weekday().num_days_from_sunday()
            } else {
                first_day_of_month.weekday().num_days_from_monday()
            };
            let days_in_month = get_days_from_month(self.date.year(), self.date.month());
            let first_day_of_next_month =
                first_day_of_month.clone() + Duration::days(days_in_month);
            let end_offset = if self.sunday_first {
                (7 - (first_day_of_next_month).weekday().num_days_from_sunday()) % 7
            } else {
                (7 - (first_day_of_next_month).weekday().num_days_from_monday()) % 7
            };
            let start_date = first_day_of_month - Duration::days(start_offset.into());
            for i in 0..(start_offset as i64 + days_in_month + end_offset as i64) {
                if i % 7 == 0 {
                    ui.end_row();
                }
                let d = start_date.clone() + Duration::days(i);
                ui.centered_and_justified(|ui| {
                    let mut day_button = egui::Button::new(d.day().to_string());
                    if self.date == &d {
                        day_button = day_button.enabled(false);
                    }
                    if self.date.month() != d.month() {
                        ui.style_mut().visuals.button_frame = false;
                    }
                    if ui.add(day_button).clicked() {
                        *self.date = d;
                    }
                });
            }
        });
    }

    /// Draw current month, buttons for next and previous month.
    fn show_header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.show_month_control(ui);
            self.show_year_control(ui);
            if ui.button("Today").clicked() {
                *self.date = Utc::now().with_timezone(&self.date.timezone()).date();
            }
        });
    }

    /// Draw button with text and add duration to current date.
    fn date_step_button(&mut self, ui: &mut Ui, text: impl ToString, duration: Duration) {
        if ui.button(text).clicked() {
            *self.date = self.date.clone() + duration;
        }
    }

    /// Draw drag value widget with current year and two buttons which substract and add 365 days
    /// to current date.
    fn show_year_control(&mut self, ui: &mut Ui) {
        self.date_step_button(ui, "<", Duration::days(-365));
        let mut drag_year = self.date.year();
        ui.add(DragValue::new(&mut drag_year));
        if drag_year != self.date.year() {
            *self.date = self.date.with_year(drag_year).unwrap();
        }
        self.date_step_button(ui, ">", Duration::days(365));
    }

    /// Draw combobox with current month and two buttons which substract and add 30 days
    /// to current date.
    fn show_month_control(&mut self, ui: &mut Ui) {
        self.date_step_button(ui, "<", Duration::days(-30));
        let mut selected = self.date.month0() as usize;
        egui::ComboBox::from_id_source(self.id.with("month_combo_box"))
            .selected_text(self.date.month0() as usize)
            .show_index(ui, &mut selected, 12, |i| {
                chrono::Month::from_usize(i + 1).unwrap().name().to_string()
            });
        if selected != self.date.month0() as usize {
            *self.date = self.date.with_month0(selected as u32).unwrap();
        }
        self.date_step_button(ui, ">", Duration::days(30));
    }
}

impl<'a, Tz> Widget for DatePicker<'a, Tz>
where
    Tz: TimeZone,
    Tz::Offset: Display,
{
    fn ui(mut self, ui: &mut Ui) -> Response {
        let formated_date = self.date.format(&self.format_string);
        let button_response = ui.button(formated_date);
        if button_response.clicked() {
            ui.memory().toggle_popup(self.id);
        }

        if ui.memory().is_popup_open(self.id) {
            let mut area = Area::new(self.id)
                .order(Order::Foreground)
                .default_pos(button_response.rect.left_bottom());
            if !self.movable {
                area = area.movable(false);
            }
            let area_response = area
                .show(ui.ctx(), |ui| {
                    Frame::popup(ui.style()).show(ui, |ui| {
                        self.show_header(ui);
                        self.show_calendar_grid(ui);
                    });
                })
                .response;

            if !button_response.clicked()
                && (ui.input().key_pressed(Key::Escape) || area_response.clicked_elsewhere())
            {
                ui.memory().toggle_popup(self.id);
            }
        }
        button_response
    }
}

// https://stackoverflow.com/a/58188385
fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}
