use std::io;

pub struct InputConsole {
    input: String,
}
impl InputConsole {
    pub fn new() -> Self {
        InputConsole {
            input: String::new(),
        }
    }
}

impl super::Input for InputConsole {
    fn read(&mut self) -> Option<bool> {
        match io::stdin().read_line(&mut self.input) {
            Ok(_) => {
                println!("buffer: {}", self.input);
                if self.input.starts_with("play") {
                    self.input.clear();
                    Some(true)
                } else {
                    println!("Nothing..");
                    self.input.clear();
                    Some(false)
                }
            }
            Err(error) => {
                self.input.clear();
                println!("error: {}", error);
                None
            }
        }
    }
}
