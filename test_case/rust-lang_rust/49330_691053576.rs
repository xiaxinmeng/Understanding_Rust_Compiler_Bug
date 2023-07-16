console
❯ rustc +nightly --version -v
rustc 1.48.0-nightly (e2be5f568 2020-09-09)
binary: rustc
commit-hash: e2be5f568d1f60365b825530f5b5cb722460591b
commit-date: 2020-09-09
host: x86_64-pc-windows-msvc
release: 1.48.0-nightly
LLVM version: 11.0
❯ cargo +nightly build
<snip>
    Finished dev [unoptimized + debuginfo] target(s) in 8m 42s
❯ cargo +nightly build --release
<snip>
