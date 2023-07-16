 rust
mod Y {
    type X = uint;
    extern {
        static x: *const uint;
    }
    fn foo(value: *const X) -> *const X {
        value
    }
}

static foo: *const Y::X = Y::foo(Y::x as *const Y::X);

fn main() {}
