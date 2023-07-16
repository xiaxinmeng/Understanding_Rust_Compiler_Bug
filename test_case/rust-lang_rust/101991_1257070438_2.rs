rust
// src/main.rs

macro_rules! source_text {
    ($end:ident) => {
        repro::source_text!(start $end)
    };
}

// huh
fn sick_fn_bro() {}

fn main() {
    println!("{}", source_text!(end));
}
