pub fn format_answer(answer: Option<(i32, i32)>) -> String {
    match answer {
        Some((x, y)) => format!("{} {}\n", x, y),
        None => String::from("0 0\n"),
    }
}
