use crossterm::{
    QueueableCommand, cursor,
    style::{self, Stylize},
};
use k_board::{keyboard::Keyboard, keys::Keys};
use rodio;
use std::io::BufReader;
use std::io::{self, Write};
use std::time::Duration;
use std::fs::File;

mod filesWork;

use crate::filesWork::filesWork::AllFilesInDirectory;
use crate::filesWork::filesWork::GetFileInDirectory;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    dbg!(args.clone());

    if args.len() <= 1 {
        panic!("No args found");
    }

    let file_path = &args[1];


    //clearscreen::clear().expect("failed to clear screen");
    let sink_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let soundFileResult = GetFileInDirectory(String::from(file_path), String::from("test.mp3"));

    let soundFile = match soundFileResult {
        Ok(f) => f,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(soundFile);
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
            //clearscreen::clear().expect("failed to clear screen");
            AllFilesInDirectory(String::from("./"))
            //menu(player.volume(), player.get_pos());
        }
    }
    Ok(())
}

fn menu(mut volume: f32, player_pos: Duration) -> std::io::Result<()> {
    let termWidth: u16 = crossterm::terminal::size().unwrap().0;
    let termHeight: u16 = crossterm::terminal::size().unwrap().1;

    if volume < 0.0 {
        volume = 0.0;
    }

    let mut stdout = io::stdout();
    let menu_text = format!(
        "Volume is {:.2} Duration is {:?} Seconds",
        volume,
        player_pos.as_secs()
    );
    for y in 0..termHeight {
        for x in 0..termWidth {
            if (y == 0 || y == termHeight - 1) || (x == 0 || x == termWidth - 1) {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("█".magenta()))?;
            }
            if (x == (termWidth / 2) - u16::try_from(menu_text.len() / 2).unwrap())
                && (y == termHeight / 2)
            {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent(menu_text.clone().magenta()))?;
            }
        }
    }
    let flush = stdout.flush();

    match flush {
        Ok(_v) => {}
        Err(_e) => panic!("Flush Error"),
    }

    Ok(())
}
