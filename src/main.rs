use crossterm::{
    QueueableCommand, cursor,
    style::{self, Stylize},
};
use k_board::{keyboard::Keyboard, keys::Keys};
use std::{fs::File, path::Path};
use std::io::BufReader;
use std::io::{self, Write};
use std::time::Duration;

mod files_work;

use crate::files_work::files_work::all_files_in_directory;
use crate::files_work::files_work::get_files_in_directory;
use crate::files_work::files_work::get_files_in_directory_with_name;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    dbg!(args.clone());

    if args.len() <= 1 {
        panic!("No args found");
    }

    let file_path = &args[1];
    
    let path = Path::new(&file_path);


    
    let mut sound_file_result = get_files_in_directory_with_name(file_path.clone());

    if path.is_dir() {
        if(args.len() <= 2){
            panic!("No file Name")
        }

        let file_name = &args[2];
        sound_file_result = get_files_in_directory(String::from(file_path), String::from(file_name));
    }
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

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(sound_file);
    let player = rodio::play(sink_handle.mixer(), file).unwrap();
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
            //all_files_in_directory(String::from("./"))
            menu(player.volume(), player.get_pos());
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
        "Volume is {:.2} Duration is {:?} Seconds",
        volume,
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
