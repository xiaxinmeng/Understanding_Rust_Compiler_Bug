rust
trait T { }

fn main() {
    a(unsafe { std::mem::zeroed() });
}

fn a(_: *mut dyn T) {}
