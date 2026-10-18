use std::env;
use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;

fn color(cell: char) -> &'static str {
    match cell {
        '@' => "\x1b[48;5;99m",
        'a' => "\x1b[48;5;183m",
        '$' => "\x1b[48;5;166m",
        's' => "\x1b[48;5;222m",
        _ => "\x1b[48;5;236m",
    }
}

fn player_name(line: &str) -> String {
    let start = line.find('[').map(|i| i + 1).unwrap_or(0);
    let end = line.find(']').unwrap_or(line.len());
    line[start..end].to_string()
}

fn draw(rows: &[String], names: &[String]) {
    let cell_width = if rows[0].len() <= 60 { "  " } else { " " };
    let mut screen = String::from("\x1b[H");
    let mut p1 = 0;
    let mut p2 = 0;

    for row in rows {
        for cell in row.chars() {
            if cell == '@' || cell == 'a' {
                p1 += 1;
            }
            if cell == '$' || cell == 's' {
                p2 += 1;
            }
            screen.push_str(color(cell));
            screen.push_str(cell_width);
        }
        screen.push_str("\x1b[0m\n");
    }

    let name1 = names.get(0).cloned().unwrap_or_default();
    let name2 = names.get(1).cloned().unwrap_or_default();

    print!("{}", screen);
    println!("\x1b[38;5;99m■ p1 {}: {}\x1b[0m", name1, p1);
    println!("\x1b[38;5;166m■ p2 {}: {}\x1b[0m", name2, p2);
    io::stdout().flush().unwrap();
}

fn main() {
    let delay: u64 = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(30);
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(|line| line.ok());
    let mut names = Vec::new();

    print!("\x1b[2J");

    while let Some(line) = lines.next() {
        if line.starts_with("$$$") {
            names.push(player_name(&line));
        } else if line.starts_with("Anfield") {
            let height: usize = line
                .split_whitespace()
                .nth(2)
                .and_then(|word| word.trim_end_matches(':').parse().ok())
                .unwrap_or(0);
            lines.next();

            let mut rows = Vec::new();
            for _ in 0..height {
                if let Some(row) = lines.next() {
                    rows.push(row.split_whitespace().nth(1).unwrap_or("").to_string());
                }
            }

            if !rows.is_empty() {
                draw(&rows, &names);
                thread::sleep(Duration::from_millis(delay));
            }
        } else if line.starts_with("Player") {
            println!("{}", line);
        }
    }
}
