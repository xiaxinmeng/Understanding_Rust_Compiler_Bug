rust
#![feature(nll)]

pub struct DATA;

impl Drop for DATA {
    fn drop(&mut self) {}
}

pub fn record<'a: 'x, 'x>(
    s: &'a mut (),
    op: fn(&'x mut (), DATA),
    data: DATA,
) {
    move || {
        op(&mut *s, data);
    };
}
