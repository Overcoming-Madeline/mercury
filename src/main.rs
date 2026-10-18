use crossterm::{
    QueueableCommand, cursor,
    style::{self, Stylize},
};
use k_board::{keyboard::Keyboard, keys::Keys};
use rodio::Source;
use rodio::{Decoder, Player, DeviceSinkBuilder};
use std::io::{BufReader, Read, Seek};
use std::io::{self, Write};
use std::time::Duration;
use std::{fs::File, path::Path};

mod files_work;

use crate::files_work::files_work::get_files_in_directory;
use crate::files_work::files_work::get_files_in_directory_with_name;
use crate::files_work::files_work::get_all_audio_files_in_directory;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    dbg!(args.clone());

    if args.len() <= 1 {
        panic!("No args found");
    }

    let file_path = &args[1];

    let path = Path::new(&file_path);

    let sound_file_result = get_files_in_directory_with_name(file_path.clone());

    //clearscreen::clear().expect("failed to clear screen");
    let sink_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

    let sound_file = match sound_file_result {
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

    let file = BufReader::new(sound_file);
    let player = rodio::play(sink_handle.mixer(), file).unwrap();
    
    let audiofile = get_all_audio_files_in_directory(path.to_str().unwrap().to_string());

    match audiofile {
    Ok(files) => {
        for file in files {
            match Decoder::try_from(file) {
                Ok(source) => {player.append(source);
                println!("loaded file {}",source.size_hint)
                },
                Err(error) => eprintln!("Failed to decode audio: {error}"),
            }
        }
    }
    Err(error) => {
        panic!("Failed to load audio files: {error}");
     }
    }
    while !player.empty() {
        for key in Keyboard::new() {
            match key {
                Keys::Up => {
                    player.set_volume({
                        let mut tmp = player.volume();
                        tmp += 0.01;
                        tmp
                    });
                }
                Keys::Down => {
                    player.set_volume({
                        let mut tmp = player.volume();
                        tmp -= 0.01;
                        tmp
                    });
                }
                _ => {}
            };
            if player.volume() < 0.0 {
                player.set_volume(0.0);
            }
            //clearscreen::clear().expect("failed to clear screen");
            //all_files_in_directory(String::from("./"))
            //let menu_result = menu(player.volume(), player.get_pos());

            match menu_result {
                Ok(()) => {},
                Err(_) => {panic!("Menu Error ");}
            }
        }
    }
    Ok(())
}

fn menu(mut volume: f32, player_pos: Duration) -> std::io::Result<()> {
    let term_width: u16 = crossterm::terminal::size().unwrap().0;
    let term_height: u16 = crossterm::terminal::size().unwrap().1;

    if volume < 0.0 {
        volume = 0.0;
    }

    let mut stdout = io::stdout();
    let menu_text = format!(
        "Volume is {:.0}% Duration is {:?} Seconds",
        volume * 100.0,
        player_pos.as_secs()
    );
    for y in 0..term_height {
        for x in 0..term_width {
            if (y == 0 || y == term_height - 1) || (x == 0 || x == term_width - 1) {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("█".magenta()))?;
            }
            if (x == (term_width / 2) - u16::try_from(menu_text.len() / 2).unwrap())
                && (y == term_height / 2)
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
