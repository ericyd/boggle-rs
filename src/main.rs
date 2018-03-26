/// Simple command-line version of Boggle®

extern crate term;
extern crate rand;

use std::io::{self, Write};
use std::fs::File;
use std::io::{BufReader, BufRead};

mod board;
mod game;
mod timer;

use board::Board;
use game::Game;
use game::Player;
use game::Guesses;
use timer::Timer;

fn main() {
    println!("Welcome to Boggle®");
    println!("==================\n");

    let dictionary: Option<Vec<String>> = match File::open("dictionary.txt") {
        Ok(file) => {
            let lines = BufReader::new(file)
                .lines()
                .filter(|line| match line {
                    &Ok(ref _l) => true,
                    &Err(_) => false,
                })
                .map(|line| line.unwrap().to_uppercase())
                .collect();
            Some(lines)
        }
        Err(_) => {
            println!(
                "\nWARNING: There was no dictionary file available! \
                \nThe game will not check that your words exist in the dictionary.\n \
                \nFor future games, please download this file, name it \"dictionary.txt\" \
                and place it in the same directory as this program \
                \nhttps://www.wordgamedictionary.com/twl06/download/twl06.txt\n\n"
            );
            None
        }
    };

    // to "prompt" on the same line, call print! macro followed by a flush
    print!("For how many minutes would you like to play? (decimals OK) ");
    io::stdout().flush().unwrap();

    let mut timer = Timer::new();
    timer.get_user_play_time();
    clean_prev_line();

    print!("Please enter your name: ");
    io::stdout().flush().unwrap();

    // read name
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect(
        "Failed to read line",
    );
    clean_prev_line();

    // create game with a new board
    let mut game = Game {
        board: Board::new(dictionary),
        player: Player::new(String::from(name.trim())),
        guesses: Guesses::new(),
    };

    println!("Hello {}, here is your game:", game.player.name);
    println!(
        "Enter as many words as possible in {} mins!",
        timer.max_time_minutes
    );
    println!("{}", game.board);

    timer.start();
    loop {
        if timer.is_time_up() {
            break;
        }
        println!(
            "Now start typing words! ({} seconds left)",
            timer.get_remaining_time()
        );

        // get guess
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect(
            "Failed to read line",
        );

        // add to guesses if time is remaining
        // (i.e. don't allow guesses after time has run out)
        clean_prev_line();
        clean_prev_line();

        if timer.is_time_up() {
            println!(
                "Oooh, so close! But \"{}\" was entered after time ran out",
                line.trim()
            );
            break;
        } else {
            game.add_guess(String::from(line.trim()));
        }
    }

    println!("Nice job! Here are your results:");
    println!("{}", game.guesses);
    println!("\nPress enter to exit the program");
    let mut end = String::new();
    io::stdin().read_line(&mut end).expect(
        "Failed to read line",
    );
}

fn clean_prev_line() {
    let mut term = term::stdout().unwrap();
    term.cursor_up().unwrap();
    term.delete_line().unwrap();
}
