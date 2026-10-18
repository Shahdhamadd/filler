use crate::parser::{parse_player, parse_size, read_board, read_piece, Player};

fn lines_of(text: &str) -> impl Iterator<Item = String> + '_ {
    text.lines().map(|line| line.to_string())
}

#[test]
fn reads_player_one() {
    let player = parse_player("$$$ exec p1 : [robots/bender]").unwrap();
    assert_eq!(player.number, 1);
}

#[test]
fn reads_player_two() {
    let player = parse_player("$$$ exec p2 : [robots/bender]").unwrap();
    assert_eq!(player.number, 2);
}

#[test]
fn rejects_bad_player_line() {
    assert!(parse_player("$$$ exec p3 : [robots/bender]").is_none());
    assert!(parse_player("hello").is_none());
}

#[test]
fn reads_anfield_size() {
    assert_eq!(parse_size("Anfield 20 15:"), Some((20, 15)));
}

#[test]
fn reads_piece_size() {
    assert_eq!(parse_size("Piece 4 1:"), Some((4, 1)));
}

#[test]
fn rejects_bad_size() {
    assert_eq!(parse_size("Anfield 20:"), None);
    assert_eq!(parse_size("Anfield a b:"), None);
}

#[test]
fn reads_board() {
    let text = "    01234567890123456789
000 ....................
001 ....................
002 .........@..........
003 ....................
004 ....................
005 ....................
006 ....................
007 ....................
008 ....................
009 ....................
010 ....................
011 ....................
012 .........$..........
013 ....................
014 ....................";
    let mut lines = lines_of(text);
    let board = read_board("Anfield 20 15:", &mut lines).unwrap();

    assert_eq!(board.width, 20);
    assert_eq!(board.height, 15);
    assert_eq!(board.cells.len(), 15);
    assert_eq!(board.cells[0].len(), 20);
    assert_eq!(board.cells[2][9], '@');
    assert_eq!(board.cells[12][9], '$');
    assert_eq!(board.cells[0][0], '.');
}

#[test]
fn board_stops_after_its_rows() {
    let text = "    0123
000 @...
001 ...$
Piece 2 1:
OO";
    let mut lines = lines_of(text);
    read_board("Anfield 4 2:", &mut lines).unwrap();

    assert_eq!(lines.next().unwrap(), "Piece 2 1:");
}

#[test]
fn rejects_board_with_missing_rows() {
    let text = "    0123
000 @...";
    let mut lines = lines_of(text);
    assert!(read_board("Anfield 4 2:", &mut lines).is_none());
}

#[test]
fn reads_one_row_piece() {
    let mut lines = lines_of(".OO.");
    let piece = read_piece("Piece 4 1:", &mut lines).unwrap();

    assert_eq!(piece.width, 4);
    assert_eq!(piece.height, 1);
    assert_eq!(piece.cells, vec![vec!['.', 'O', 'O', '.']]);
}

#[test]
fn reads_bigger_piece() {
    let text = ".##...
###...
#..#..";
    let mut lines = lines_of(text);
    let piece = read_piece("Piece 6 3:", &mut lines).unwrap();

    assert_eq!(piece.width, 6);
    assert_eq!(piece.height, 3);
    assert_eq!(piece.cells[0], vec!['.', '#', '#', '.', '.', '.']);
    assert_eq!(piece.cells[2][3], '#');
}

#[test]
fn player_one_owns_at_and_a() {
    let player = Player { number: 1 };
    assert!(player.is_mine('@'));
    assert!(player.is_mine('a'));
    assert!(player.is_enemy('$'));
    assert!(player.is_enemy('s'));
    assert!(!player.is_mine('.'));
    assert!(!player.is_enemy('.'));
}

#[test]
fn player_two_owns_dollar_and_s() {
    let player = Player { number: 2 };
    assert!(player.is_mine('$'));
    assert!(player.is_mine('s'));
    assert!(player.is_enemy('@'));
    assert!(player.is_enemy('a'));
}
