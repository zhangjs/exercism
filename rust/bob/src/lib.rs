pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let has_letter = message.chars().any(|c| c.is_alphabetic());
    let all_capital = has_letter && !message.chars().any(|c| c.is_lowercase()); 
    let question = message.ends_with("?");

    match (all_capital, question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        _ => "Whatever.",
    }
}
