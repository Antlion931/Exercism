pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() { return "Fine. Be that way!";}

    let all_caps = message == message.to_uppercase() && message.to_lowercase() != message.to_uppercase(); 
    let question = message.trim().ends_with('?');
    
    match (all_caps, question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        _ => "Whatever.",
    }
}
