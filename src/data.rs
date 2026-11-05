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

// One sentence per variant, written so main can print
// "data/fleet.txt line 10: <sentence>".
impl fmt::Display for ShipLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShipLineError::MissingComma => {
                write!(f, "needs exactly one comma, like Carrier,5")
            }
            ShipLineError::EmptyName => write!(f, "has no ship name before the comma"),
            ShipLineError::BadLength(text) => {
                write!(f, "has length \"{text}\", which is not a whole number")
            }
            ShipLineError::LengthOutOfRange(n) => {
                write!(f, "has length {n}, outside {MIN_SHIP}-{MAX_SHIP}")
            }
        }
    }
}

// Why the text of a board file was rejected. Line numbers are 1-based file lines.
#[derive(Debug, Clone, PartialEq)]
pub enum BoardError {
    WrongRowCount(usize),
    WrongRowWidth { line: usize, found: usize },
    BadChar { line: usize, ch: char },
}

// One sentence per variant, with the line number and the character or counts.
impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardError::WrongRowCount(n) => {
                write!(f, "found {n} rows, but a board needs exactly {SIZE}")
            }
            BoardError::WrongRowWidth { line, found } => write!(
                f,
                "line {line} has {found} characters, but a row needs exactly {SIZE}"
            ),
            BoardError::BadChar { line, ch } => write!(
                f,
                "line {line} contains '{ch}', but only '.' and 'S' are allowed"
            ),
        }
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
// MissingComma (not exactly one comma), EmptyName, BadLength(the text that did
// not parse), or LengthOutOfRange(the number).
pub fn parse_ship_line(line: &str) -> Result<ShipSpec, ShipLineError> {
    let parts: Vec<&str> = line.trim().split(',').collect();
    if parts.len() != 2 {
        return Err(ShipLineError::MissingComma);
    }
    let name = parts[0].trim();
    if name.is_empty() {
        return Err(ShipLineError::EmptyName);
    }
    let length_text = parts[1].trim();
    let length = match length_text.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err(ShipLineError::BadLength(length_text.to_string())),
    };
    if length < MIN_SHIP || length > MAX_SHIP {
        return Err(ShipLineError::LengthOutOfRange(length));
    }
    Ok(ShipSpec {
        name: name.to_string(),
        length,
    })
}

// Read a fleet file. An unreadable file is Err(DataError::Io(..)); bad lines go
// into `skipped` as (1-based line number, error); zero valid ships is
// Err(DataError::NoShips(path)). Blank and '#' lines are skipped silently but
// still count for line numbers. Prints nothing: main reports skipped lines.
pub fn load_fleet(path: &str) -> Result<FleetReport, DataError> {
    let text = fs::read_to_string(path)?;
    let mut fleet = Vec::new();
    let mut skipped = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match parse_ship_line(line) {
            Ok(ship) => fleet.push(ship),
            Err(e) => skipped.push((line_number, e)),
        }
    }
    if fleet.is_empty() {
        return Err(DataError::NoShips(path.to_string()));
    }
    Ok(FleetReport { fleet, skipped })
}

// Turn the text of a board file into a Board.
// WrongRowWidth { line, found } for a row that is not SIZE characters wide,
// BadChar { line, ch } for any character other than '.' and 'S', and
// WrongRowCount(n) when there are not exactly SIZE rows (n = rows found).
// `line` is the 1-based file line, comments and blank lines included.
pub fn parse_board(text: &str) -> Result<Board, BoardError> {
    let mut board = new_board();
    let mut row = 0;
    for (index, line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let chars: Vec<char> = line.chars().collect();
        if chars.len() != SIZE {
            return Err(BoardError::WrongRowWidth {
                line: line_number,
                found: chars.len(),
            });
        }
        for col in 0..SIZE {
            let cell = match chars[col] {
                '.' => Cell::Water,
                'S' => Cell::Ship,
                ch => {
                    return Err(BoardError::BadChar {
                        line: line_number,
                        ch,
                    });
                }
            };
            if row < SIZE {
                board[row][col] = cell;
            }
        }
        row += 1;
    }
    if row != SIZE {
        return Err(BoardError::WrongRowCount(row));
    }
    Ok(board)
}

// Read a board file. The From impls on DataError let `?` convert both errors.
pub fn load_board(path: &str) -> Result<Board, DataError> {
    let text = fs::read_to_string(path)?;
    let board = parse_board(&text)?;
    Ok(board)
}

// A missing file is normal on the first run: Ok with an empty Vec.
// Any other read error is Err(DataError::Io(e)), so `?` cannot be used here.
// Lines that parse_score rejects are skipped.
pub fn load_scores(path: &str) -> Result<Vec<ScoreRecord>, DataError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(DataError::Io(e)),
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
pub fn append_score(path: &str, record: &ScoreRecord) -> Result<(), DataError> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    let name = record.name.replace(',', " ");
    writeln!(file, "{},{}", name, record.score)?;
    Ok(())
}
