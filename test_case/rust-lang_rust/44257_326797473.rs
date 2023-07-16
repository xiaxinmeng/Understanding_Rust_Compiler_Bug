Rust
fn foo() {
    use std::ops::{GeneratorState, Generator};

    || {
        let a = {
            let mut f = || {
                if false { yield }
                format!("a")
            };
            let a = loop {
                match f.resume() {
                    GeneratorState::Complete(e) => break e,
                    GeneratorState::Yielded(()) => {}
                }
                yield // [X] this yield
            };
            a.len() // [X] is caught up by this expression
        };
        if false { yield }
        return a
    };
}
