use crate::fluent::containers::Innable;
use eframe::egui::{Color32, Response, Ui, Widget};
use serialport::SerialPort;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct SerialConnection {
    port: Arc<Mutex<Box<dyn SerialPort>>>,
}

#[derive(Debug)]
pub struct PollResult {
    pub led_alive: bool,
}

impl SerialConnection {
    pub fn new(port_name: &str) -> anyhow::Result<Self> {
        let opened = serialport::new(port_name, 9600)
            .timeout(std::time::Duration::from_secs(10))
            .open()?;

        Ok(Self { port: opened.in_mutex().in_arc() })
    }

    pub fn send_poll(&self) -> anyhow::Result<PollResult> {
        let mut guard = self
            .port
            .lock()
            .map_err(|_| anyhow::anyhow!("Poisoned mutex"))?;

        println!("Sending poll");
        guard.write(&[0xAA])?;

        let mut response = [0u8; 2];
        guard.read_exact(&mut response)?;

        println!("Received response: {:?}", response);

        Ok(PollResult { led_alive: true })
    }

    pub fn send_reset(&self) -> anyhow::Result<()> {
        let mut guard = self
            .port
            .lock()
            .map_err(|_| anyhow::anyhow!("Poisoned mutex"))?;

        guard.write(&[0x55])?;
        Ok(())
    }

    pub fn send_activate_alarm(&self) -> anyhow::Result<()> {
        let mut guard = self
            .port
            .lock()
            .map_err(|_| anyhow::anyhow!("Poisoned mutex"))?;

        guard.write(&[0xA5])?;
        Ok(())
    }

    pub fn widget(&mut self) -> impl Widget {
        SerialConnectionWidget { connection: self }
    }
}

pub struct SerialConnectionWidget<'a> {
    connection: &'a mut SerialConnection,
}

impl Widget for SerialConnectionWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        if let Ok(mut guard) = self.connection.port.lock() {
            let port_name = guard
                .name()
                .unwrap_or_else(|| "<unknown port>".to_owned());

            ui.label(format!("Connected to port {port_name}"))
        } else {
            ui.colored_label(Color32::RED, "Mutex Poisoned")
        }
    }
}
