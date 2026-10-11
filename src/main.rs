use crossterm::{
    ExecutableCommand, QueueableCommand, cursor, style::{self, ContentStyle, Stylize}, terminal,
};
use k_board::{keyboard::Keyboard, keys::Keys};
use rodio;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Write};
use std::time::Duration;

fn main() -> std::io::Result<()> {

            clearscreen::clear().expect("failed to clear screen");
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
                }
                _ => {}
            };
            clearscreen::clear().expect("failed to clear screen");
            menu(player.volume(), player.get_pos());
        }
    }
    Ok(())
}

fn menu(mut volume: f32, player_pos: Duration) -> std::io::Result<()> {
    //for 1..10{
    //    println!("")
    //}
    
    let termWidth : u16 = crossterm::terminal::size().unwrap().0;
    let termHeight : u16 = crossterm::terminal::size().unwrap().1;

    if volume < 0.0{
        volume = 0.0;
    }

    let mut stdout = io::stdout();
    let menu_text = format!("Volume is {:.2} Duration is {:?} Seconds",volume,player_pos.as_secs());
    for y in 0..termHeight {
        for x in 0..termWidth {
            if (y == 0 || y == termHeight - 1) || (x == 0 || x == termWidth - 1) {
                // in this loop we are more efficient by not flushing the buffer.
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("█".magenta()))?;
            }
            if (x==(termWidth / 2) - u16::try_from(menu_text.len() / 2).unwrap())
                && (y==termHeight /2) {
                stdout.queue(cursor::MoveTo(x,y))?
                    .queue(style::PrintStyledContent(menu_text.clone()
                            .magenta()))?;
            }

    }
    }
    stdout.flush();

    Ok(())
}
