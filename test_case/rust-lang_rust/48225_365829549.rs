rust
#![feature(nll)]

enum State {
    A
}

fn bar() {
    let mut state = State::A;

    let mut closure = || {
        state = match state {
            State::A => State::A,
        };
    };

    while match state {  State::A => false } {
        closure();
    }
}

fn main() {}
