mod input;
mod actor;

use input::Input;
use actor::Actor;

fn main() {
    let mut actor = actor::get_actor();
    let mut input = input::get_input();

    loop {
        if let Some(read) = input.read() {
            if read {
                actor.act();
            }
        }
    }
}
