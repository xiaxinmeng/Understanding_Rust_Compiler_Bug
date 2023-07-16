 rust
mod u8 {}
fn f(_: u8) {}
//~^ WARN the type `u8` is shadowed by the user-defined module `::u8`, use `::std::u8` instead
