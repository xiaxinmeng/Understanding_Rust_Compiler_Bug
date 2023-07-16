rust
#![feature(macros_in_extern)]

macro_rules! m {
    () => {
        let
    };
}

extern "C" {
    m!();
}
