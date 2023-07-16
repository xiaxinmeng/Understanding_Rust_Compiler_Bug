rust
pub struct SpannedLabel<'a> {
    span: Span,
    label: &'a dyn fmt::Display, // maybe this should be an enum supporting `String` and `Box<fmt::Display>`
    file_id: FileId, // didn't figure out the exact type yet
}

#[derive(Copy, Clone /* ... */)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
