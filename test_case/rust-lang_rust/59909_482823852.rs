
$ cargo clippy -- -D warnings
    Checking num-traits v0.2.2
    Checking unicode-width v0.1.4
    Checking bitflags v1.0.1
thread 'rustc' panicked at 'begin <= end (190 <= 0) when slicing `}
                        }
                    )+
                }
                let mut first = true;
                $(
                    if <$BitFlags as __BitFlags>::$Flag(self) {
                        if !first {
                            t`[...]', src/libcore/str/mod.rs:2014:5
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.34.0 (91856ed52 2019-04-10) running on x86_64-unknown-linux-gnu
note: compiler flags: -C debuginfo=2 --crate-type lib
note: some of the compiler flags provided by cargo are hidden
error: Could not compile `bitflags`.
warning: build failed, waiting for other jobs to finish...
error: build failed
The command "cargo clippy -- -D warnings" exited with 101.
