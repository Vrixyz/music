#[cfg(not(feature = "music"))]
mod console;
#[cfg(feature = "music")]
mod music;

pub trait Actor {
    fn act(&mut self) -> Result<(), ()>;
}

#[cfg(not(feature = "music"))]
pub fn get_actor() -> console::ActorConsole {
    console::ActorConsole::new()
}
#[cfg(feature = "music")]
pub fn get_actor() -> music::ActorMusic {
    music::ActorMusic::new()
}