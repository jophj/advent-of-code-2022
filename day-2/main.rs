use std::fs::read;

// A/X = rock/lose/1
// B/Y = paper/draw/2
// C/Z = scissors/win/3

fn main() {
    let file_content = read("./input.txt");
    let binding = file_content.unwrap();
    let file_txt = String::from_utf8_lossy(&binding);

    let mut score = 0;
    let mut strat_score = 0;
    for line in file_txt.lines() {
        let (player1, player2) = line.split_once(" ").unwrap();

        let win_score = match (player1, player2) {
            ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            _ => 3,
        };

        let move_score = match player2 {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0
        };

        let strat_move_score = match (player1, player2) {
            ("B", "X") | ("A", "Y") | ("C", "Z") => 1,
            ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
            _ => 3,
        };
        let strat_win_score = match player2 {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0
        };

        score += win_score + move_score;
        strat_score += strat_move_score + strat_win_score;
    }
    println!("{}", score);
    println!("{}", strat_score);
}