use binrobot_rs::{
    api::{self, types::Binrobot},
    message,
};
use std::{thread, time};

fn main() {
    message::wait_for_starting();

    loop {
        for _ in 0..11 {
            let mv = api::command::CommandAPI::Move {
                direction: api::types::Direction::Right,
                velocity: api::types::Number(1.0),
            }
            .to_command()
            .unwrap();

            println!("{}", mv);
            thread::sleep(time::Duration::from_millis(100));
        }

        for _ in 0..11 {
            let mv = api::command::CommandAPI::Move {
                direction: api::types::Direction::Down,
                velocity: api::types::Number(1.0),
            }
            .to_command()
            .unwrap();

            println!("{}", mv);
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}
