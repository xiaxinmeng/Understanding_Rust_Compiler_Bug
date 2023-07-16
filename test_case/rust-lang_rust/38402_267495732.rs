rust
//! Reduced neon test case from the mp4parse_capi crate.

pub struct Parser {
    state: State,
}

#[derive(Clone)]
pub struct State {
    // This struct must have at least two members to trigger
    // NEON code generation.
    pub u: u32,
    pub v: u32,
}

pub unsafe fn parser_new(state: *const State) -> *mut Parser {
    let parser = Box::new(Parser {
        state: (*state).clone(),
    });
    Box::into_raw(parser)
}

