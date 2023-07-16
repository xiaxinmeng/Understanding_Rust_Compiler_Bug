rust
pub fn with_globals<F, R>(f: F) -> R
    where F: FnOnce() -> R
{
    let globals = Globals::new();
    GLOBALS.set(&globals, || {
        syntax_pos::GLOBALS.set(&globals.syntax_pos_globals, f)
    })
}
