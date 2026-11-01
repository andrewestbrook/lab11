// Loading and saving game data. Week 11: every failure gets a name (Tasks 3 to 8).
use crate::board::{Board, Cell, SIZE, new_board};
use std::fmt;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

pub const MIN_SHIP: usize = 2;
pub const MAX_SHIP: usize = 5;

#[derive(Debug, Clone, PartialEq)]
pub struct ShipSpec {
    pub name: String,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScoreRecord {
    pub name: String,
    pub score: u32,
}

// ---- Error types ----

// Why one line of fleet.txt was rejected.
#[derive(Debug, Clone, PartialEq)]
pub enum ShipLineError {
    MissingComma,
    EmptyName,
    BadLength(String),
    LengthOutOfRange(usize),
}

// TODO (Task 3): one sentence per variant, written so main can print
// "data/fleet.txt line 10: <your sentence>". Example: "has length 1, outside 2-5".
// Use MIN_SHIP and MAX_SHIP in the message instead of typing 2 and 5.
impl fmt::Display for ShipLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: {self:?}")
    }
}

// Why the text of a board file was rejected. Line numbers are 1-based file lines.
#[derive(Debug, Clone, PartialEq)]
pub enum BoardError {
    WrongRowCount(usize),
    WrongRowWidth { line: usize, found: usize },
    BadChar { line: usize, ch: char },
}

// TODO (Task 5): one sentence per variant. Include the line number and the
// character or counts the variant carries, and use SIZE instead of typing 10.
impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: {self:?}")
    }
}

// Why a data file could not be used. PROVIDED: study this as a template.
#[derive(Debug)]
pub enum DataError {
    Io(io::Error),
    Board(BoardError),
    NoShips(String),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::Io(e) => write!(f, "{e}"),
            DataError::Board(e) => write!(f, "bad board layout: {e}"),
            DataError::NoShips(path) => write!(f, "no valid ships in {path}"),
        }
    }
}

// These two impls are what let `?` convert an error into a DataError.
impl From<io::Error> for DataError {
    fn from(e: io::Error) -> DataError {
        DataError::Io(e)
    }
}

impl From<BoardError> for DataError {
    fn from(e: BoardError) -> DataError {
        DataError::Board(e)
    }
}

// The fleet plus every line that was skipped: (1-based line number, why).
#[derive(Debug)]
pub struct FleetReport {
    pub fleet: Vec<ShipSpec>,
    pub skipped: Vec<(usize, ShipLineError)>,
}

// WORKED EXAMPLE (provided). One line of scores.txt looks like:  Ana,83
// Returns None for any line that is not "name,number".
pub fn parse_score(line: &str) -> Option<ScoreRecord> {
    let parts: Vec<&str> = line.trim().split(',').collect();
    if parts.len() != 2 {
        return None;
    }
    let name = parts[0].trim();
    if name.is_empty() {
        return None;
    }
    match parts[1].trim().parse::<u32>() {
        Ok(score) => Some(ScoreRecord {
            name: name.to_string(),
            score,
        }),
        Err(_) => None,
    }
}

// PROVIDED: sort highest score first (selection sort).
pub fn sort_scores_desc(records: &mut Vec<ScoreRecord>) {
    for i in 0..records.len() {
        let mut best = i;
        for j in (i + 1)..records.len() {
            if records[j].score > records[best].score {
                best = j;
            }
        }
        records.swap(i, best);
    }
}

// One line of fleet.txt looks like:  Carrier,5
// TODO (Task 3): every failure is MissingComma right now. Return the variant
// that matches the problem: MissingComma (not exactly one comma), EmptyName,
// BadLength(the text that did not parse), LengthOutOfRange(the number).
pub fn parse_ship_line(line: &str) -> Result<ShipSpec, ShipLineError> {
    let parts: Vec<&str> = line.trim().split(',').collect();
    if parts.len() != 2 {
        return Err(ShipLineError::MissingComma);
    }
    let name = parts[0].trim();
    if name.is_empty() {
        return Err(ShipLineError::MissingComma);
    }
    let length = match parts[1].trim().parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err(ShipLineError::MissingComma),
    };
    if length < MIN_SHIP || length > MAX_SHIP {
        return Err(ShipLineError::MissingComma);
    }
    Ok(ShipSpec {
        name: name.to_string(),
        length,
    })
}

