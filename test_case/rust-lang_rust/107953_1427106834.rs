rust
#![feature(lint_reasons)]

pub fn reachable() {
    loop { break; }
    #[deny(unfulfilled_lint_expectations)]
    #[expect(unreachable_code)]
    { unreachable!() }
}


pub fn not_reachable() {
    loop {}
    #[deny(unfulfilled_lint_expectations)]
    #[expect(unreachable_code)]
    { unreachable!() }
}
