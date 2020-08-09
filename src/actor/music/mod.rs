use rand::seq::SliceRandom;
use rodio::Sink;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use super::Actor;

pub struct ActorMusic {
    random: rand::rngs::ThreadRng,
    sound_paths: Vec<String>,
    prev_sink: Option<Sink>,
}

impl ActorMusic {
    pub fn new() -> Self {
        let random = rand::thread_rng();
        let sound_paths = fs::read_dir("assets/ogg/")
            .unwrap()
            .map(|res| res.map(|e| e.path().display().to_string()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();
            Self {
                random,
                sound_paths,
                prev_sink: None,
            }
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

impl Actor for ActorMusic {
    fn act(&mut self) {
        self.play_random();
    }
}