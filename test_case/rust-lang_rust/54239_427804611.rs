rust
#![feature(generators, generator_trait)]
#![deny(missing_debug_implementations)]

use std::ops::Generator;

struct DontLookAtMe(i32);

fn secret() -> DontLookAtMe {
    DontLookAtMe(41)
}

// Comment this function out to fix the lint...
pub fn looking() -> impl Generator<Yield = (), Return = i32> {
    || {
        let d = secret();
        yield;
        d.0
    }
}
