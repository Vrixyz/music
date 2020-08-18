use super::Actor;

pub struct ActorConsole {
}

impl ActorConsole {
    pub fn new() -> Self {
        Self{}
    }
}

impl Actor for ActorConsole {
    fn act(&mut self) -> Result<(), ()> {
        println!("Act!");
        Ok(())
    }
}