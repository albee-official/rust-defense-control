#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)]

use std::ops::DerefMut;
use std::sync::Arc;

use crate::authorized::api::Api;
use crate::authorized::serial_connection::PollResult;
use crate::data::AppState;
use battery::State;
use chrono::{Duration, Utc};
use eframe::egui::{Color32, FontData, FontDefinitions, FontFamily};
use eframe::epaint::CornerRadius;
use eframe::NativeOptions;
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

    let manager =
        battery::Manager::new().expect("Failed to create a battery manager!");

    let mut toasts = Toasts::default();
    let mut user_data = AppState {
        api: Api::new(),
        last_poll_result: PollResult {
            open_door_front: false,
            motion_detected_1: false,
            motion_detected_2: false,
            accelerometer: false,
            fire_detected: false,
            door_invade: false,
            open_door_back: false,
        },
        battery_manager: manager,
        current_session: None,
        input_password: "".to_owned(),
        input_username: "".to_owned(),
        alarm_end_time: None,
    };

    let mut options = NativeOptions::default();
    options.dithering = true;

    let mut previous_on_battery = false;
    let mut previous_poll_time = chrono::Utc::now();
    let mut send_alarm_time = None;

    run_simple_native(
        "Control",
        options,
        move |ctx, _frame| {
            ctx.set_fonts(fonts.clone());
            customize_ui(ctx);

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.ctx().request_repaint();

                let mut alarmed = false;
                if let Some(end_time) = user_data.alarm_end_time {
                    if Utc::now() > end_time {
                        ui.colored_label(Color32::RED, "ALARM TRIGGERED!");
                        ui.colored_label(Color32::RED, "Timer expired!");
                    } else {
                        ui.colored_label(Color32::RED, "ALARM TRIGGERED!");
                        ui.label("Remaining time:");
                        let remaining_time = end_time - Utc::now();
                        let seconds = remaining_time.num_seconds();
                        let minutes = seconds / 60;
                        let remaining_seconds = seconds % 60;
                        let remaining_time_str = format!(
                            "{:02}:{:02}",
                            minutes, remaining_seconds
                        );

                        ui.colored_label(Color32::RED, remaining_time_str);
                    }

                    alarmed = true;
                }

                ui.add(user_data.api.widget());

                // poll data every 100 ms
                if Utc::now() - previous_poll_time
                    > Duration::milliseconds(250)
                {
                    if user_data.api.exists() {
                        previous_poll_time = Utc::now();

                        match user_data.api.send_poll() {
                            Ok(res) => {
                                if res.open_door_back
                                    || res.open_door_front
                                    || res.motion_detected_1
                                    || res.motion_detected_2
                                    || res.accelerometer
                                    || res.fire_detected
                                    || res.door_invade
                                    && !alarmed {
                                    user_data.alarm_end_time = Some(Utc::now() + Duration::minutes(5));
                                }

                                user_data.last_poll_result = res;
                            }

                            Err(err) => {
                                toasts.error(format!("{}", err));
                                eprintln!("{:?}", err);
                                user_data.api.close_connection();
                            }
                        }
                    }
                }

                if let Some(time) = send_alarm_time
                    && time < Utc::now() && !alarmed
                {
                    toasts.error("The device has been on battery for 3 seconds. Activating alarm!");
                    user_data.alarm_end_time = Some(Utc::now() + Duration::minutes(5));
                    send_alarm_time = None;
                }

                if user_data
                    .battery_manager
                    .batteries()
                    .iter_mut()
                    .flatten()
                    .any(|b| {
                        if let Ok(b) = b {
                            let state = b.state();
                            state == State::Discharging
                                || state == State::Empty
                                || state == State::Unknown
                        } else {
                            false
                        }
                    })
                {
                    ui.colored_label(Color32::RED, "THE DEVICE IS ON BATTERY");

                    if !previous_on_battery {
                        toasts.warning("The device is running on battery.");
                        send_alarm_time =
                            Some(chrono::Utc::now() + Duration::seconds(3));
                    }

                    previous_on_battery = true;
                } else {
                    if previous_on_battery {
                        toasts.info("The device is running on charger.");
                    }

                    previous_on_battery = false;
                    send_alarm_time = None;
                }

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

    style.visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);

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
