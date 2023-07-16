rust
fn test1() {
    macro_rules! m { () => () }
    m!();
}

fn test2() {
    #[macro_export]
    macro_rules! m { () => () } // <- planted into the root module,
                                // so `m!()` in `test1` becomes ambiguous due to restrictions on macro-expanded macros
    m!();
}
