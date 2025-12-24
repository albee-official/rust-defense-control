use crate::fluent::bit_inspect::BitInspect;
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
    pub open_door_front: bool,
    pub motion_detected_1: bool,
    pub motion_detected_2: bool,
    pub accelerometer: bool,
    pub fire_detected: bool,
    pub door_invade: bool,
    pub open_door_back: bool,
}

impl SerialConnection {
    pub fn new(port_name: &str) -> anyhow::Result<Self> {
        let opened = serialport::new(port_name, 9600)
            .timeout(std::time::Duration::from_secs(3))
            .open()?;

        Ok(Self { port: opened.in_mutex().in_arc() })
    }

    pub fn send_poll(&self) -> anyhow::Result<PollResult> {
        let mut guard = self
            .port
            .lock()
            .map_err(|_| anyhow::anyhow!("Poisoned mutex"))?;

        println!("Sending: 0xAA");
        guard.write(&[0xAA])?;

        let mut response = [0u8; 1];
        guard.read_exact(&mut response)?;

        println!("Received response: {:?}", response);

        Ok(PollResult {
            open_door_front: response[0].is_bit_set(1),
            motion_detected_1: response[0].is_bit_set(6),
            motion_detected_2: response[0].is_bit_set(5),
            accelerometer: response[0].is_bit_set(4),
            fire_detected: response[0].is_bit_set(3),
            door_invade: response[0].is_bit_set(2),
            open_door_back: response[0].is_bit_set(7),
        })
    }

    pub fn send_reset(&self) -> anyhow::Result<()> {
        let mut guard = self
            .port
            .lock()
            .map_err(|_| anyhow::anyhow!("Poisoned mutex"))?;

        println!("Sending: 0x55");

        guard.write(&[0x55])?;

        let mut response = [0u8; 1];
        guard.read_exact(&mut response)?;

        println!("Received response: {:?}", response);

        Ok(())
    }

    pub fn send_unlock_back_door(&self) -> anyhow::Result<()> {
        let mut guard = self
            .port
            .lock()
            .map_err(|_| anyhow::anyhow!("Poisoned mutex"))?;

        println!("Sending: 0xA3");

        guard.write(&[0xA3])?;

        let mut response = [0u8; 1];
        guard.read_exact(&mut response)?;

        println!("Received response: {:?}", response);

        Ok(())
    }

    pub fn send_lock_back_door(&self) -> anyhow::Result<()> {
        let mut guard = self
            .port
            .lock()
            .map_err(|_| anyhow::anyhow!("Poisoned mutex"))?;

        println!("Sending: 0xA2");

        guard.write(&[0xA2])?;

        let mut response = [0u8; 1];
        guard.read_exact(&mut response)?;

        println!("Received response: {:?}", response);

        Ok(())
    }

    pub fn send_lock_front_door(&self) -> anyhow::Result<()> {
        let mut guard = self
            .port
            .lock()
            .map_err(|_| anyhow::anyhow!("Poisoned mutex"))?;

        println!("Sending: 0xA4");

        guard.write(&[0xA4])?;

        let mut response = [0u8; 1];
        guard.read_exact(&mut response)?;

        println!("Received response: {:?}", response);

        Ok(())
    }

    pub fn send_unlock_front_door(&self) -> anyhow::Result<()> {
        let mut guard = self
            .port
            .lock()
            .map_err(|_| anyhow::anyhow!("Poisoned mutex"))?;

        println!("Sending: 0xA5");

        guard.write(&[0xA5])?;

        let mut response = [0u8; 1];
        guard.read_exact(&mut response)?;

        println!("Received response: {:?}", response);

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
