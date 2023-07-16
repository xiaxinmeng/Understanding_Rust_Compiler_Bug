 rust
#![feature(associated_types)]

fn add_state(op: <int as HasState>::State) {}

trait HasState {
    type State;
}
