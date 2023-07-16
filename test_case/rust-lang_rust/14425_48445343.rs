 rust
fn main() {
    let s = "this is a sentence";
    let mut it = s.char_indices().scan(0u, |i1, (i2, c)| {
        if c.is_whitespace() {
            let word = s.slice(*i1, i2);
            *i1 = i2+1;
            Some(Some(word))
        } else {
            Some(None)
        }
    }).filter_map(|x| x);
    assert_eq!(it.next(), Some("this"));
    assert_eq!(it.next(), Some("is"));
    assert_eq!(it.next(), Some("a"));
    // assert_eq!(it.next(), Some("sentence")); # NOT WORKING
    assert_eq!(it.next(), None);
}
