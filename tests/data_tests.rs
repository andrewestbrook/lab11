use battleship::board::{Cell, SIZE};
use battleship::data::{
    BoardError, DataError, ScoreRecord, ShipLineError, ShipSpec, append_score, load_board,
    load_fleet, load_scores, parse_board, parse_score, parse_ship_line,
};
use std::io;

// A valid board as text. Row 0 starts with two ship cells; row 1 has one at column 2.
const GOOD_BOARD: &str = "SS........\n\
                          ..S.......\n\
                          ..........\n\
                          ..........\n\
                          ..........\n\
                          ..........\n\
                          ..........\n\
                          ..........\n\
                          ..........\n\
                          .........S\n";

// ---- PROVIDED tests: parse_ship_line ----

#[test]
fn parse_ship_line_valid() {
    assert_eq!(
        parse_ship_line("Carrier,5"),
        Ok(ShipSpec {
            name: "Carrier".to_string(),
            length: 5
        })
    );
}

#[test]
fn parse_ship_line_trims_spaces() {
    assert_eq!(
        parse_ship_line("  Cruiser , 3 "),
        Ok(ShipSpec {
            name: "Cruiser".to_string(),
            length: 3
        })
    );
}

#[test]
fn parse_ship_line_missing_comma() {
    assert_eq!(parse_ship_line("Flagship"), Err(ShipLineError::MissingComma));
}

#[test]
fn parse_ship_line_length_one() {
    assert_eq!(
        parse_ship_line("Tugboat,1"),
        Err(ShipLineError::LengthOutOfRange(1))
    );
}

#[test]
fn ship_line_messages_are_sentences() {
    let too_short = ShipLineError::LengthOutOfRange(1).to_string();
    assert!(too_short.contains('1') && too_short.contains("2-5"));
    for error in [
        ShipLineError::MissingComma,
        ShipLineError::EmptyName,
        ShipLineError::BadLength("x".to_string()),
        ShipLineError::LengthOutOfRange(6),
    ] {
        let text = error.to_string();
        assert!(!text.is_empty());
        assert!(!text.contains("TODO"), "message still a stub: {text}");
        assert!(!text.contains("OutOfRange"), "debug form leaked: {text}");
        assert!(!text.contains("MissingComma"), "debug form leaked: {text}");
        assert!(!text.contains("EmptyName"), "debug form leaked: {text}");
        assert!(!text.contains("BadLength"), "debug form leaked: {text}");
    }
}

// ---- PROVIDED tests: load_fleet ----

#[test]
fn load_fleet_reports_good_ships_and_skipped_lines() {
    let report = load_fleet("data/fleet.txt").expect("data/fleet.txt should load");
    assert_eq!(report.fleet.len(), 5);
    assert_eq!(report.fleet[0].name, "Carrier");
    assert_eq!(
        report.skipped,
        vec![
            (10, ShipLineError::LengthOutOfRange(1)),
            (11, ShipLineError::BadLength("x".to_string())),
            (12, ShipLineError::MissingComma),
        ]
    );
}

#[test]
fn load_fleet_missing_file_is_io_not_found() {
    match load_fleet("definitely/not/here.txt") {
        Err(DataError::Io(e)) => assert_eq!(e.kind(), io::ErrorKind::NotFound),
        other => panic!("expected Err(Io(NotFound)), got {other:?}"),
    }
}

#[test]
fn load_fleet_directory_is_io_error() {
    assert!(matches!(load_fleet("data"), Err(DataError::Io(_))));
}

#[test]
fn load_fleet_comments_only_is_no_ships() {
    match load_fleet("data/fleet_empty.txt") {
        Err(DataError::NoShips(path)) => assert_eq!(path, "data/fleet_empty.txt"),
        other => panic!("expected Err(NoShips), got {other:?}"),
    }
}

#[test]
fn load_fleet_all_bad_is_no_ships() {
    assert!(matches!(
        load_fleet("data/fleet_all_bad.txt"),
        Err(DataError::NoShips(_))
    ));
}

// ---- PROVIDED tests: parse_board and load_board ----

#[test]
fn parse_board_valid() {
    let board = parse_board(GOOD_BOARD).expect("GOOD_BOARD should parse");
    assert_eq!(board[0][0], Cell::Ship);
    assert_eq!(board[0][1], Cell::Ship);
    assert_eq!(board[0][2], Cell::Water);
    assert_eq!(board[1][2], Cell::Ship);
    assert_eq!(board[SIZE - 1][SIZE - 1], Cell::Ship);
}

