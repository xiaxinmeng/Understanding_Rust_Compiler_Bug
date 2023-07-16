 rust
#![feature(default_type_params)]

use std::thunk::Thunk;

fn main() {
    let mut p: Thunk<&()> = match () {
        _ => Thunk::with_arg(move |_| {})
    };
}
