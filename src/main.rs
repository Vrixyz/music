mod input;
mod actor;

use input::Input;
use actor::Actor;
use std::{thread, time};

fn main() {
    let mut actor = actor::get_actor();

    loop {
        // TODO: maybe some shell init (sudo bluetoothctl < connect device_id)
        let input = input::get_input();
        match input {
            Ok(mut input) => {
                loop {
                    if let Some(read) = input.read() {
                        if read {
                            if (actor.act().is_err()) {
                                println!("action did have an error.");
                                break;
                            }
                        }
                        else {
                            println!("We read nothing, This is unusual.");
                            break;
                        }
                    }
                }
            },
            Err(e) => {
                println!("{:#?}", e);
            }
        }
        thread::sleep(time::Duration::from_millis(10));
        println!("Retrying...");
    }
    
}
