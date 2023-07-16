rust
#[derive(SessionDiagnostic)]
#[error(passes::doc_alias_bad_char)]
pub struct DocAliasBadChar<'a> {
    #[primary_span]
    pub span: Span,
    pub attr_str: &'a str,
    pub char_: char,
}
