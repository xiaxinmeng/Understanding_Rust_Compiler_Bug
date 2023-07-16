rust
#![feature(nll)]

enum State {
    A,
    B
}

fn bar() {
    let mut state = State::A;
    let state_ref = &mut state;

    let mut closure = || {
        *state_ref = match *state_ref {
            _ => State::A,
        };
    };

    while match state { State::A => true, State::B => false } {
        closure();
    }
}

fn main() { 
    bar();
}
