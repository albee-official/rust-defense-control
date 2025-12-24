use crate::{
    auth,
    data::{AppState, AuthLevel, SessionData},
    widgets::boolean_indicator::BooleanIndicator,
};
use chrono::{Duration, TimeDelta, Utc};
use eframe::egui::PopupCloseBehavior::CloseOnClickOutside;
use eframe::egui::{self};
use egui_notify::Toasts;

pub fn render(data: &mut AppState, toasts: &mut Toasts, ui: &mut egui::Ui) {
    ui.heading("Анти-(Анти-Автомат) System");

    egui::Popup::menu(&ui.button("Auth"))
        .close_behavior(CloseOnClickOutside)
        .width(220.0)
        .show(|ui| {
            ui.ctx().request_repaint();

            if let Some(session) = data.current_session.as_mut() {
                render_session(session, ui);

                if session.auth_level == AuthLevel::View {
                    data.current_session = None;

                    toasts.warning("Logged out");
                }
            } else {
                render_login(data, toasts, ui);
            }
        });

    render_data(data, toasts, ui);
}

fn render_data(data: &mut AppState, toasts: &mut Toasts, ui: &mut egui::Ui) {
    if data.current_session.is_some() {
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.label("General:");
            if ui.button("Poll").clicked() {
                let _ = data.api.send_poll().inspect_err(|err| {
                    toasts.error(format!("{:?}", err));
                });
            }

            if ui.button("Reset").clicked() {
                let _ = data
                    .api
                    .send_reset()
                    .inspect_err(|err| {
                        toasts.error(format!("{:?}", err));
                    });
            }

            if ui.button("Alarm!!!!!!").clicked() {
                data.alarm_end_time = Some(Utc::now() + Duration::minutes(5));
                data.api.send_lock_back_door();
                data.api.send_lock_front_door();
            }

            ui.horizontal(|ui| {
                ui.label("Back: ");

                if ui.button("Lock").clicked() {
                    let _ = data
                        .api
                        .send_lock_back_door()
                        .inspect_err(|err| {
                            toasts.error(format!("{:?}", err));
                        });
                }

                if ui.button("Unlock").clicked() {
                    let _ = data
                        .api
                        .send_unlock_back_door()
                        .inspect_err(|err| {
                            toasts.error(format!("{:?}", err));
                        });
                }
            });

            ui.horizontal(|ui| {
                ui.label("Front: ");

                if ui.button("Lock").clicked() {
                    let _ = data
                        .api
                        .send_lock_front_door()
                        .inspect_err(|err| {
                            toasts.error(format!("{:?}", err));
                        });
                }

                if ui.button("Unlock").clicked() {
                    let _ = data
                        .api
                        .send_unlock_front_door()
                        .inspect_err(|err| {
                            toasts.error(format!("{:?}", err));
                        });
                }
            });
        });
    }

    ui.group(|ui| {
        ui.label("Двери");

        ui.add(BooleanIndicator {
            label: "Передняя дверь:".to_owned(),
            value_ref: &data.last_poll_result.open_door_front,
        });

        ui.add(BooleanIndicator {
            label: "Задняя дверь:".to_owned(),
            value_ref: &data.last_poll_result.open_door_back,
        });
    });

    ui.group(|ui| {
        ui.label("Датчики движения");

        ui.add(BooleanIndicator {
            label: "Датчик движения 1:".to_owned(),
            value_ref: &data.last_poll_result.motion_detected_1,
        });

        ui.add(BooleanIndicator {
            label: "Датчик движения 2:".to_owned(),
            value_ref: &data.last_poll_result.motion_detected_2,
        });
    });

    ui.group(|ui| {
        ui.label("Остальное");

        ui.add(BooleanIndicator {
            label: "Неправильная RFID карточка:".to_owned(),
            value_ref: &data.last_poll_result.door_invade,
        });

        ui.add(BooleanIndicator {
            label: "Акселерометр:".to_owned(),
            value_ref: &data.last_poll_result.accelerometer,
        });

        ui.add(BooleanIndicator {
            label: "Пожар:".to_owned(),
            value_ref: &data.last_poll_result.fire_detected,
        });
    });
}

fn render_login(data: &mut AppState, toasts: &mut Toasts, ui: &mut egui::Ui) {
    let username = data.input_username.clone();
    ui.label(format!("Will login as [{username}]"));

    ui.separator();

    ui.horizontal(|ui| {
        let name_label = ui.label("Username: ");
        ui.text_edit_singleline(&mut data.input_username)
            .labelled_by(name_label.id);
    });

    ui.horizontal(|ui| {
        let name_label = ui.label("Password: ");
        ui.text_edit_singleline(&mut data.input_password)
            .labelled_by(name_label.id);
    });

    if ui.button("Login").clicked() {
        if auth::is_valid(
            data.input_username.as_str(),
            data.input_password.as_str(),
        ) {
            data.input_password = "".to_owned();
            data.current_session = Some(SessionData {
                username: data.input_username.to_owned(),
                auth_level: AuthLevel::Modify,
                begin_timestamp: Utc::now(),
                timeout_time: if data.input_username == "god" {
                    TimeDelta::hours(24)
                } else {
                    TimeDelta::minutes(5)
                },
            });

            toasts.info(format!(
                "Logged in as {}",
                data.input_username
            ));
        } else {
            toasts.error("Invalid credentials");
        }
    }
}

fn render_session(session: &mut SessionData, ui: &mut egui::Ui) {
    let session_end_time = session.begin_timestamp + session.timeout_time;
    if Utc::now() > session_end_time {
        session.auth_level = AuthLevel::View;
        return;
    }

    display_session_header(session, ui, session_end_time);

    ui.separator();

    if ui.button("Logout").clicked() {
        session.auth_level = AuthLevel::View;
        return;
    }
}

fn display_session_header(
    session: &mut SessionData,
    ui: &mut egui::Ui,
    session_end_time: chrono::DateTime<Utc>,
) {
    let username = session.username.clone();
    ui.label(format!("Logged in as [{username}]"));

    let begin_ts = session
        .begin_timestamp
        .format("%Y-%m-%d %H:%M:%S");
    ui.label(format!("Session start: [{begin_ts}]"));

    let remaining_time = session_end_time - Utc::now();
    let seconds = remaining_time.num_seconds();
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    let remaining_time_str = format!(
        "Remaining session time: {:02}:{:02}",
        minutes, remaining_seconds
    );

    ui.label(remaining_time_str);
}
