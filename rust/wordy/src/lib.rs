pub struct WordProblem;

static CMD_START: &str = "What is ";

pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with(CMD_START) || !command.ends_with("?") {
        return None;
    }

    let tokens: Vec<_> = command[CMD_START.len()..command.len() - 1]
        .split_ascii_whitespace()
        .filter(|&s| !s.is_empty() && !s.eq("by"))
        .collect();

    if tokens.len() % 2 == 0 {
        return None;
    }

    if let Ok(mut r) = tokens[0].parse::<i32>() {
        for chunk in tokens[1..].chunks(2) {
            match (chunk[0], chunk[1].parse::<i32>()) {
                ("plus", Ok(num)) => r += num,
                ("minus", Ok(num)) => r -= num,
                ("multiplied", Ok(num)) => r *= num,
                ("divided", Ok(num)) => r /= num,
                (_, _) => return None,
            }
        }

        Some(r)
    } else {
        None
    }
}
