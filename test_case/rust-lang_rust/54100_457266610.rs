console
$ cargo +nightly-2019-01-23 rustdoc -p rust-orgmode-derive
    Checking unicode-xid v0.1.0
    Checking proc-macro2 v0.4.17
    Checking quote v0.6.8
    Checking syn v0.15.3
 Documenting rust-orgmode-derive v0.1.0 (/home/misdreavus/clones/rust-orgmode/rust-orgmode-derive)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.33.0-nightly (4c2be9c97 2019-01-22) running on x86_64-unknown-linux-gnu

error: Could not document `rust-orgmode-derive`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name rust_orgmode_derive rust-orgmode-derive/src/lib.rs --color always -o /home/misdreavus/clones/rust-orgmode/target/doc -L dependency=/home/misdreavus/clones/rust-orgmode/target/debug/deps --extern proc_macro2=/home/misdreavus/clones/rust-orgmode/target/debug/deps/libproc_macro2-7d0cb2f69b5e6e7d.rmeta --extern quote=/home/misdreavus/clones/rust-orgmode/target/debug/deps/libquote-9877f9e0a5ea1a89.rmeta --extern syn=/home/misdreavus/clones/rust-orgmode/target/debug/deps/libsyn-97f0413104af47b5.rmeta` (exit code: 1)
