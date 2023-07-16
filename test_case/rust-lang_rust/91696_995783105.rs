
[w@ww hyper]$ cargo c
    Checking log v0.4.14
    Checking futures-util v0.3.15
thread 'rustc' panicked at 'assertion failed: sentinel == STR_SENTINEL', /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/compiler/rustc_serialize/src/opaque.rs:669:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (0b42deacc 2021-12-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C link-arg=-fuse-ld=lld -C target-cpu=znver3 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `log`
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'assertion failed: sentinel == STR_SENTINEL', /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/compiler/rustc_serialize/src/opaque.rs:669:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (0b42deacc 2021-12-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C link-arg=-fuse-ld=lld -C target-cpu=znver3 --crate-type lib
