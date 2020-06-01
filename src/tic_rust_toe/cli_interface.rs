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
    const DEFAULT_BOARD_SIZE: usize = 5;
    const DEFAULT_POINTS_TARGET: u32 = 3;

    pub fn new() -> CliInterface {
        CliInterface {
            game: Game::new(CliInterface::DEFAULT_BOARD_SIZE, CliInterface::DEFAULT_POINTS_TARGET) // some dummy game
        }
    }

    pub fn start_game(&mut self) -> () {
        println!("Welcome to TicRustToe");
        println!("starting the game");
        println!();

        let (board_size, points_target) = self.read_game_settings(CliInterface::DEFAULT_BOARD_SIZE, CliInterface::DEFAULT_POINTS_TARGET);
        self.game = Game::new(board_size, points_target);

        loop {
            self.print_state();
            println!();
            self.print_players_turn_message();

            let input = self.read_move_from_input();
            println!();

            match input {
                Err(_error) => continue,
                Ok(pos) => {
                    let player_mark = self.game.get_player_mark();
                    if self.game.conquer_field(&pos).unwrap() {
                        self.print_state();
                        self.print_game_won(player_mark);
                        break;
                    }
                },
            }
        }
    }

    fn print_state(&self) -> () {
        self.game.print();
    }

    fn read_game_settings(&self, default_board_size: usize, default_points_target: u32) -> (usize, u32) {
        let mut board_size: usize = 0;
        let mut points_target: u32 = 0;

        while board_size == 0 {
            println!("Enter board size (default: {})", default_board_size);
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let clean_input = input.trim();
            if clean_input.trim() == "" {
                board_size = default_board_size;
                continue;
            }

            match clean_input.parse() {
                Ok(item) => {
                    if item > 0 {
                        board_size = item;
                    } else {
                        self.print_invalid_settings_number();
                    }
                },
                _ => self.print_invalid_settings_number(),
            }
        }

        while points_target == 0 {
            println!("Enter target winning point (default: {})", default_points_target);
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let clean_input = input.trim();
            if clean_input.trim() == "" {
                points_target = default_points_target;
                continue;
            }

            match clean_input.parse() {
                Ok(item) => {
                    if item > 0 {
                        points_target = item;
                    } else {
                        self.print_invalid_settings_number();
                    }
                }
                _ => self.print_invalid_settings_number(),
            }
        }

        return (board_size, points_target);
    }

    fn read_move_from_input(&self) -> Result<BoardPosition, GameErrors> {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split(" ").collect();

        if parts.len() != 2 {
            self.print_invalid_input_message();
            return Err(GameErrors::InvalidInput);
        }

        let board_size = self.game.get_board_size();
        let x = match parts[0].parse() {
            Ok(item) => item,
            _ => {
                self.print_invalid_input_message();
                return Err(GameErrors::InvalidInput);
            }
        };
        let y = match parts[1].parse() {
            Ok(item) => item,
            _ => {
                self.print_invalid_input_message();
                return Err(GameErrors::InvalidInput);
            }
        };

        if x >= board_size || y >= board_size {
            self.print_input_out_of_gameboard_message();
            return Err(GameErrors::OutOfGameboard);
        }

        let pos = BoardPosition{x, y};
        if self.game.get_field(&pos).unwrap() != FieldValue::Nothing {
            self.print_field_already_occupied_message();
            return Err(GameErrors::FieldArleadyOccupied);
        }
        return Ok(pos)
    }

    fn print_invalid_settings_number(&self) -> () {
        println!("Invalid input. Please enter a number greater than zero.");
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

    fn print_game_won(&self, player_mark: FieldValue) -> () {
        println!("Player {} have won the game!", player_mark)
    }
}
