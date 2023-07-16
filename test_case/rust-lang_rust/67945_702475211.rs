rust
use std::mem::MaybeUninit;

struct Bug<S> {
    A: [(); {
        let x: S = MaybeUninit::uninit();
        let b = &*(&x as *const _ as *const S);
        0
    }],
}
