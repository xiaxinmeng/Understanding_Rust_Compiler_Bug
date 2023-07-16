
thread 'rustc' panicked at 'byte index 79 is not a char boundary; it is inside 'ä' (bytes 78..80) of `struct TestStruct {
    my_str: String
}

fn main() {
    let x = TestStruct("ä");
}
`', src/libcore/str/mod.rs:2034:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0 (eae3437df 2019-08-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `structs-1`.

To learn more, run the command again with --verbose.
