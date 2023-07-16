rust
use std::marker::PhantomData;

use std::mem::{self, MaybeUninit};

struct Bug<S> {
    A: [(); {
        let x: Option<S> = None;
        0
    }],
}
