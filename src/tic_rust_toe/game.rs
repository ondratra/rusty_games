use std::sync::mpsc;
use std::thread;

use super::board::{Board, FieldValue, BoardPosition};


#[derive(Clone)]
pub struct Game {
    first_player_turn: bool,
    points_target: u32,
    board: Board
}

impl Game {
    pub fn new(game_board_size: usize, points_target: u32) -> Game {
        Game {
            first_player_turn: true,
            points_target: points_target,
            board: Board::new(game_board_size)
        }
    }

    pub fn get_player_mark(&self) -> FieldValue {
        if self.first_player_turn {
            return FieldValue::Xmark
        }

        return FieldValue::Circle
    }

    pub fn conquer_field(&mut self, pos: &BoardPosition) -> bool {
        let player_mark = self.get_player_mark();
        self.first_player_turn = !self.first_player_turn;

        self.board.set_field(pos, player_mark);

        return self.was_winning_move(pos, player_mark);
    }

    // this algorithm is unnecessarily running in parallel - let's try Rust's threads
    fn was_winning_move(&self, pos: &BoardPosition, player_mark: FieldValue) -> bool {
        const ALL_DIRECTIONS: [(i8, i8); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
        ];

        let (tx, rx) = mpsc::channel::<()>();
        let threads: Vec<_> = ALL_DIRECTIONS.iter().map(move |item| {
            let thread_pos = pos.clone();
            let thread_self = self.clone();
            let thread_tx = tx.clone();

            return thread::spawn(move || {
                let points = thread_self.count_direction_points(&thread_pos, player_mark, &item);
                if points >= thread_self.points_target {
                    thread_tx.send(()).unwrap();
                }
            });
        }).collect();

        for item in threads {
            item.join().unwrap();
        }

        let is_winning = match rx.try_recv() {
            Ok(()) => true,
            Err(_error) => false,
        };

        return is_winning;
    }

    fn count_direction_points(&self, pos: &BoardPosition, player_mark: FieldValue, direction: &(i8, i8)) -> u32 {
        let mut current_pos = *pos;
        let mut points = 1;

        loop {
            current_pos = match self.board.get_neighbour_field(&current_pos, direction) {
                Ok(tmp_pos) => *Box::new(tmp_pos),
                Err(_) => break,
            };
            if self.board.get_field(&current_pos) != player_mark {
                break;
            }

            points += 1;
        }

        return points;
    }

    pub fn get_field(&self, pos: &BoardPosition) -> FieldValue {
        self.board.get_field(pos)
    }

    pub fn print(&self) -> () {
        self.board.print()
    }
}
