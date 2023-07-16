rust
pub fn is_pangram(sentence: &str) -> bool {
    let mut letters = std::collections::HashSet::<_>::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
    sentence.to_lowercase().chars().any(|c| {
        letters.remove(&c);
        letters.is_empty()
    })
}
