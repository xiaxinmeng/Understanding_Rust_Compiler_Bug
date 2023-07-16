rust
fn doit(data: &'static mut ()) {
    || {
        let d = data;
        doit(d);
    };
}
