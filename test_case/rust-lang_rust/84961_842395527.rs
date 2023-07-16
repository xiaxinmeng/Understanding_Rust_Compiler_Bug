plain
    Checking toml v0.5.7
    Checking url v2.2.2
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.54 (/checkout/src/tools/clippy/clippy_lints)
error[E0621]: explicit lifetime required in the type of `text`
    |
    |
551 | fn check_code(cx: &LateContext<'_>, text: &str, edition: Edition, span: Span) {
    |                                           ---- help: add explicit lifetime `'static` to the type of `text`: `&'static str`
...
619 |     if thread::spawn(move || has_needless_main(text.to_owned(), edition)).join().expect("thread::spawn failed") {
    |        ^^^^^^^^^^^^^ lifetime `'static` required
error: aborting due to previous error

For more information about this error, try `rustc --explain E0621`.
error: could not compile `clippy_lints`
