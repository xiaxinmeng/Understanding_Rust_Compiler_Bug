rust
fn bar<'a>() -> &'a u8 { &5 } // OK
fn bar() -> &'_ u8 { &5 } // ERROR: missing lifetime specifier
