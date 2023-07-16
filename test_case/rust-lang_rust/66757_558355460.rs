rust
struct E;

impl From<!> for E {
    fn from(_: !) -> E {
        E
    }
}

#[allow(unreachable_code)]
fn foo(never: !) {
    <E as From<!>>::from(never);  // Ok
    <E as From<_>>::from(never);  // Inference fails here
}
