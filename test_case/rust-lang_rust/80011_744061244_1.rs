rust
let alnum = peekable.next_if(char::is_ascii_alphanumeric);

// equivalent to:
let alnum = match peekable.peek() {
    Some(ch) if ch.is_ascii_alphanumeric() => Some(peekable.next().unwrap()),
    _ => None,
};
