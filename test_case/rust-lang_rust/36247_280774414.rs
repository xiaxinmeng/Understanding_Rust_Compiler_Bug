rust
mod sys {
    extern { fn noop(); }
}

fn noop() {
    // I assert that this is safe to call at any time
    unsafe { noop(); }
}
