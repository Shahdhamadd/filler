use crate::parser::{Board, Piece};

pub fn make_board(rows: &[&str]) -> Board {
    let cells: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();
    Board {
        width: cells[0].len(),
        height: cells.len(),
        cells,
    }
}

pub fn make_piece(rows: &[&str]) -> Piece {
    let cells: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();
    Piece {
        width: cells[0].len(),
        height: cells.len(),
        cells,
    }
}
