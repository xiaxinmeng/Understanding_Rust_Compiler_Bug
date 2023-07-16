rust
// Works, but only with feature(nll)
fn magical() {
    let x = <fn (&())>::make_f();
}
