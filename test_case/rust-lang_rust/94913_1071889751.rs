rust
if !rustc_lexer::is_ident(string) {
    panic!("`{:?}` is not a valid identifier", string)
}
