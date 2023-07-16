
pub struct VTable {
    a: extern fn (&'static (), int),
    b: extern fn (&'static (), int)
}

static compiles : (&'static (), VTable) = (&'static (), VTable { a : a, b : b });

fn a (_ : &'static (), _ : int) { }
fn b (_ : &'static (), _ : int) { }

fn main() { }
