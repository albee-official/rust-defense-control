use crate::{
    auth,
    data::{AppState, AuthLevel, SessionData},
    widgets::boolean_indicator::BooleanIndicator,
};
use chrono::{TimeDelta, Utc};
use eframe::egui::PopupCloseBehavior::CloseOnClickOutside;
use eframe::egui::{self};

pub fn render(data: &mut AppState, ui: &mut egui::Ui) {
    ui.heading("Анти-(Анти-Автомат) System");
    
    let response = ui.button("Auth");
    
    egui::Popup::menu(&response)
        .close_behavior(CloseOnClickOutside)
        .width(220.0)
        .show(|ui| {
            if let Some(session) = data.current_session.as_mut() {
                render_session(session, ui);

                if session.auth_level == AuthLevel::View {
                    data.current_session = None;
                    data.enable_editing
                        .set(data.current_session.is_some());
                }
            } else {
                render_login(data, ui);
            }
        });

    render_data(data, ui);
}

fn render_data(data: &mut AppState, ui: &mut egui::Ui) {
    ui.group(|ui| {
        ui.label("General:");

        if data.current_session.is_some() {
            if ui.button("Poll").clicked() {
                data.api.send_poll();
            }

            if ui.button("Reset").clicked() {
                data.api.send_reset();
            }

            if ui.button("Alarm!!!!!!").clicked() {
                data.api.send_activate_alarm();
            }
        }

        ui.add(BooleanIndicator {
            label: "Is On Battery:".to_owned(),
            value_ref: &data.sensor_data.is_on_battery,
        });
    });
}

fn render_login(data: &mut AppState, ui: &mut egui::Ui) {
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

            data.enable_editing
                .set(data.current_session.is_some());
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
