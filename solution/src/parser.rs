pub struct Player {
    pub number: u8,
}

impl Player {
    pub fn is_mine(&self, cell: char) -> bool {
        if self.number == 1 {
            cell == '@' || cell == 'a'
        } else {
            cell == '$' || cell == 's'
        }
    }

    pub fn is_enemy(&self, cell: char) -> bool {
        if self.number == 1 {
            cell == '$' || cell == 's'
        } else {
            cell == '@' || cell == 'a'
        }
    }
}

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<char>>,
}

pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<char>>,
}

pub fn parse_player(line: &str) -> Option<Player> {
    let word = line.split_whitespace().nth(2)?;
    match word {
        "p1" => Some(Player { number: 1 }),
        "p2" => Some(Player { number: 2 }),
        _ => None,
    }
}

pub fn parse_size(line: &str) -> Option<(usize, usize)> {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.len() != 3 {
        return None;
    }
    let width: usize = words[1].parse().ok()?;
    let height: usize = words[2].trim_end_matches(':').parse().ok()?;
    Some((width, height))
}

pub fn read_board<I: Iterator<Item = String>>(header: &str, lines: &mut I) -> Option<Board> {
    let (width, height) = parse_size(header)?;

    lines.next()?;

    let mut cells = Vec::new();
    for _ in 0..height {
        let line = lines.next()?;
        let row: Vec<char> = line.split_whitespace().nth(1)?.chars().collect();
        cells.push(row);
    }

    Some(Board {
        width,
        height,
        cells,
    })
}

pub fn read_piece<I: Iterator<Item = String>>(header: &str, lines: &mut I) -> Option<Piece> {
    let (width, height) = parse_size(header)?;

    let mut cells = Vec::new();
    for _ in 0..height {
        let line = lines.next()?;
        let row: Vec<char> = line.trim().chars().collect();
        cells.push(row);
    }

    Some(Piece {
        width,
        height,
        cells,
    })
}
