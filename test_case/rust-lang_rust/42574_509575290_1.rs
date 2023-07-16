rust
#![feature(nll)]

fn doit(data: &'static mut ()) {
    || {
        let d: &mut _ = data;
        doit(d)
    };
}
