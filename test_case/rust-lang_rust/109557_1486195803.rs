plain
[TIMING] tool::Tidy { compiler: Compiler { stage: 0, host: aarch64-unknown-linux-gnu }, target: aarch64-unknown-linux-gnu } -- 0.000
fmt check
tidy check
tidy: Skipping binary file check, read-only filesystem
##[error]tidy error: /checkout/tests/ui/rfc-2632-const-trait-impl/const-drop-fail-2.rs:31: line longer than 100 chars
some tidy checks failed
