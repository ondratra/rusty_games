use std::fmt;


#[derive(Clone, Copy, PartialEq)]
pub enum FieldValue {
    Nothing,
    Circle,
    Xmark
}

impl fmt::Display for FieldValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldValue::Nothing => write!(f, "."),
            FieldValue::Circle => write!(f, "O"),
            FieldValue::Xmark => write!(f, "X"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct BoardPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct Board {
    fields: Vec<Vec<FieldValue>>,
    pub board_size: usize,
}

impl Board {
    pub fn new(game_board_size: usize) -> Board {
        Board {
            fields: vec![vec![FieldValue::Nothing; game_board_size]; game_board_size],
            board_size: game_board_size,
        }
    }

    pub fn set_field(&mut self, pos: &BoardPosition, value: FieldValue) -> Result<(), ()> {
        if self.is_out_of_board(pos) {
            return Err(());
        }

        self.fields[pos.y][pos.x] = value;

        return Ok(());
    }

    pub fn get_field(&self, pos: &BoardPosition) -> Result<FieldValue, ()> {
        if self.is_out_of_board(pos) {
            return Err(());
        }

        return Ok(self.fields[pos.y][pos.x]);
    }

    fn is_out_of_board(&self, pos: &BoardPosition) -> bool {
        return pos.x >= self.board_size || pos.y >= self.board_size;
    }

    pub fn get_neighbour_field(&self, pos: &BoardPosition, direction: &(i8, i8)) -> Result<BoardPosition, ()> {
        if pos.x == 0 && direction.0 < 0 {
            return Err(());
        }

        if pos.x == self.board_size - 1 && direction.0 > 0 {
            return Err(());
        }

        if pos.y == 0 && direction.1 < 0 {
            return Err(());
        }

        if pos.y == self.board_size - 1 && direction.1 > 0 {
            return Err(());
        }

        let neighbour_pos = BoardPosition {
            x: match direction.0 {
                i if i < 0 => pos.x - 1,
                i if i > 0 => pos.x + 1,
                _ => pos.x,
            },
            y: match direction.1 {
                i if i < 0 => pos.y - 1,
                i if i > 0 => pos.y + 1,
                _ => pos.y,
            },
        };

        return Ok(neighbour_pos);
    }

    pub fn print(&self) -> () {
        for i in 0..self.fields.len() {
            for o in 0..self.fields[i].len() {
                print!("{}", self.fields[i][o]);
            }
            println!()
        }
    }
}
