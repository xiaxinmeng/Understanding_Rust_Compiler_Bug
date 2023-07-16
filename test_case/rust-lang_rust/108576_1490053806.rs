plain
[TIMING] tool::Tidy { compiler: Compiler { stage: 0, host: i686-unknown-linux-gnu }, target: i686-unknown-linux-gnu } -- 0.000
fmt check
tidy check
tidy: Skipping binary file check, read-only filesystem
##[error]tidy error: /checkout/tests/rustdoc-ui/issue-105742.rs:9: trailing whitespace
##[error]tidy error: /checkout/tests/rustdoc-ui/issue-105742.rs:17: trailing whitespace
##[error]tidy error: /checkout/tests/rustdoc-ui/issue-105742.rs:25: trailing whitespace
##[error]tidy error: /checkout/tests/rustdoc-ui/issue-105742.rs:32: trailing whitespace
some tidy checks failed