#[test]
fn parse_board_nine_rows() {
    let nine_rows = GOOD_BOARD.replacen("..........\n", "", 1);
    assert_eq!(parse_board(&nine_rows), Err(BoardError::WrongRowCount(9)));
}

#[test]
fn parse_board_eleven_rows() {
    let eleven_rows = format!("{GOOD_BOARD}..........\n");
    assert_eq!(parse_board(&eleven_rows), Err(BoardError::WrongRowCount(11)));
}

#[test]
fn parse_board_bad_character() {
    let bad = GOOD_BOARD.replacen("SS........", "SS...x....", 1);
    assert_eq!(
        parse_board(&bad),
        Err(BoardError::BadChar { line: 1, ch: 'x' })
    );
}

#[test]
fn board_messages_are_sentences() {
    for error in [
        BoardError::WrongRowCount(9),
        BoardError::WrongRowWidth { line: 3, found: 9 },
        BoardError::BadChar { line: 6, ch: 'x' },
    ] {
        let text = error.to_string();
        assert!(!text.contains("TODO"), "message still a stub: {text}");
        assert!(!text.contains("WrongRow"), "debug form leaked: {text}");
        assert!(!text.contains("BadChar"), "debug form leaked: {text}");
    }
    assert!(BoardError::BadChar { line: 6, ch: 'x' }.to_string().contains("line 6"));
}

#[test]
fn load_board_reads_the_practice_file() {
    let board = load_board("data/boards/practice.txt").expect("practice board should load");
    let ships = board.iter().flatten().filter(|c| **c == Cell::Ship).count();
    assert_eq!(ships, 16);
}

#[test]
fn load_board_reports_nine_rows() {
    match load_board("data/boards/broken_rows.txt") {
        Err(DataError::Board(e)) => assert_eq!(e, BoardError::WrongRowCount(9)),
        other => panic!("expected Err(Board(WrongRowCount(9))), got {other:?}"),
    }
}

#[test]
fn load_board_reports_bad_character_with_file_line() {
    match load_board("data/boards/broken_char.txt") {
        Err(DataError::Board(e)) => assert_eq!(e, BoardError::BadChar { line: 6, ch: 'x' }),
        other => panic!("expected Err(Board(BadChar)), got {other:?}"),
    }
}

#[test]
fn load_board_missing_file_is_io() {
    assert!(matches!(
        load_board("definitely/not/here.txt"),
        Err(DataError::Io(_))
    ));
}

// ---- PROVIDED tests: From conversions (already passing) ----

#[test]
fn data_error_from_board_error() {
    let converted = DataError::from(BoardError::WrongRowCount(3));
    assert!(matches!(
        converted,
        DataError::Board(BoardError::WrongRowCount(3))
    ));
}

#[test]
fn data_error_from_io_error() {
    let converted = DataError::from(io::Error::new(io::ErrorKind::NotFound, "gone"));
    assert!(matches!(converted, DataError::Io(_)));
}

// ---- PROVIDED tests: scores ----

#[test]
fn parse_score_valid() {
    assert_eq!(
        parse_score("Ana,83"),
        Some(ScoreRecord {
            name: "Ana".to_string(),
            score: 83
        })
    );
}

#[test]
fn load_scores_missing_file_is_normal() {
    match load_scores("definitely/not/here.txt") {
        Ok(records) => assert!(records.is_empty()),
        Err(e) => panic!("a missing scores file must be Ok(empty), got Err({e})"),
    }
}

#[test]
fn load_scores_directory_is_an_error() {
    assert!(matches!(load_scores("data"), Err(DataError::Io(_))));
}

#[test]
fn append_score_to_a_directory_is_an_error() {
    let record = ScoreRecord {
        name: "Ana".to_string(),
        score: 83,
    };
    assert!(matches!(append_score("data", &record), Err(DataError::Io(_))));
}

// ---- STUDENT tests: write these three (Task 11) ----
// 1. parse_ship_line returns Err(ShipLineError::EmptyName) for ",5".
// 2. parse_ship_line returns Err(ShipLineError::BadLength("x".to_string())) for "Dinghy,x".
// 3. parse_board returns Err(BoardError::WrongRowWidth { line: 1, found: 9 }) when
//    the first row of GOOD_BOARD is one character short. Build the text with
//    GOOD_BOARD.replacen(...) as the bad-character test does.
