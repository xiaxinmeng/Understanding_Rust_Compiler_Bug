rust
impl ControlFlow<(), ()> {
    #[must_use]
    pub fn continue_if(should_continue: bool) -> Self {
        if should_continue {
            ControlFlow::CONTINUE
        } else {
            ControlFlow::BREAK
        }
    }

    #[must_use]
    pub fn break_if(should_break: bool) -> Self {
        if should_break {
            ControlFlow::BREAK
        } else {
            ControlFlow::CONTINUE
        }
    }
}
