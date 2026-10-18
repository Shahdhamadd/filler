use crate::parser::{Board, Piece, Player};
use crate::placement::{all_valid_moves, shape_cells};
use std::collections::VecDeque;

pub fn distance_map(board: &Board, player: &Player, from_enemy: bool) -> Vec<Vec<usize>> {
    let far_away = board.width * board.height;
    let mut distance = vec![vec![far_away; board.width]; board.height];
    let mut queue = VecDeque::new();

    for (y, row) in board.cells.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let start = if from_enemy {
                player.is_enemy(cell)
            } else {
                player.is_mine(cell)
            };
            if start {
                distance[y][x] = 0;
                queue.push_back((x, y));
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        let next = distance[y][x] + 1;
        let neighbours = [
            (x as i32 + 1, y as i32),
            (x as i32 - 1, y as i32),
            (x as i32, y as i32 + 1),
            (x as i32, y as i32 - 1),
        ];

        for (nx, ny) in neighbours {
            if nx < 0 || ny < 0 || nx >= board.width as i32 || ny >= board.height as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if board.cells[ny][nx] == '.' && distance[ny][nx] == far_away {
                distance[ny][nx] = next;
                queue.push_back((nx, ny));
            }
        }
    }

    distance
}

pub fn move_score(distance: &[Vec<usize>], piece: &Piece, x: i32, y: i32) -> usize {
    let mut score = 0;
    for (board_x, board_y) in shape_cells(piece, x, y) {
        score += distance[board_y as usize][board_x as usize];
    }
    score
}

pub fn space_score(board: &Board, piece: &Piece, player: &Player, x: i32, y: i32) -> i32 {
    let my_symbol = if player.number == 1 { '@' } else { '$' };
    let mut after = Board {
        width: board.width,
        height: board.height,
        cells: board.cells.clone(),
    };
    for (board_x, board_y) in shape_cells(piece, x, y) {
        after.cells[board_y as usize][board_x as usize] = my_symbol;
    }

    let mine = distance_map(&after, player, false);
    let theirs = distance_map(&after, player, true);

    let mut score = 0;
    for y in 0..board.height {
        for x in 0..board.width {
            if mine[y][x] < theirs[y][x] {
                score += 1;
            }
            if theirs[y][x] < mine[y][x] {
                score -= 1;
            }
        }
    }
    score
}

pub fn pick_move(board: &Board, piece: &Piece, player: &Player) -> Option<(i32, i32)> {
    let mut moves = all_valid_moves(board, piece, player);
    let distance = distance_map(board, player, true);

    moves.sort_by_key(|&(x, y)| move_score(&distance, piece, x, y));
    moves.truncate(10);

    let mut best_move = None;
    let mut best_space = i32::MIN;

    for (x, y) in moves {
        let space = space_score(board, piece, player, x, y);
        if space > best_space {
            best_space = space;
            best_move = Some((x, y));
        }
    }

    best_move
}
