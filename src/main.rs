use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use crate::authorized::api::Api;
use crate::data::{AppState, ControlledValue, SensorData};
use eframe::NativeOptions;
use eframe::egui::{FontData, FontDefinitions, FontFamily};
use eframe::epaint::CornerRadius;
use eframe::{egui, run_simple_native};
use egui_notify::Toasts;

mod app;
mod auth;
mod authorized;
mod data;
mod fluent;
mod widgets;

fn main() -> eframe::Result {
    let fonts = configure_fonts();

    let enable_editing = Rc::new(Cell::new(false));
    let mut toasts = Toasts::default();
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
            customize_ui(ctx);

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(user_data.api.widget());

                if user_data.api.exists() {
                    ui.separator();
                    app::render(&mut user_data, &mut toasts, ui);
                }
            });

            toasts.show(ctx);
        },
    )
}

fn customize_ui(ctx: &egui::Context) {
    // Get a mutable reference to the default style
    let mut style = (*ctx.style()).clone();

    style
        .visuals
        .widgets
        .noninteractive
        .corner_radius = CornerRadius::ZERO;

    style
        .visuals
        .widgets
        .inactive
        .corner_radius = CornerRadius::ZERO;
    style
        .visuals
        .widgets
        .hovered
        .corner_radius = CornerRadius::ZERO;
    style
        .visuals
        .widgets
        .active
        .corner_radius = CornerRadius::ZERO;
    style.visuals.widgets.open.corner_radius = CornerRadius::ZERO;

    style.visuals.window_corner_radius = CornerRadius::ZERO;
    style.visuals.menu_corner_radius = CornerRadius::ZERO;

    // Apply the modified style
    ctx.set_style(style);
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
