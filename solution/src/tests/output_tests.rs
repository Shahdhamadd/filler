use crate::output::format_answer;
use crate::parser::Player;
use crate::strategy::pick_move;
use crate::tests::helpers::{make_board, make_piece};

#[test]
fn formats_x_then_y() {
    assert_eq!(format_answer(Some((7, 2))), "7 2\n");
}

#[test]
fn formats_big_numbers() {
    assert_eq!(format_answer(Some((99, 100))), "99 100\n");
}

#[test]
fn formats_negative_numbers() {
    assert_eq!(format_answer(Some((-1, 0))), "-1 0\n");
}

#[test]
fn answers_zero_zero_when_no_move() {
    assert_eq!(format_answer(None), "0 0\n");
}

#[test]
fn answer_is_one_line_with_one_space() {
    let answer = format_answer(Some((7, 2)));
    assert!(answer.ends_with('\n'));
    assert_eq!(answer.matches('\n').count(), 1);
    assert_eq!(answer.matches(' ').count(), 1);
}

#[test]
fn picked_move_is_a_valid_answer() {
    let board = make_board(&[".....", "..@..", ".....", "....$"]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    let answer = pick_move(&board, &piece, &me);
    assert!(answer.is_some());
    assert_eq!(format_answer(answer), "2 1\n");
}

#[test]
fn no_move_gives_zero_zero() {
    let board = make_board(&["$$$", "$@$", "$$$"]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    let answer = pick_move(&board, &piece, &me);
    assert_eq!(answer, None);
    assert_eq!(format_answer(answer), "0 0\n");
}
