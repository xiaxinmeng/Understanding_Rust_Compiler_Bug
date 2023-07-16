 sh
$ cargo +nightly build
# ...
$ cargo +stable build -p cfg-if -v
   Compiling cfg-if v0.1.0
     Running `rustc /home/alex/.cargo/registry/src/github.com-1ecc6299db9ec823/cfg-if-0.1.0/src/lib.rs --crate-name cfg_if --crate-type lib -g -C metadata=72c1f992b13d5087 -C extra-filename=-72c1f992b13d5087 --out-dir /home/alex/code/cargo/wut/debug/deps --emit=dep-info,link -L dependency=/home/alex/code/cargo/wut/debug/deps --cap-lints allow`
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

thread 'rustc' panicked at 'index out of bounds: the len is 382871 but the index is 382871', ../src/libserialize/leb128.rs:46
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: Could not compile `cfg-if`.

Caused by:
  process didn't exit successfully: `rustc /home/alex/.cargo/registry/src/github.com-1ecc6299db9ec823/cfg-if-0.1.0/src/lib.rs --crate-name cfg_if --crate-type lib -g -C metadata=72c1f992b13d5087 -C extra-filename=-72c1f992b13d5087 --out-dir /home/alex/code/cargo/wut/debug/deps --emit=dep-info,link -L dependency=/home/alex/code/cargo/wut/debug/deps --cap-lints allow` (exit code: 101)
