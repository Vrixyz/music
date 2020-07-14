#[cfg(not(feature = "bluetooth"))]
mod console;
#[cfg(feature = "bluetooth")]
mod serial;

pub trait Input {
    fn read(&mut self) -> Option<bool>;
}

#[cfg(not(feature = "bluetooth"))]
pub fn get_input() -> console::InputConsole {
    console::InputConsole::new()
}
#[cfg(feature = "bluetooth")]
pub fn get_input() -> serial::InputSerial {
    use clap::{App, AppSettings, Arg};
    let matches = App::new("Serialport Example - Receive Data")
        .about("Reads data from a serial port and echoes it to stdout")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("port")
                .help("The device path to a serial port")
                .use_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::with_name("baud")
                .help("The baud rate to connect at")
                .use_delimiter(false)
                .required(true),
        )
        .get_matches();
    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap();
    let input: serial::InputSerial;
    if let Ok(baud_rate) = baud_rate.parse::<u32>() {
        input = serial::InputSerial::new(port_name, baud_rate).unwrap();
    } else {
        eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
        ::std::process::exit(1);
    }
    input
}
