use battleship::board::{self, Board, Shot};
use battleship::data::{self, ScoreRecord, ShipSpec};
use battleship::ui;
use rand::SeedableRng;
use rand::rngs::StdRng;

const FLEET_PATH: &str = "data/fleet.txt";
const SCORES_PATH: &str = "data/scores.txt";
const PRACTICE_PATH: &str = "data/boards/practice.txt";
const TOP_N: usize = 5;

fn main() {
    // TODO (Task 9): replace this unwrap. A load_fleet Err is fatal: print
    // "Error: could not load <path>: <message>" with eprintln! and call
    // std::process::exit(1). Then print each skipped line as
    // "<path> line <n>: <message>" with eprintln! and carry on.
    let report = data::load_fleet(FLEET_PATH).unwrap();
    let fleet = report.fleet;
    if fleet.is_empty() {
        println!("No fleet loaded. Check {FLEET_PATH}.");
        return;
    }

    let menu = vec![
        String::from("Play"),
        String::from("Practice board"),
        String::from("Quit"),
    ];
    loop {
        println!("\n=== BATTLESHIP ===");
        show_top_scores(TOP_N);
        match ui::menu_choice("\nChoose:", &menu) {
            1 => play_game(&fleet),
            2 => practice_game(),
            _ => {
                println!("Goodbye.");
                break;
            }
        }
    }
}

// Print the best scores, highest first, or a message if there are none.
fn show_top_scores(n: usize) {
    println!("TOP {n}");
    // TODO (Task 9): replace this unwrap. If the scores cannot be read, print
    // "  scores unavailable: <message>" and return. The game keeps running.
    let mut records = data::load_scores(SCORES_PATH).unwrap();
    if records.is_empty() {
        println!("  no scores yet");
        return;
    }
    data::sort_scores_desc(&mut records);
    let shown = if records.len() < n { records.len() } else { n };
    for i in 0..shown {
        println!(
            "{:>2}. {:<15} {:>4}",
            i + 1,
            records[i].name,
            records[i].score
        );
    }
}

// A scored game against a randomly placed fleet.
fn play_game(fleet: &[ShipSpec]) {
    let mut board = board::new_board();
    let mut rng = StdRng::seed_from_u64(rand::random::<u64>());
    if let Err(e) = board::place_fleet(&mut board, fleet, &mut rng) {
        println!("The fleet does not fit: {e}. Check {FLEET_PATH}.");
        return;
    }
    if let Some(shots) = play_round(&mut board) {
        let score = board::score_for(shots);
        println!("All ships sunk in {shots} shots. Score: {score}");
        if score > 0 {
            let name = ui::prompt("Name for the high-score list: ");
            if !name.is_empty() {
                let record = ScoreRecord { name, score };
                // TODO (Task 9): when this fails, say why. Print
                // "Could not save the score: <message>" and carry on.
                if data::append_score(SCORES_PATH, &record).is_ok() {
                    println!("Score saved.");
                }
            }
        }
    }
}

// An unscored game on the fixed layout in the practice file.
fn practice_game() {
    match data::load_board(PRACTICE_PATH) {
        Ok(mut board) => {
            if let Some(shots) = play_round(&mut board) {
                println!("Practice board cleared in {shots} shots.");
            }
        }
        // TODO (Task 9): include the reason: "Practice board unavailable: <message>".
        Err(_) => println!("Practice board unavailable."),
    }
}

// Fire until every ship is sunk. Returns the number of shots, or None if the
// player types "quit". A rejected coordinate is not a shot.
fn play_round(board: &mut Board) -> Option<u32> {
    let mut shots = 0;
    while !board::all_sunk(board) {
        println!();
        print!("{}", board::render(board, false));
        let text = ui::prompt("Fire at (like B7, or 'quit'): ");
        if text.eq_ignore_ascii_case("quit") {
            return None;
        }
        match board::parse_coord(&text) {
            // TODO (Task 9): show the CoordError message instead of this generic line.
            Err(_) => println!("Enter a column A-J and a row 1-10, like B7."),
            Ok((row, col)) => match board::fire(board, row, col) {
                Shot::Hit => {
                    shots += 1;
                    println!("Hit!");
                }
                Shot::Miss => {
                    shots += 1;
                    println!("Miss.");
                }
                Shot::AlreadyShot => println!("You already fired there."),
            },
        }
    }
    print!("\n{}", board::render(board, false));
    Some(shots)
}
