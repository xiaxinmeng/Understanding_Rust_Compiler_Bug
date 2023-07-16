 rust
fn main() {
    let x = 5i;
    match (&x,) {
        (__arg0,) => {
            static __STATIC_FMTSTR: [Piece, ..3] = [
                String("x is: "),
                Argument(Argument {
                    position: ArgumentNext,
                    format: FormatSpec {
                        fill: ' ',
                        align: AlignUnknown,
                        flags: 0u,
                        precision: CountImplied,
                        width: CountImplied,
                    },
                },
                String(" foobar"),
            ];
            let __args_vec = &[argument(secret_string, __arg0)];
            let __args = unsafe { Arguments::new(__STATIC_FMTSTR, __args_vec) };

            println_args(&__args)
        }
    };
}
