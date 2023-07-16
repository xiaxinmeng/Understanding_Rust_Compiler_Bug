Rust
extern "Rust" {
    fn do_not_elim(v: u16);
}

// Commented out, otherwise cargo asm doesn't pick up the function.
// #[no_mangle]
pub fn print_if_some(arg: Result<u16, u8>) {
    if arg.is_ok() {
        let v = arg.unwrap();
        if v == 42 {
            unsafe{
                do_not_elim(v);
            }
        }
    }
}
