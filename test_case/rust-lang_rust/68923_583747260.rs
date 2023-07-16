rust
let gen = |arg: &mut bool| {
    *arg = true;
    let arg = yield ();
    *arg = false
    let arg = yield ();
};
