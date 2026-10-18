use crate::parser::Player;
use crate::placement::{all_valid_moves, can_place};
use crate::tests::helpers::{make_board, make_piece};

#[test]
fn rejects_going_off_the_right_side() {
    let board = make_board(&["....@", ".....", "$...."]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert!(!can_place(&board, &piece, &me, 4, 0));
}

#[test]
fn rejects_going_off_the_bottom() {
    let board = make_board(&[".....", ".....", "$...@"]);
    let piece = make_piece(&["O", "O"]);
    let me = Player { number: 1 };

    assert!(!can_place(&board, &piece, &me, 4, 2));
}

#[test]
fn rejects_going_off_the_left_side() {
    let board = make_board(&["@....", ".....", "....$"]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert!(!can_place(&board, &piece, &me, -1, 0));
}

#[test]
fn rejects_going_off_the_top() {
    let board = make_board(&["@....", ".....", "....$"]);
    let piece = make_piece(&["O", "O"]);
    let me = Player { number: 1 };

    assert!(!can_place(&board, &piece, &me, 0, -1));
}

#[test]
fn rejects_being_completely_outside() {
    let board = make_board(&["@....", ".....", "....$"]);
    let piece = make_piece(&["O"]);
    let me = Player { number: 1 };

    assert!(!can_place(&board, &piece, &me, 10, 10));
    assert!(!can_place(&board, &piece, &me, -5, -5));
}

#[test]
fn allows_empty_piece_cells_to_hang_outside() {
    let board = make_board(&["@....", ".....", "....$"]);
    let piece = make_piece(&[".O"]);
    let me = Player { number: 1 };

    assert!(can_place(&board, &piece, &me, -1, 0));
}

#[test]
fn allows_touching_the_corners_from_inside() {
    let board = make_board(&["....@", ".....", "$...."]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert!(can_place(&board, &piece, &me, 3, 0));
}

#[test]
fn every_found_move_stays_inside_the_board() {
    let board = make_board(&[".....", ".@...", ".....", ".....", "....$"]);
    let piece = make_piece(&[".O.", "OOO", ".O."]);
    let me = Player { number: 1 };

    let moves = all_valid_moves(&board, &piece, &me);
    assert!(!moves.is_empty());

    for (x, y) in moves {
        for row in 0..piece.height {
            for col in 0..piece.width {
                if piece.cells[row][col] == 'O' {
                    let board_x = x + col as i32;
                    let board_y = y + row as i32;
                    assert!(board_x >= 0 && board_x < board.width as i32);
                    assert!(board_y >= 0 && board_y < board.height as i32);
                }
            }
        }
    }
}
