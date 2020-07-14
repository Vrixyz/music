use serialport::prelude::*;
use std::io;
use std::io::Write;
use std::time::Duration;

pub struct InputSerial {
    serial_buf: Vec<u8>,
    port: Box<dyn SerialPort>,
}

impl InputSerial {
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self, String> {
        let mut settings: SerialPortSettings = Default::default();
        settings.timeout = Duration::from_millis(10);
        settings.baud_rate = baud_rate;

        match serialport::open_with_settings(&port_name, &settings) {
            Ok(mut port) => Ok(InputSerial {
                serial_buf: vec![0; 1000],
                port,
            }),
            Err(e) => Err(format!("open serial port failed with error: {:#?}", e).to_string()),
        }
    }
}

impl super::Input for InputSerial {
    fn read(&mut self) -> Option<bool> {
        match self.port.read(self.serial_buf.as_mut_slice()) {
            Ok(t) => Some(true),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => None,
            Err(e) => None,
        }
    }
}
