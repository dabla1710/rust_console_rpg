use clearscreen;
use std::{time::Duration, fs, io::{self, ErrorKind}, num::ParseIntError, error::Error, mem::Discriminant};
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

    while &mut valid_selection != &true {
        // read input
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // parse choice to number
        match choice.trim().parse::<u64>() {
            Ok(n) => match n {
                1 => {
                    create_new_game();
                    valid_selection = true
                }
                2 => {
                    load_game();
                    valid_selection = true
                }
                3 => {
                    exit_game();
                    valid_selection = true
                }
                _ => println!("No valid selection. Please try again."),
            },
            Err(e) => eprintln!("{} - Please Input a Number", e),
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
        println!("Does the name {} make you happy ? y/n", player_name.trim_end().green());
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
    // generate the savegame for given name
    let mut path_to_savegame = String::from("./savegames/");
    path_to_savegame.push_str(&player_name.trim_end());

    // create folder if it does not already exist
    let folder = fs::create_dir_all(path_to_savegame).expect("Failed to create savegame!");
}

fn load_game() {
    // check if savegame folder exists and print loadable saves
    clearscreen::clear().expect("Failed to clear screen");
    println!("{}", "Which savegame would you like to be loaded ?".blue());
    let path_to_savegames = "./savegames";

    if let Ok(folders) = fs::read_dir(path_to_savegames) {
        for savegame in folders {
            let savegame = savegame.unwrap().file_name();
            println!("{:?}", savegame);
        }
    } else {
        // if there are no previous savegames -> go back to the main menu
        startup_messages();
    };

    // Let user decide which savegame to load
    let mut save_game_selection = String::new();

    io::stdin()
    .read_line(&mut save_game_selection)
    .expect("No valid input !");

    if let Ok(folders) = fs::read_dir(path_to_savegames) {
        'compare_savegames: for savegame in folders {
            let savegame = savegame.unwrap().file_name();
            // check if safegame is the same as user input
            if save_game_selection.trim_end() == savegame {
                // TODO:  Load data in player struct
                println!("Savegame: {} is being loaded !", save_game_selection.trim_end().green());
                break 'compare_savegames;
            }
        }
    } else {
        println!("No matching savegame found - Returning to main menu");
        std::thread::sleep(Duration::from_secs(3));
        // if there are no previous savegames -> go back to the main menu
        startup_messages();
    };
}

fn exit_game() {
    clearscreen::clear().expect("Failed to clear screen");

    println!("Exiting the game");
    std::process::exit(1);
}

