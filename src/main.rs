use rand::seq::SliceRandom;
use rodio::Sink;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;

struct Player {
    random: rand::rngs::ThreadRng,
    sound_paths: Vec<String>,
    prev_sink: Option<Sink>,
}

impl Player {
    fn new(random: rand::rngs::ThreadRng, sound_paths: Vec<String>) -> Self {
        Self {random, sound_paths, prev_sink: None}
    }
    fn play_random(&mut self) {
        let path = self.sound_paths.choose(&mut self.random).unwrap();
        println!("Play {}!", path);
        // TODO: Next step 2: connect to bluetooth manually and send music file. (so we can then play to different positions)
        let device = rodio::default_output_device().unwrap();
        let file = File::open(path).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        let sink = Sink::new(&device);
        sink.append(source);
        if let Some(prev_sink) = self.prev_sink.take() {
            prev_sink.stop();
        }
        sink.play();
        self.prev_sink = Some(sink);
    }
}

trait Input {
    fn read(&mut self) -> Option<bool>;
}
struct InputConsole {
    input: String,
}
impl InputConsole {
    fn new() -> Self {
        InputConsole {input: String::new()}
    }
}

impl Input for InputConsole {
    fn read(&mut self) -> Option<bool> {
        match io::stdin().read_line(&mut self.input) {
            Ok(_) => {
                if self.input.starts_with("play") {
                    Some(true)
                } else {
                    println!("Nothing..");
                    Some(false)
                }
            }
            Err(error) => {
                println!("error: {}", error);
                None
            },
        }
    }
}


use std::io::{Write};
use std::time::Duration;

use clap::{App, AppSettings, Arg};
use serialport::prelude::*;

struct InputSerial {
    serial_buf: Vec<u8>,
    port: Box<dyn SerialPort>,
}

impl InputSerial {
    fn new(port_name: &str, baud_rate: u32) -> Result<Self, String> {
        
        let mut settings: SerialPortSettings = Default::default();
        settings.timeout = Duration::from_millis(10);
        settings.baud_rate = baud_rate;

        match serialport::open_with_settings(&port_name, &settings) {
            Ok(mut port) => {
                Ok(InputSerial{serial_buf: vec![0; 1000], port})
            },
            _ => {
                Err("fuck".to_string())
            }
        }
    }
}

impl Input for InputSerial {
    fn read(&mut self) -> Option<bool> {
        match self.port.read(self.serial_buf.as_mut_slice()) {
            Ok(t) => Some(true),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => None,
            Err(e) => None,
        }
    }
}

fn main() {
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
    let mut input : InputSerial;
    if let Ok(baud_rate) = baud_rate.parse::<u32>() {
        input = InputSerial::new(port_name, baud_rate).unwrap();
    } else {
        eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
        ::std::process::exit(1);
    }

    let random = rand::thread_rng();
    let sound_paths = fs::read_dir("assets/ogg/")
        .unwrap()
        .map(|res| res.map(|e| e.path().display().to_string()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    let mut player = Player::new(random, sound_paths);

    // TODO: Next step 1: physical button to trigger a sound
    //let mut input  = InputConsole::new();
    
    loop {
        if let Some(read) = input.read() {
            if read {
                player.play_random();
            }
        }
    }
}
