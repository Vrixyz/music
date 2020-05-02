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

fn main() {
    let random = rand::thread_rng();
    let sound_paths = fs::read_dir("assets/ogg/")
        .unwrap()
        .map(|res| res.map(|e| e.path().display().to_string()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    let mut player = Player::new(random, sound_paths);
    // TODO: Next step 1: physical button to trigger a sound
    let mut input  = InputConsole::new();
    loop {
        if let Some(read) = input.read() {
            if read {
                player.play_random();
            }
        }
    }
}
