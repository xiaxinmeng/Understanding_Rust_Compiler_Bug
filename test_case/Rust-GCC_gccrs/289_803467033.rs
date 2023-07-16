rust
#[lang = "start"]
fn lang_start(argc: usize, argv: *const *const u8, main: fn()) -> isize {
    main();
    0
}

fn main() {}
