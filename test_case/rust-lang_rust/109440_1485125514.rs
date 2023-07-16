plain
[TIMING] tool::Tidy { compiler: Compiler { stage: 0, host: aarch64-unknown-linux-gnu }, target: aarch64-unknown-linux-gnu } -- 0.000
fmt check
tidy check
tidy: Skipping binary file check, read-only filesystem
##[error]tidy error: /checkout/compiler/rustc_attr/src/builtin.rs:230: \
Use a single space after dots in comments.
##[error]tidy error: /checkout/compiler/rustc_attr/src/builtin.rs:284: \
Use a single space after dots in comments.
tidy error: /checkout/tests/rustdoc/issue-108925.rs: too many trailing newlines (2)
some tidy checks failed
