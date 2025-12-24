use crate::authorized::serial_connection::{PollResult, SerialConnection};
use eframe::egui::{Color32, Response, Ui, Widget};

#[derive(Debug)]
pub struct Api {
    active_connection: Option<SerialConnection>,
}

impl Api {
    pub fn new() -> Self {
        Self { active_connection: None }
    }

    pub fn widget(&mut self) -> impl Widget + '_ {
        ApiWidget { api: self }
    }

    pub fn close_connection(&mut self) {
        self.active_connection = None;
    }
}

impl Api {
    pub fn exists(&self) -> bool {
        self.active_connection.is_some()
    }

    pub fn send_poll(&self) -> anyhow::Result<PollResult> {
        match &self.active_connection {
            Some(active_connection) => active_connection.send_poll(),
            None => {
                anyhow::bail!("No active connection");
            }
        }
    }

    pub fn send_reset(&self) -> anyhow::Result<()> {
        match &self.active_connection {
            Some(active_connection) => active_connection.send_reset(),
            None => {
                anyhow::bail!("No active connection");
            }
        }
    }

    pub fn send_unlock_back_door(&self) -> anyhow::Result<()> {
        match &self.active_connection {
            Some(active_connection) => {
                active_connection.send_unlock_back_door()
            }
            None => {
                anyhow::bail!("No active connection");
            }
        }
    }

    pub fn send_lock_back_door(&self) -> anyhow::Result<()> {
        match &self.active_connection {
            Some(active_connection) => active_connection.send_lock_back_door(),
            None => {
                anyhow::bail!("No active connection");
            }
        }
    }

    pub fn send_unlock_front_door(&self) -> anyhow::Result<()> {
        match &self.active_connection {
            Some(active_connection) => {
                active_connection.send_unlock_front_door()
            }
            None => {
                anyhow::bail!("No active connection");
            }
        }
    }

    pub fn send_lock_front_door(&self) -> anyhow::Result<()> {
        match &self.active_connection {
            Some(active_connection) => active_connection.send_lock_front_door(),
            None => {
                anyhow::bail!("No active connection");
            }
        }
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
                    eprintln!("Failed to connect to port: {}", err);
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
