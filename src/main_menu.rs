use clearscreen;
use std::{fs, io, num::ParseIntError};
use text_colorizer::*;

pub fn start_game() {
    startup_messages();
    
}

fn startup_messages() {
    // Clear screen and print welcome messages
    clearscreen::clear().expect("Failed to clear screen");
    println!("{}", "Welcome to Flakes and Midgets!".red().bold());
    println!("1) New Game");
    println!("2) Load Game");
    println!("3) Exit");

    // get user choice
    let mut valid_selection = false;

    while valid_selection != true {
        // read input
        let mut choice = String::new();

        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    
        // parse choice to number
        match choice
            .trim()
            .parse::<u64>() 
            {
                Ok(n) => match n {
                    1 => {create_new_game(); valid_selection = true},
                    2 => {load_game(); valid_selection = true},
                    3 => {exit_game(); valid_selection = true},
                    _ => println!("No valid selection. Please try again."),
                }
                Err(e) => eprintln!("Please Input a Number"),
            };
    }
}

fn create_new_game() {
    clearscreen::clear().expect("Failed to clear screen");

    println!("What would you like to be called on your journey?");
    let mut player_name = String::new();

    // user sure about his name ?
    let mut decision = false;

    while decision != true {
        // read in player name
        io::stdin()
        .read_line(&mut player_name)
        .expect("Should have been valid input");

        // Ask for confirmation
        println!("Does the name {} make you happy ? y/n", player_name.green());
        let mut confirmation = String::new();

        io::stdin()
        .read_line(&mut confirmation)
        .expect("Should have been valid input");

        match confirmation.trim() {
            "y" => decision = true,
            "n" => decision = false,
            _ => {
                eprintln!("Your input should have been y or n");
            }
        }
    }
    // generate the savegame for thes name
    let mut path_to_savegame = String::from("./savegames/");
    path_to_savegame.push_str(&player_name);

    // create folder if it does not already exist
    let folder = fs::create_dir_all(path_to_savegame).expect("New Save could not be created");
}

fn load_game() -> std::io::Result<()> {
    // check if savegame folder exists
    let path_to_savegames = "./savegames";

    // Use read_dir to open the folder and iterate over its contents
    let entries = fs::read_dir(path_to_savegames)?;

    for entry in entries {
        // Unwrap the entry to get the DirEntry
        let entry = entry?;

        // Get the file or directory name as a String
        let file_name = entry.file_name();

        // Print the name of each file or directory in the folder
        println!("{}", file_name.to_string_lossy());
    }

    Ok(())
}

fn exit_game() {
    clearscreen::clear()
    .expect("Failed to clear screen");

    println!("Exiting the game");
    std::process::exit(1);
}