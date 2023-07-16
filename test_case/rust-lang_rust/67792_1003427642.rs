rust
fn wildcard(_1: Result<T, E>) -> Option<T> {
    debug x => _1; 
    let mut _0: std::option::Option<T>;
    let mut _2: isize;
    let _3: T;
    let mut _4: T;
    let mut _5: isize;
    -snip-
    bb5: {
        drop(_1) -> bb4;  // The actual control flow is a bit complicated,
        // but the whole thing is dropped when the variant is `Err`.
    }
    -snip-
}

fn variable_bound(_1: Result<T, E>) -> Option<T> {
    debug x => _1;
    let mut _0: std::option::Option<T>;
    let mut _2: isize;
    let _3: T;
    let mut _4: T; 
    let _5: E; 
    -snip-
    bb1: {
        _5 = move ((_1 as Err).0: E);
        discriminant(_0) = 0;
        drop(_5) -> bb4; // <- the bound variable is dropped instead
    }
    -snip-
}
