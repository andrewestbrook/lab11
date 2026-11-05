// Board rules. Week 11: you edit parse_coord and place in this file (Tasks 1 and 2).
use crate::data::ShipSpec;
use rand::RngExt;
use rand::rngs::StdRng;
use std::fmt;

pub const SIZE: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Water,
    Ship,
    Hit,
    Miss,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shot {
    Miss,
    Hit,
    AlreadyShot,
}

// board[row][col]. Row 0 is the top row (shown as 1); column 0 is A.
pub type Board = [[Cell; SIZE]; SIZE];

pub fn new_board() -> Board {
    [[Cell::Water; SIZE]; SIZE]
}

// Why a typed coordinate was rejected.
#[derive(Debug, Clone, PartialEq)]
pub enum CoordError {
    BadFormat,
    ColumnOutOfRange(char),
    RowOutOfRange(usize),
}

// One sentence per variant, never the debug form.
impl fmt::Display for CoordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoordError::BadFormat => write!(f, "type a column letter and a row number, like B7"),
            CoordError::ColumnOutOfRange(c) => write!(f, "column {c} is off the board (A-J)"),
            CoordError::RowOutOfRange(n) => write!(f, "row {n} is off the board (1-10)"),
        }
    }
}

// Why a ship could not be placed.
#[derive(Debug, Clone, PartialEq)]
pub enum PlaceError {
    OutOfBounds,
    Overlap,
}

// Why a whole fleet could not be placed.
#[derive(Debug, Clone, PartialEq)]
pub enum FleetError {
    NoRoom(String),
}

impl fmt::Display for FleetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FleetError::NoRoom(name) => write!(f, "no room on the board for the {name}"),
        }
    }
}

// Place a ship starting at (row, col).
// Checks the edge first (OutOfBounds), then overlap (Overlap), and only then
// changes the board, so a failed call leaves the board untouched.
pub fn place(
    board: &mut Board,
    row: usize,
    col: usize,
    length: usize,
    horizontal: bool,
) -> Result<(), PlaceError> {
    if row >= SIZE || col >= SIZE {
        return Err(PlaceError::OutOfBounds);
    }
    if horizontal && col + length > SIZE {
        return Err(PlaceError::OutOfBounds);
    }
    if !horizontal && row + length > SIZE {
        return Err(PlaceError::OutOfBounds);
    }
    for i in 0..length {
        let (r, c) = if horizontal {
            (row, col + i)
        } else {
            (row + i, col)
        };
        if board[r][c] != Cell::Water {
            return Err(PlaceError::Overlap);
        }
    }
    for i in 0..length {
        if horizontal {
            board[row][col + i] = Cell::Ship;
        } else {
            board[row + i][col] = Cell::Ship;
        }
    }
    Ok(())
}

// PROVIDED: place every ship at a random legal position. Err names the first ship that will not fit.
pub fn place_fleet(
    board: &mut Board,
    fleet: &[ShipSpec],
    rng: &mut StdRng,
) -> Result<(), FleetError> {
    for spec in fleet {
        let mut placed = false;
        for _ in 0..1000 {
            let row = rng.random_range(0..SIZE);
            let col = rng.random_range(0..SIZE);
            let horizontal = rng.random_bool(0.5);
            if place(board, row, col, spec.length, horizontal).is_ok() {
                placed = true;
                break;
            }
        }
        if !placed {
            return Err(FleetError::NoRoom(spec.name.clone()));
        }
    }
    Ok(())
}

// Fire at (row, col). Panics if the position is off the board.
pub fn fire(board: &mut Board, row: usize, col: usize) -> Shot {
    match board[row][col] {
        Cell::Water => {
            board[row][col] = Cell::Miss;
            Shot::Miss
        }
        Cell::Ship => {
            board[row][col] = Cell::Hit;
            Shot::Hit
        }
        Cell::Hit | Cell::Miss => Shot::AlreadyShot,
    }
}

pub fn count_cells(board: &Board, kind: Cell) -> usize {
    let mut total = 0;
    for row in 0..SIZE {
        for col in 0..SIZE {
            if board[row][col] == kind {
                total += 1;
            }
        }
    }
    total
}

pub fn all_sunk(board: &Board) -> bool {
    count_cells(board, Cell::Ship) == 0
}

// "B7" -> Ok((6, 1)): letter is the column, number is the row (1-based).
// BadFormat for text that is not a letter then a number, ColumnOutOfRange for a
// column past J (uppercase letter), RowOutOfRange for a row of 0 or past 10.
// The column is checked before the row.
pub fn parse_coord(text: &str) -> Result<(usize, usize), CoordError> {
    let text = text.trim();
    let first = match text.chars().next() {
        Some(c) => c,
        None => return Err(CoordError::BadFormat),
    };
    if !first.is_ascii_alphabetic() {
        return Err(CoordError::BadFormat);
    }
    let row_number = match text[first.len_utf8()..].trim().parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err(CoordError::BadFormat),
    };
    let letter = first.to_ascii_uppercase();
    let col = (letter as usize) - ('A' as usize);
    if col >= SIZE {
        return Err(CoordError::ColumnOutOfRange(letter));
    }
    if row_number < 1 || row_number > SIZE {
        return Err(CoordError::RowOutOfRange(row_number));
    }
    Ok((row_number - 1, col))
}

// One character per cell. Ships are hidden unless `reveal` is true.
pub fn render(board: &Board, reveal: bool) -> String {
    let mut out = String::from("    A B C D E F G H I J\n");
    for row in 0..SIZE {
        out.push_str(&format!("{:>2}  ", row + 1));
        for col in 0..SIZE {
            let symbol = match board[row][col] {
                Cell::Water => '.',
                Cell::Ship => {
                    if reveal {
                        'S'
                    } else {
                        '.'
                    }
                }
                Cell::Hit => 'X',
                Cell::Miss => 'o',
            };
            out.push(symbol);
            if col + 1 < SIZE {
                out.push(' ');
            }
        }
        out.push('\n');
    }
    out
}

// Higher is better. Fewer shots score more; 0 if every cell was fired at.
pub fn score_for(shots: u32) -> u32 {
    let most = (SIZE * SIZE) as u32;
    if shots >= most { 0 } else { most - shots }
}
