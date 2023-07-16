 rust
impl <'a, 'b, R: Read> Iterator for SentenceSplitter<'a, R> {
    type Item = Result<&'b str, io::CharsError>;
    fn next(&mut self) -> Option<Iterator::Item> { // <--- Look Here
    ...
