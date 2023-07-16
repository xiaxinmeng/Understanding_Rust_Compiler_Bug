 rust
struct F<'a> {
    x: &'a mut uint
}

impl<'a> Fn<(),()> for F<'a> {
    #[rust_call_abi_hack]
    fn call(&self, _: ()) {
        *self.x = 42; // error: cannot assign to data in a `&` reference
    }
}

fn main() {
    let mut x = 0u;
    let _f = F { x: &mut x };
}
