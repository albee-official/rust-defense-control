use crate::authorized::serial_connection::SerialConnection;
use eframe::egui::{Color32, Response, Ui, Widget};
use egui_notify::Toasts;

#[derive(Debug)]
pub struct Api {
    active_connection: Option<SerialConnection>,
}

impl Api {
    pub fn exists(&self) -> bool {
        self.active_connection.is_some()
    }
}

impl Api {
    pub fn send_poll(&self, toasts: &mut Toasts) {
        if let Some(active_connection) = &self.active_connection {
            match active_connection.send_poll() {
                Ok(res) => {
                    println!("Res: {res:?}");
                }
                Err(e) => {
                    toasts.error(format!("Failed to poll: {}", e));
                    eprintln!("{}", e);
                }
            }
        }
    }

    pub fn send_reset(&self, toasts: &mut Toasts) {
        if let Some(active_connection) = &self.active_connection {
            match active_connection.send_reset() {
                Ok(_) => (),
                Err(e) => {
                    toasts.error(format!("Failed to send reset: {}", e));
                    eprintln!("{}", e);
                }
            }
        }
    }

    pub fn send_activate_alarm(&self, toasts: &mut Toasts) {
        if let Some(active_connection) = &self.active_connection {
            match active_connection.send_activate_alarm() {
                Ok(_) => (),
                Err(e) => {
                    toasts.error(format!(
                        "Failed to send activate alarm: {}",
                        e
                    ));
                    eprintln!("{}", e);
                }
            }
        }
    }
}

impl Api {
    pub fn new() -> Self {
        Self { active_connection: None }
    }

    pub fn widget(&mut self) -> impl Widget {
        ApiWidget { api: self }
    }
}

#[derive(Debug)]
pub struct ApiWidget<'a> {
    api: &'a mut Api,
}

impl Widget for ApiWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let connection = match &mut self.api.active_connection {
            Some(c) => c,
            None => {
                self.connection_picker(ui);
                return ui.colored_label(Color32::RED, "No connection!");
            }
        };

        ui.add(connection.widget());
        if ui.button("Disconnect").clicked() {
            self.api.active_connection = None;
        }

        ui.response()
    }
}

impl ApiWidget<'_> {
    fn connection_picker(self, ui: &mut Ui) -> Response {
        ui.label("Connect to port:");

        let ports = match serialport::available_ports() {
            Ok(p) => p,
            Err(err) => {
                return ui.colored_label(
                    Color32::RED,
                    format!("Failed to get ports: {}", err),
                );
            }
        };

        let mut selected = None;
        for p in ports {
            if ui
                .button(p.port_name.as_str())
                .clicked()
            {
                selected = Some(p.port_name);
            }
        }

        if let Some(selected) = selected {
            println!("Connecting to port: {}", selected);
            match SerialConnection::new(selected.as_str()) {
                Ok(connection) => {
                    self.api.active_connection = Some(connection);
                }

                Err(err) => {
                    return ui.colored_label(
                        Color32::RED,
                        format!("Failed to connect to port: {}", err),
                    );
                }
            }
        }

        ui.response()
    }
}
