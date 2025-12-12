use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use crate::authorized::api::Api;
use crate::data::{AppState, ControlledValue, SensorData};
use eframe::egui::{FontData, FontDefinitions, FontFamily};
use eframe::NativeOptions;
use eframe::{egui, run_simple_native};

mod app;
mod auth;
mod authorized;
mod data;
mod fluent;
mod widgets;

fn main() -> eframe::Result {
    let fonts = configure_fonts();

    let enable_editing = Rc::new(Cell::new(false));
    let mut user_data = AppState {
        enable_editing: enable_editing.clone(),
        api: Api::new(),
        current_session: None,
        input_password: "".to_owned(),
        input_username: "".to_owned(),
        sensor_data: SensorData {
            is_alarm_enabled: ControlledValue::from(&enable_editing),
            is_on_battery: true,
        },
    };

    let mut options = NativeOptions::default();
    options.dithering = true;

    run_simple_native(
        "Control",
        options,
        move |ctx, _frame| {
            ctx.set_fonts(fonts.clone());

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(user_data.api.widget());

                if user_data.api.exists() {
                    ui.separator();

                    ctx.request_repaint();
                    app::render(&mut user_data, ui);
                }
            });
        },
    )
}

fn configure_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "JetBrains Mono".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/JetBrainsMono-Regular.ttf"
        ))),
    );

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "JetBrains Mono".to_owned());

    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .push("JetBrains Mono".to_owned());

    fonts
}
