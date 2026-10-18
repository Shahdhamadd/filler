use crate::parser::Player;
use crate::placement::can_place;
use crate::strategy::{distance_map, move_score, pick_move, space_score};
use crate::tests::helpers::{make_board, make_piece};

#[test]
fn enemy_cells_have_distance_zero() {
    let board = make_board(&["@....", ".....", "....$"]);
    let me = Player { number: 1 };

    let distance = distance_map(&board, &me, true);
    assert_eq!(distance[2][4], 0);
}

#[test]
fn distance_grows_by_one_each_step() {
    let board = make_board(&["@....", ".....", "....$"]);
    let me = Player { number: 1 };

    let distance = distance_map(&board, &me, true);
    assert_eq!(distance[2][3], 1);
    assert_eq!(distance[1][4], 1);
    assert_eq!(distance[1][3], 2);
    assert_eq!(distance[0][1], 5);
}

#[test]
fn distance_uses_the_closest_enemy() {
    let board = make_board(&["$...@", ".....", "....$"]);
    let me = Player { number: 1 };

    let distance = distance_map(&board, &me, true);
    assert_eq!(distance[0][1], 1);
    assert_eq!(distance[1][4], 1);
}

#[test]
fn distance_cannot_pass_through_other_cells() {
    let board = make_board(&["$@.", ".@.", ".@."]);
    let me = Player { number: 1 };

    let distance = distance_map(&board, &me, true);
    assert_eq!(distance[0][2], 9);
}

#[test]
fn distance_can_start_from_our_cells() {
    let board = make_board(&["@....", ".....", "....$"]);
    let me = Player { number: 1 };

    let distance = distance_map(&board, &me, false);
    assert_eq!(distance[0][0], 0);
    assert_eq!(distance[2][3], 5);
}

#[test]
fn score_adds_the_distance_of_each_shape_cell() {
    let board = make_board(&["@....", ".....", "....$"]);
    let piece = make_piece(&["OO."]);
    let me = Player { number: 1 };

    let distance = distance_map(&board, &me, true);
    assert_eq!(move_score(&distance, &piece, 1, 0), 5 + 4);
}

#[test]
fn space_score_counts_squares_we_reach_first() {
    let board = make_board(&["@...$"]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert_eq!(space_score(&board, &piece, &me, 0, 0), 3 - 2);
}

#[test]
fn picks_the_move_that_wins_more_space() {
    let board = make_board(&["....@....$"]);
    let piece = make_piece(&["OO"]);
    let me = Player { number: 1 };

    assert_eq!(space_score(&board, &piece, &me, 4, 0), 5);
    assert_eq!(space_score(&board, &piece, &me, 3, 0), 4);
    assert_eq!(pick_move(&board, &piece, &me), Some((4, 0)));
}

#[test]
fn picked_move_is_always_valid() {
    let board = make_board(&[".......", ".@@....", ".@.....", ".......", "......$"]);
    let piece = make_piece(&[".O", "OO"]);
    let me = Player { number: 1 };

    let (x, y) = pick_move(&board, &piece, &me).unwrap();
    assert!(can_place(&board, &piece, &me, x, y));
}
