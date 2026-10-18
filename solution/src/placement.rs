use crate::parser::{Board, Piece, Player};

pub fn shape_cells(piece: &Piece, x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut cells = Vec::new();

    for row in 0..piece.cells.len() {
        for col in 0..piece.cells[row].len() {
            if piece.cells[row][col] != '.' {
                cells.push((x + col as i32, y + row as i32));
            }
        }
    }

    cells
}

pub fn can_place(board: &Board, piece: &Piece, player: &Player, x: i32, y: i32) -> bool {
    let mut overlaps = 0;

    for (board_x, board_y) in shape_cells(piece, x, y) {
        if board_x < 0 || board_y < 0 {
            return false;
        }
        if board_x >= board.width as i32 || board_y >= board.height as i32 {
            return false;
        }

        let cell = board.cells[board_y as usize][board_x as usize];

        if player.is_enemy(cell) {
            return false;
        }
        if player.is_mine(cell) {
            overlaps += 1;
        }
    }

    overlaps == 1
}

pub fn all_valid_moves(board: &Board, piece: &Piece, player: &Player) -> Vec<(i32, i32)> {
    let mut moves = Vec::new();

    let first_y = -(piece.height as i32) + 1;
    let first_x = -(piece.width as i32) + 1;

    for y in first_y..board.height as i32 {
        for x in first_x..board.width as i32 {
            if can_place(board, piece, player, x, y) {
                moves.push((x, y));
            }
        }
    }

    moves
}
