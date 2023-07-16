rust
macro_rules! m { ($tt:tt) => () }

m!("str"suffix); // OK right now

#[cfg(FALSE)]
fn f() {
    "str"suffix; // ERROR, but probably shouldn't be
                 // will be OK if semantic checking of literals is delayed until HIR
}

fn main() {
    "str"suffix; // ERROR, expected semantic error
}
