rust
#![feature(nll)]
fn main() {
    enum State { A, B }
    let mut state = State::A;
    let closure = || {
        state = match state {
            State::A => State::B,
            State::B => State::A,
        };
    };
    while match state { State::A => false, State::B => true } {
        closure();
    }
}
