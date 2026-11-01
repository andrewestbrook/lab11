// PROVIDED: tests for the board rules. Some fail in the starter until you finish Tasks 1 and 2.
use battleship::board::{
    Board, Cell, CoordError, FleetError, PlaceError, SIZE, Shot, all_sunk, count_cells, fire,
    new_board, parse_coord, place, place_fleet, render, score_for,
};
use battleship::data::ShipSpec;
use rand::SeedableRng;
use rand::rngs::StdRng;

fn spec(name: &str, length: usize) -> ShipSpec {
    ShipSpec {
        name: name.to_string(),
        length,
    }
}

#[test]
fn new_board_is_all_water() {
    let board: Board = new_board();
    assert_eq!(count_cells(&board, Cell::Water), SIZE * SIZE);
}

// ---- Task 2: place ----

#[test]
fn place_accepts_ships_that_fit() {
    let mut a = new_board();
    let mut b = new_board();
    assert_eq!(place(&mut a, 0, 7, 3, true), Ok(()));
    assert_eq!(place(&mut b, 7, 0, 3, false), Ok(()));
    assert_eq!(count_cells(&a, Cell::Ship), 3);
    assert_eq!(count_cells(&b, Cell::Ship), 3);
}

#[test]
fn place_rejects_off_the_edge() {
    let mut board = new_board();
    assert_eq!(place(&mut board, 0, 8, 3, true), Err(PlaceError::OutOfBounds));
    assert_eq!(place(&mut board, 8, 0, 3, false), Err(PlaceError::OutOfBounds));
    assert_eq!(place(&mut board, 10, 0, 2, true), Err(PlaceError::OutOfBounds));
    assert_eq!(place(&mut board, 0, 10, 2, false), Err(PlaceError::OutOfBounds));
}

#[test]
fn place_rejects_overlap() {
    let mut board = new_board();
    assert_eq!(place(&mut board, 2, 2, 3, true), Ok(()));
    assert_eq!(place(&mut board, 0, 3, 4, false), Err(PlaceError::Overlap));
    assert_eq!(place(&mut board, 3, 2, 3, true), Ok(()));
}

#[test]
fn failed_place_leaves_the_board_unchanged() {
    let mut board = new_board();
    place(&mut board, 2, 2, 3, true).unwrap();
    let before = board;
    assert!(place(&mut board, 0, 3, 4, false).is_err());
    assert_eq!(board, before);
    assert!(place(&mut board, 0, 8, 3, true).is_err());
    assert_eq!(board, before);
}

#[test]
fn seeded_placement_is_repeatable_and_complete() {
    let fleet = vec![spec("Carrier", 5), spec("Cruiser", 3), spec("Destroyer", 2)];
    let mut a = new_board();
    let mut b = new_board();
    assert!(place_fleet(&mut a, &fleet, &mut StdRng::seed_from_u64(7)).is_ok());
    assert!(place_fleet(&mut b, &fleet, &mut StdRng::seed_from_u64(7)).is_ok());
    assert_eq!(a, b);
    assert_eq!(count_cells(&a, Cell::Ship), 10);
}

#[test]
fn place_fleet_names_the_ship_that_does_not_fit() {
    let fleet = vec![spec("Destroyer", 2), spec("Huge", 11)];
    let mut board = new_board();
    let result = place_fleet(&mut board, &fleet, &mut StdRng::seed_from_u64(1));
    assert_eq!(result, Err(FleetError::NoRoom("Huge".to_string())));
}

// ---- Task 1: parse_coord ----

#[test]
fn parse_coord_accepts_good_input() {
    assert_eq!(parse_coord("A1"), Ok((0, 0)));
    assert_eq!(parse_coord("b7"), Ok((6, 1)));
    assert_eq!(parse_coord(" J10 "), Ok((9, 9)));
}

#[test]
fn parse_coord_column_out_of_range() {
    assert_eq!(parse_coord("K1"), Err(CoordError::ColumnOutOfRange('K')));
    assert_eq!(parse_coord("k1"), Err(CoordError::ColumnOutOfRange('K')));
}

#[test]
fn parse_coord_row_out_of_range() {
    assert_eq!(parse_coord("A0"), Err(CoordError::RowOutOfRange(0)));
    assert_eq!(parse_coord("A11"), Err(CoordError::RowOutOfRange(11)));
}

#[test]
fn parse_coord_bad_format() {
    assert_eq!(parse_coord("7B"), Err(CoordError::BadFormat));
    assert_eq!(parse_coord("A"), Err(CoordError::BadFormat));
    assert_eq!(parse_coord(""), Err(CoordError::BadFormat));
    assert_eq!(parse_coord("A1x"), Err(CoordError::BadFormat));
    assert_eq!(parse_coord("!"), Err(CoordError::BadFormat));
}

#[test]
fn coord_error_messages_are_sentences() {
    assert_eq!(
        CoordError::BadFormat.to_string(),
        "type a column letter and a row number, like B7"
    );
    assert_eq!(
        CoordError::ColumnOutOfRange('K').to_string(),
        "column K is off the board (A-J)"
    );
    assert_eq!(
        CoordError::RowOutOfRange(11).to_string(),
        "row 11 is off the board (1-10)"
    );
}

// ---- Unchanged from Week 10 ----

#[test]
fn fire_hit_miss_and_repeat() {
    let mut board = new_board();
    place(&mut board, 0, 0, 2, true).unwrap();
    assert_eq!(fire(&mut board, 0, 0), Shot::Hit);
    assert_eq!(fire(&mut board, 5, 5), Shot::Miss);
    assert_eq!(fire(&mut board, 0, 0), Shot::AlreadyShot);
    assert_eq!(fire(&mut board, 5, 5), Shot::AlreadyShot);
    assert!(!all_sunk(&board));
    assert_eq!(fire(&mut board, 0, 1), Shot::Hit);
    assert!(all_sunk(&board));
}

#[test]
fn render_hides_ships_unless_revealed() {
    let mut board = new_board();
    place(&mut board, 0, 0, 2, true).unwrap();
    assert!(!render(&board, false).contains('S'));
    assert!(render(&board, true).contains('S'));
}

#[test]
fn score_for_table() {
    assert_eq!(score_for(17), 83);
    assert_eq!(score_for(50), 50);
    assert_eq!(score_for(100), 0);
    assert_eq!(score_for(150), 0);
}
