rust
fn foo() -> impl FnOnce() -> bool {
    match () { () => () }
    || match () { () => true } // closure
}

fn bar() -> bool {
    // (match () { () => true }) || match () { () => true } // logical or
    match () { () => true } || match () { () => true } // logical or
}

fn baz() -> &'static &'static bool {
    match () { () => () }
    &&match () { () => true } // reference to reference
}

fn quux() -> bool {
    // (match () { () => true }) && match () { () => true } // logical and
    match () { () => true } && match () { () => true } // logical and
}
