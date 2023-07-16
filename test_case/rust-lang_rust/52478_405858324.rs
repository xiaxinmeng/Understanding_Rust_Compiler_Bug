
$ cargo new minimaltestcase
     Created binary (application) `minimaltestcase` project
$ cd minimaltestcase/
$ cargo add proptest
      Adding proptest v0.8.3 to dependencies
$ cargo tarpaulin
Building project
error: internal compiler error: librustc/traits/trans/mod.rs:68: Encountered error `Unimplemented` selecting `Binder(<core::char::DecodeUtf16<<std::vec::Vec<u16> as core::iter::IntoIterator>::IntoIter> as arbitrary::traits::Arbitrary>)` during trans

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.27.1 (5f2b325f6 2018-07-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C relocation-model=dynamic-no-pic -C link-dead-code -C opt-level=0 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

Error during run
$ rustc --version
rustc 1.27.1 (5f2b325f6 2018-07-07)
$ cargo tarpaulin  --version
cargo-tarpaulin version: 0.6.4
