rust
extern {
    pub fn malloc(x: usize) -> &'static mut ();
}

#[used]
static FOO: unsafe extern "C" fn(usize) -> &'static mut () = malloc as unsafe extern "C" fn(usize) -> &'static mut ();

fn main() {
    Vec::<u8>::with_capacity(0x13371337deadbeef);
}
