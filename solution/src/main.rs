mod output;
mod parser;
mod placement;
mod strategy;

#[cfg(test)]
mod tests;

use output::format_answer;
use parser::{parse_player, read_board, read_piece, Board, Player};
use std::io::{self, BufRead, Write};
use strategy::pick_move;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(|line| line.ok());

    let mut player = Player { number: 1 };
    let mut board: Option<Board> = None;

    while let Some(line) = lines.next() {
        if line.starts_with("$$$") {
            if let Some(p) = parse_player(&line) {
                player = p;
            }
        } else if line.starts_with("Anfield") {
            board = read_board(&line, &mut lines);
        } else if line.starts_with("Piece") {
            let piece = read_piece(&line, &mut lines);

            let mut answer = None;
            if let (Some(b), Some(p)) = (&board, &piece) {
                answer = pick_move(b, p, &player);
            }

            print!("{}", format_answer(answer));
            io::stdout().flush().unwrap();
        }
    }
}
