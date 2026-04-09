use rodio::{Decoder, DeviceSinkBuilder, Player, Source};
use std::io::Cursor;
use winput::message_loop;
use winput::{Action, Button};

fn main() {
    let receiver = message_loop::start().unwrap();
    let handle = DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

    let mut players = Vec::new();

    let file = include_bytes!("waa.ogg");

    println!("waa.ogg will now be played");
    println!("blame arti (https://arti.gay)");

    loop {
        match receiver.next_event() {
            message_loop::Event::MouseButton {
                button,
                action: Action::Press,
                ..
            } => {
                if button == Button::Left {
                    let player = Player::connect_new(handle.mixer());
                    let cursor = Cursor::new(file);
                    let source = Decoder::try_from(cursor).unwrap();
                    player.append(source.amplify(0.3));
                    players.push(player);
                }
            }
            _ => (),
        }
    }
}
