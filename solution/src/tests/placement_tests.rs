use crate::parser::Player;
use crate::placement::{all_valid_moves, can_place};
use crate::tests::helpers::{make_board, make_piece};

#[test]
fn accepts_exactly_one_overlap() {
    let board = make_board(&[".....", "..@..", ".....", "....$"]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert!(can_place(&board, &piece, &me, 2, 1));
    assert!(can_place(&board, &piece, &me, 1, 1));
}

#[test]
fn rejects_zero_overlap() {
    let board = make_board(&[".....", "..@..", ".....", "....$"]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert!(!can_place(&board, &piece, &me, 0, 0));
    assert!(!can_place(&board, &piece, &me, 3, 2));
}

#[test]
fn rejects_two_overlaps() {
    let board = make_board(&[".....", ".@@..", ".....", "....$"]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert!(!can_place(&board, &piece, &me, 1, 1));
}

#[test]
fn rejects_touching_the_enemy() {
    let board = make_board(&[".....", "..@$.", ".....", "....."]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert!(!can_place(&board, &piece, &me, 2, 1));
}

#[test]
fn rejects_touching_the_enemy_last_piece() {
    let board = make_board(&[".....", "..@s.", ".....", "....."]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert!(!can_place(&board, &piece, &me, 2, 1));
}

#[test]
fn our_last_piece_counts_as_ours() {
    let board = make_board(&[".....", "..a..", ".....", "....$"]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert!(can_place(&board, &piece, &me, 2, 1));
}

#[test]
fn empty_cells_of_the_piece_do_not_matter() {
    let board = make_board(&[".....", "..@$.", ".....", "....."]);
    let piece = make_piece(&["O."]);
    let me = Player { number: 1 };

    assert!(can_place(&board, &piece, &me, 2, 1));
}

#[test]
fn works_for_player_two() {
    let board = make_board(&[".....", "..$..", ".....", "@...."]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 2 };

    assert!(can_place(&board, &piece, &me, 2, 1));
    assert!(!can_place(&board, &piece, &me, 0, 3));
}

#[test]
fn finds_all_moves_for_a_single_cell() {
    let board = make_board(&[".....", "..@..", ".....", "....$"]);
    let piece = make_piece(&["O"]);
    let me = Player { number: 1 };

    assert_eq!(all_valid_moves(&board, &piece, &me), vec![(2, 1)]);
}

#[test]
fn finds_all_moves_for_a_line() {
    let board = make_board(&[".....", "..@..", ".....", "....$"]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert_eq!(all_valid_moves(&board, &piece, &me), vec![(1, 1), (2, 1)]);
}

#[test]
fn finds_no_moves_when_blocked() {
    let board = make_board(&["$$$", "$@$", "$$$"]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert!(all_valid_moves(&board, &piece, &me).is_empty());
}
