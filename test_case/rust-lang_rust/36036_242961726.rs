 rust
trait Context {
    type Container: ?Sized;
}

impl Context for u16 {
    type Container = u8;
}

struct Wrapper<C: Context> {
    container: *const C::Container
}

fn foobar(_: Wrapper<u16>) {}

fn main() {}