// TODO (Task 4): this is still the Week 10 behavior wrapped in Ok. Change it so:
//   - an unreadable file is Err(DataError::Io(..)): read with `?`, no match;
//   - bad lines go into `skipped` as (1-based line number, error), with no
//     eprintln! (main reports them);
//   - a file with zero valid ships is Err(DataError::NoShips(path.to_string())).
// Blank lines and '#' lines are skipped silently but still count for line numbers.
pub fn load_fleet(path: &str) -> Result<FleetReport, DataError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Could not read {path}: {e}");
            return Ok(FleetReport {
                fleet: Vec::new(),
                skipped: Vec::new(),
            });
        }
    };
    let mut fleet = Vec::new();
    let mut skipped_count = 0;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match parse_ship_line(line) {
            Ok(ship) => fleet.push(ship),
            Err(_) => skipped_count += 1,
        }
    }
    if skipped_count > 0 {
        eprintln!("Skipped {skipped_count} invalid line(s) in {path}");
    }
    Ok(FleetReport {
        fleet,
        skipped: Vec::new(),
    })
}

// Turn the text of a board file into a Board.
// TODO (Task 5): every failure is WrongRowCount right now. Return
// WrongRowWidth { line, found } for a row that is not SIZE characters wide, and
// BadChar { line, ch } for any character other than '.' and 'S'. `line` is the
// 1-based line number in the file, comments and blank lines included
// (enumerate() starts at 0). WrongRowCount(n) carries how many rows were found.
pub fn parse_board(text: &str) -> Result<Board, BoardError> {
    let mut board = new_board();
    let mut row = 0;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if row >= SIZE {
            return Err(BoardError::WrongRowCount(row));
        }
        let chars: Vec<char> = line.chars().collect();
        if chars.len() != SIZE {
            return Err(BoardError::WrongRowCount(row));
        }
        for col in 0..SIZE {
            match chars[col] {
                '.' => board[row][col] = Cell::Water,
                'S' => board[row][col] = Cell::Ship,
                _ => return Err(BoardError::WrongRowCount(row)),
            }
        }
        row += 1;
    }
    if row != SIZE {
        return Err(BoardError::WrongRowCount(row));
    }
    Ok(board)
}

// Read a board file.
// TODO (Task 6): this works, but it is long. Rewrite the body as three lines
// that use `?` twice, with no match: the From impls above do the conversion.
pub fn load_board(path: &str) -> Result<Board, DataError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return Err(DataError::Io(e)),
    };
    match parse_board(&text) {
        Ok(board) => Ok(board),
        Err(e) => Err(DataError::Board(e)),
    }
}

// A missing file is normal on the first run: Ok with an empty Vec.
// TODO (Task 7): right now every read error is treated as "missing". Return
// Ok(empty) only when e.kind() == io::ErrorKind::NotFound. Any other error is
// Err(DataError::Io(e)). Keep skipping score lines that parse_score rejects.
pub fn load_scores(path: &str) -> Result<Vec<ScoreRecord>, DataError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(_) => return Ok(Vec::new()),
    };
    let mut records = Vec::new();
    for line in text.lines() {
        if let Some(record) = parse_score(line) {
            records.push(record);
        }
    }
    Ok(records)
}

// Append "name,score" as one line, creating the file if needed.
// Commas in the name are replaced with spaces so the line stays parseable.
// TODO (Task 8): this works, but it is long. Rewrite it with `?` on the open and
// on the writeln!, with no match.
pub fn append_score(path: &str, record: &ScoreRecord) -> Result<(), DataError> {
    let mut file = match OpenOptions::new().create(true).append(true).open(path) {
        Ok(file) => file,
        Err(e) => return Err(DataError::Io(e)),
    };
    let name = record.name.replace(',', " ");
    match writeln!(file, "{},{}", name, record.score) {
        Ok(()) => Ok(()),
        Err(e) => Err(DataError::Io(e)),
    }
}
