rust
mod state {
    use std::borrow::Cow;

    #[derive(PartialEq, Eq)]
    pub struct State(Cow<'static, str>);
    impl State {
        pub fn new(s: &'static str) -> Self {
            State(Cow::Borrowed(s))
        }
    }

    pub const COLD: State = State(Cow::Borrowed("cold"));
}

fn main() {
    let s = state::State::new("hot");
    match s {
        state::COLD => (),
        _ => (),
    }
}
