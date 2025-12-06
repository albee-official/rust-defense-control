use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::fluent::containers::*;

use serialport::SerialPort;

#[derive(Debug)]
pub struct ControllerPoller {
    pub port: Arc<Mutex<Box<dyn SerialPort>>>,
    poll_thread_context: Arc<Mutex<PollThreadState>>,
}

#[derive(Debug)]
pub struct PollThreadState {
    pub keep_alive: bool,
    pub state: ControllerState,
}

#[derive(Debug)]
pub struct ControllerState {
    pub alarm_enabled: bool,
}

impl Drop for ControllerPoller {
    fn drop(&mut self) {
        self.poll_thread_context.clear_poison();

        match self.poll_thread_context.lock() {
            Ok(mut value) => value.keep_alive = false,
            Err(mut e) => e.get_mut().keep_alive = false,
        }
    }
}

impl ControllerPoller {
    pub fn new() -> serialport::Result<Self> {
        let mut selected_port_name = "".to_owned();
        for available_port in serialport::available_ports().unwrap().iter() {
            println!("Available port: {}", available_port.port_name);
            selected_port_name = available_port.port_name.to_owned();
        }

        println!("connecting to: {}", selected_port_name);
        let connection = serialport::new(selected_port_name.to_owned(), 9600)
            .timeout(Duration::from_millis(1000))
            .flow_control(serialport::FlowControl::None)
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::One)
            .parity(serialport::Parity::None)
            .open();

        if let Err(connection_error) = connection {
            eprintln!(
                "Connection to port: {} failed. Reason: {}",
                selected_port_name.to_owned(),
                connection_error
            );

            return Err(connection_error);
        }

        let connection = connection.unwrap().in_mutex().in_arc();
        let controller_state = PollThreadState {
            keep_alive: true,
            state: ControllerState {
                alarm_enabled: true,
            },
        }
        .in_mutex()
        .in_arc();

        let poll_thread_controller_state = controller_state.clone();
        let poll_thread_connection = connection.clone();
        std::thread::spawn(move || {
            println!("Starting poll thread.");

            loop {
                let mut connection = poll_thread_connection
                    .lock()
                    .expect("Poisoned connection mutex.");

                println!("Sending code [0xAA]...");
                connection
                    .write_all(&[0xAA])
                    .expect("Failed to write to port!");
                connection.flush().expect("Failed to send data");

                let mut receieved = [0u8; 8];
                connection
                    .read(&mut receieved)
                    .expect("Failed to read from port!");

                println!("Received: {:#?}", receieved);
                std::mem::drop(connection);

                let mut state = poll_thread_controller_state
                    .lock()
                    .expect("Poisoned controller state Mutex.");

                state.state.alarm_enabled = !state.state.alarm_enabled;

                if !state.keep_alive {
                    break;
                }

                std::mem::drop(state);
                thread::sleep(Duration::from_millis(1000));
            }

            println!("Ending poll thread.");
        });

        return Ok(Self {
            port: connection,
            poll_thread_context: controller_state,
        });
    }
}
