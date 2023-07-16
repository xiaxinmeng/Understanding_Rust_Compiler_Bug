rust
struct Lexer<'d>(&'d ());

impl Lexer<'d> {
    type Cursor = ();
}

fn test(_: Lexer::Cursor) {}
