rust
// Returns the byte index where `searcher` first returns true.
// `searcher` can inspect `char`s decoded from UTF-8.
// Non UTF-8 encoded characters are skipped.
fn find_char(&self, searcher: FnMut(char) -> bool) -> Option<usize>;
