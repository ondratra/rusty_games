use speculate::speculate;

use rusty_games::tic_rust_toe::board::{Board, BoardPosition, FieldValue};
use rusty_games::tic_rust_toe::game::{Game};


speculate! {
    use assert_cmd::Command;

    describe "board" {
        it "is empty by default" {
            let board_size = 5;
            let board = Board::new(board_size);

            for y in 0..board_size {
                for x in 0..board_size {
                    assert!(board.get_field(&BoardPosition{x, y}).unwrap() == FieldValue::Nothing);
                }
            }
        }

        it "can save and recover field in gameboard" {
            let board_size = 5;
            let mut board = Board::new(board_size);
            let pos = BoardPosition{
                x: 2,
                y: 4
            };

            let field_value = FieldValue::Xmark;

            board.set_field(&pos, field_value).unwrap();
            assert!(board.get_field(&pos).unwrap() == field_value);
        }

        #[should_panic(expected = "Result::unwrap()` on an `Err` value: ()")]
        it "throws on out of gameboard read" {
            let board_size = 5;
            let board = Board::new(board_size);

            let pos = BoardPosition{
                x: board_size * 2,
                y: board_size * 3
            };

            board.get_field(&pos).unwrap();
        }

        #[should_panic(expected = "Result::unwrap()` on an `Err` value: ()")]
        it "throws on out of gameboard write" {
            let board_size = 5;
            let mut board = Board::new(board_size);

            let pos = BoardPosition{
                x: board_size * 2,
                y: board_size * 3
            };
            let field_value = FieldValue::Xmark;

            board.set_field(&pos, field_value).unwrap();
        }
    }

    describe "game" {
        it "can retrieve current player's mark" {
            let board_size = 5;
            let points_target = 3;
            let game = Game::new(board_size, points_target);

            assert!(game.get_player_mark() == FieldValue::Xmark)
        }

        it "can be won" {
            let board_size = 5;
            let points_target = 3;
            let mut game = Game::new(board_size, points_target);

            game.conquer_field(&BoardPosition{x: 0, y: 0}).unwrap(); // X's turn
            game.conquer_field(&BoardPosition{x: 1, y: 0}).unwrap(); // O's turn
            game.conquer_field(&BoardPosition{x: 1, y: 1}).unwrap(); // X's turn
            game.conquer_field(&BoardPosition{x: 0, y: 1}).unwrap(); // O's turn
            let game_won = game.conquer_field(&BoardPosition{x: 2, y: 2}).unwrap(); // X's turn

            assert!(game_won);
        }
    }

    describe "cli interface" {
        it "can be played and won" {
            let game_input = "
                \n
                \n
                0 0\n
                1 0\n
                1 1\n
                0 1\n
                2 2\n
            ";

            println!("DDDDDDdebuuug");
            let output = Command::new("cargo")
                .arg("run")
                .write_stdin(game_input)
                .output()
                .unwrap();

            let string_output: String = String::from_utf8(output.stdout).unwrap();
            println!("DDDDDDdebuuug");
            println!("{}", string_output);
        }
    }
}
