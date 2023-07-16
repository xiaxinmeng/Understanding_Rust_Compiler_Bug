 rust
fn parse<'a>(pat: Pattern, text: &'a str, offset: uint) -> Option<Token<'a>> {
    match pat {
        Literal(s) => {
            let slc = text.slice_from(offset);
// ...
