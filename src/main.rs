use rdev::{Button, Event, EventType, listen};
use rodio::{Decoder, DeviceSinkBuilder, Player, Source};
use std::io::Cursor;

fn main() {
    let handle = DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let mut players = Vec::new();
    let file = include_bytes!("waa.ogg");

    println!("waa.ogg will now be played");
    println!("blame arti (https://arti.gay)");

    // Callback for handling events
    let callback = move |event: Event| {
        match event.event_type {
            EventType::ButtonPress(Button::Left) => {
                let player = Player::connect_new(handle.mixer());
                let cursor = Cursor::new(file);
                let source = Decoder::try_from(cursor).unwrap();
                player.append(source.amplify(0.3));
                players.push(player);
            }
            _ => (), // Ignore other events
        }
    };

    // Start listening (this blocks)
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
