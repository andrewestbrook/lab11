# Error table

One row per failure. Every "What the player sees" entry was copied from a real run of the program.

| Scenario | Function that detects it | Error value | What the player sees |
|---|---|---|---|
| Fleet file does not exist (`data/fleet.txt` renamed) | `load_fleet` | `Err(DataError::Io(..))` (kind `NotFound`) | `Error: could not load data/fleet.txt: No such file or directory (os error 2)`, then the program exits with code 1 (fatal) |
| A folder (`data`) is given as the fleet path | `load_fleet` | `Err(DataError::Io(..))` (reading a directory fails) | `Error: could not load data: Is a directory (os error 21)`, then exit code 1 (fatal) |
| Fleet file has only comments (`data/fleet_empty.txt`) | `load_fleet` | `Err(DataError::NoShips("data/fleet_empty.txt"))` | `Error: could not load data/fleet_empty.txt: no valid ships in data/fleet_empty.txt`, then exit code 1 (fatal) |
| Every line in the fleet file is bad (`data/fleet_all_bad.txt`) | `load_fleet` (each line rejected by `parse_ship_line`) | `Err(DataError::NoShips("data/fleet_all_bad.txt"))` | `Error: could not load data/fleet_all_bad.txt: no valid ships in data/fleet_all_bad.txt`, then exit code 1 (fatal) |
| Fleet line with a length outside 2-5 (`data/fleet.txt` line 10) | `parse_ship_line` | `Err(ShipLineError::LengthOutOfRange(1))`, stored in `skipped` | `data/fleet.txt line 10: has length 1, outside 2-5`; the line is skipped and the menu appears |
| Fleet line with a length that is not a number (line 11) | `parse_ship_line` | `Err(ShipLineError::BadLength("x"))`, stored in `skipped` | `data/fleet.txt line 11: has length "x", which is not a whole number`; the game carries on |
| Fleet line without exactly one comma (line 12) | `parse_ship_line` | `Err(ShipLineError::MissingComma)`, stored in `skipped` | `data/fleet.txt line 12: needs exactly one comma, like Carrier,5`; the game carries on |
| Practice board file has nine rows (`data/boards/broken_rows.txt`) | `parse_board` (through `load_board`) | `Err(DataError::Board(BoardError::WrongRowCount(9)))` | `Practice board unavailable: bad board layout: found 9 rows, but a board needs exactly 10`, then back to the menu |
| Practice board file has a bad character (`data/boards/broken_char.txt`) | `parse_board` (through `load_board`) | `Err(DataError::Board(BoardError::BadChar { line: 6, ch: 'x' }))` | `Practice board unavailable: bad board layout: line 6 contains 'x', but only '.' and 'S' are allowed`, then back to the menu |
| Scores file does not exist (first run) | `load_scores` | Not an error: `Ok(vec![])` (the `NotFound` case) | `  no scores yet` under TOP 5, and the menu draws |
| Scores path cannot be read (`SCORES_PATH` set to the `data` folder) | `load_scores` | `Err(DataError::Io(..))` (not `NotFound`) | `  scores unavailable: Is a directory (os error 21)`, and the menu still draws |
| Scores file cannot be written after a win (`SCORES_PATH` set to the `data` folder) | `append_score` | `Err(DataError::Io(..))` | `Could not save the score: Is a directory (os error 21)`, then back to the menu |
| `K11` typed at the fire prompt | `parse_coord` | `Err(CoordError::ColumnOutOfRange('K'))` (column is checked before row) | `column K is off the board (A-J)`; asked again, no shot counted |
| `A0` typed at the fire prompt | `parse_coord` | `Err(CoordError::RowOutOfRange(0))` | `row 0 is off the board (1-10)`; asked again, no shot counted |
| `7B` or a blank line typed at the fire prompt | `parse_coord` | `Err(CoordError::BadFormat)` | `type a column letter and a row number, like B7`; asked again, no shot counted |
