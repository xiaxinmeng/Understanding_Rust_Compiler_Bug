 rust
fn main() {
    let x = 5i;
    match (&x,) {
        (__arg0,) => {
            static __STATIC_FMTSTR: [&'static str, ..2u] = [
                "x is: ",
                " foobar"
            ];
            static __STATIC_FMTARGS: [rt::Argument<'static>, ..0u] = []; // optimized out
            let __args_vec = &[argument(secret_show, __arg0)];
            let __args = unsafe {
                Arguments::new(__STATIC_FMTSTR, __STATIC_FMTARGS, __args_vec)
            };
            println_args(&__args)
        }
    };
}
