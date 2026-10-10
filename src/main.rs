use k_board::{keyboard::Keyboard, keys::Keys};
use rodio::{Decoder, MixerDeviceSink, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let sink_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("test.mp3").unwrap());
    let player = rodio::play(&sink_handle.mixer(), file).unwrap();
    while !player.empty() {
        for key in Keyboard::new() {
            match key {
                Keys::Up => {
                    player.set_volume({
                        let mut tmp = player.volume();
                        tmp += 0.1;
                        tmp
                    });
                }
                Keys::Down => {
                    player.set_volume({
                        let mut tmp = player.volume();
                        tmp -= 0.1;
                        tmp
                    });
                },
                _ => {

                }
            };
        clearscreen::clear().expect("failed to clear screen");
        menu(player.volume(),player.get_pos());
        }
    }
    Ok(())
}

fn menu(volume : f32, player_pos : Duration) {

    //for 1..10{
    //    println!("")
    //}
    println!("Volume is {} \nDuration is {:?} ",volume,player_pos.as_secs())
}
