rust
#[lang = "start"]
fn lang_start<T: Terminator>(argc: usize, argv: *const *const u8, main: fn() -> T) -> isize {
    main().report()
}

fn main() {}
