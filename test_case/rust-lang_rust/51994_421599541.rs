rust
#![feature(self_struct_ctor)]

struct Example(i32, bool);

impl Example {
    fn new() -> Self {
        Self(42, true)
    }
}
