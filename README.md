# Battleship (Weeks 10-15 lab thread): Week 11

Week 11 replaces `None` and "print a message and return something empty" with `Result`. Each failure gets a name, the name travels upward, and `main` decides what the player sees. This starter is the Week 10 solution with new signatures. It builds with no warnings and runs, but 16 of 40 tests fail until you finish the tasks.

## Run

```
cargo run
cargo test --no-fail-fast
cargo build      # must finish with zero warnings
```

Run from the project folder (the one with `Cargo.toml`), not from `src/`.

## Layout

| File | Who edits it in Week 11 |
|---|---|
| `src/board.rs` | You: `CoordError` Display, `parse_coord`, `place`. `place_fleet` and `FleetError` are provided. |
| `src/data.rs` | You: Displays for `ShipLineError` and `BoardError`, `parse_ship_line`, `load_fleet`, `parse_board`, `load_board`, `load_scores`, `append_score`. `DataError` is provided: study it. |
| `src/main.rs` | You: replace each `TODO (Task 9)`. |
| `src/ui.rs` | Nobody. |
| `tests/board_tests.rs`, `tests/data_tests.rs` | Provided tests. You add three at the bottom of `data_tests.rs`. |
| `data/` | Read only. New files: `fleet_empty.txt`, `fleet_all_bad.txt`, `boards/broken_rows.txt`, `boards/broken_char.txt`. |
| `ERROR_TABLE.md`, `design/` | You (Tasks 10 and 12). |

## Tasks (commit after each)

1. `CoordError` Display and `parse_coord`.
2. `place` with `PlaceError`; delete `can_place`.
3. `ShipLineError` Display and `parse_ship_line`.
4. `load_fleet` with `FleetReport`, using `?` for the read.
5. `BoardError` Display and `parse_board`.
6. `load_board` with `?` twice.
7. `load_scores`: `NotFound` is normal, anything else is an `Err`.
8. `append_score` with `?`.
9. `main`: handle every `Err`. No `unwrap` or `expect` may remain in `data.rs`, `board.rs`, or `main.rs`.
10. Complete `ERROR_TABLE.md`.
11. Write the three tests at the bottom of `tests/data_tests.rs`.
12. Flowcharts in `design/`.

Cut-line if time runs out: drop `Box<dyn Error>`, then Tasks 5 and 6. Core: Tasks 1, 2, 4, 9.

## Rules

- `cargo build` shows no warnings.
- At least one commit per function you write.
- Do not change `ui.rs` or the provided tests.
