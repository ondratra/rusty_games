use std::io::stdin;

use super::game::Game;
use super::board::{FieldValue, BoardPosition};


pub struct CliInterface {
    game: Game
}

enum GameErrors {
    InvalidInput,
    OutOfGameboard,
    FieldArleadyOccupied,
}

impl CliInterface {
    pub fn new() -> CliInterface {
        CliInterface {
            game: Game::new(5, 3)
        }
    }

    pub fn start_game(&mut self) -> () {
        println!("Welcome to TicRustToe");
        println!("starting the game");
        println!();

        loop {
            self.print_state();
            println!();
            self.print_players_turn_message();

            let input = self.read_move_from_input();
            println!();

            match input {
                Err(_error) => continue,
                Ok(pos) => {
                    if self.game.conquer_field(&pos) {
                        self.print_game_won();
                        break;
                    }
                },
            }
        }
    }

    fn print_state(&self) -> () {
        self.game.print();
    }

    fn read_move_from_input(&self) -> Result<BoardPosition, GameErrors> {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split(" ").collect();

        if parts.len() != 2 {
            self.print_invalid_input_message();
            return Err(GameErrors::InvalidInput);
        }

        let x = parts[0].parse().unwrap();
        let y = parts[1].parse().unwrap();

        if x > 4 || y > 4 {
            self.print_input_out_of_gameboard_message();
            return Err(GameErrors::OutOfGameboard);
        }


        let pos = BoardPosition{x, y};
        if self.game.get_field(&pos) != FieldValue::Nothing {
            self.print_field_already_occupied_message();
            return Err(GameErrors::FieldArleadyOccupied);
        }
        return Ok(pos)
    }

    fn print_players_turn_message(&self) -> () {
        let player_mark = self.game.get_player_mark();

        println!("Player {} turn", player_mark);
        println!("Enter position in format 'x y'.")
    }

    fn print_invalid_input_message(&self) -> () {
        println!("Invalid input.");
    }

    fn print_input_out_of_gameboard_message(&self) -> () {
        println!("Position outside of the gameboard.");
    }

    fn print_field_already_occupied_message(&self) -> () {
        println!("Field is already occupied.");
    }

    fn print_game_won(&self) -> () {
        println!("You have won the game!")
    }
}
