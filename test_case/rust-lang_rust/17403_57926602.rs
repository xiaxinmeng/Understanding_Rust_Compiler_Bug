 rust
#![feature(overloaded_calls)]

struct F<'a> {
    x: &'a mut uint
}

impl<'a> Fn<(), &'a mut uint> for F<'a> {
    #[rust_call_abi_hack]
    fn call(&self, _: ()) -> &'a mut uint {
        self.x
    }
}

fn main() {
    let mut x = 0u;
    let f = F { x: &mut x };
    let _y = f();
    let _z = f();
}
