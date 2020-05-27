use regex::Regex;


pub fn reply(message: &str) -> &str {
    match message.trim() {
        msg if msg.trim().len() == 0 => "Fine. Be that way!",
        msg if msg.ends_with("?") && is_shouting(msg) => "Calm down, I know what I'm doing!",
        msg if msg.ends_with("?") => "Sure.",
        msg if is_shouting(msg) => "Whoa, chill out!",
        _ => "Whatever."
    }
}

fn is_shouting(message: &str) -> bool {
    let re_letters = Regex::new(r"[A-z]").unwrap();
    let have_letters: bool = re_letters.is_match(message);
    message.to_uppercase() == message && have_letters
}
