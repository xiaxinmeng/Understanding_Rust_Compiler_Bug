 shell
[tamird:~/src/rust-foo] $ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling regex v0.1.29
   Compiling Test v0.0.1 (file:///Users/tamird/src/rust-foo)
src/bin/main.rs:6:18: 6:35 error: expected function, found `core::result::Result<regex::re::Regex, regex::parse::Error>`
src/bin/main.rs:6     let result = datere(testdate1); // apply regex
                                   ^~~~~~~~~~~~~~~~~
error: aborting due to previous error
Could not compile `Test`.

To learn more, run the command again with --verbose.
