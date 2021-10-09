use chrono::prelude::*;
use eframe::{egui, epi};
use egui_datepicker::DatePicker;

struct ExampleApp {
    date: chrono::Date<Utc>,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            date: Utc::now().date(),
        }
    }
}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "Datepicker example"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        // ctx.set_debug_on_hover(true);
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("exaamples_grid").show(ui, |ui| {
                ui.label("Default");
                ui.add(DatePicker::new("default", &mut self.date));
                ui.end_row();
                ui.label("Sunday first");
                ui.add(DatePicker::new("sundayfirst", &mut self.date).sunday_first(true));
                ui.end_row();
                ui.label("Movable popup");
                ui.add(DatePicker::new("movable", &mut self.date).movable(true));
                ui.end_row();
                ui.label("Different format");
                ui.add(DatePicker::new("differentformat", &mut self.date).date_format(&"%d/%m/%Y"));
                ui.end_row();
            });
        });
    }
}

fn main() {
    let app = ExampleApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
