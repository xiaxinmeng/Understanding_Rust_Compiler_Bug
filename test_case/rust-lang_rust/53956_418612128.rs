
[havvy@rust-nightly:~/scratchspace]$ cargo new --bin include_self
     Created binary (application) `include_self` project

[havvy@rust-nightly:~/scratchspace]$ cd include_self/

[havvy@rust-nightly:~/scratchspace/include_self]$ ls
Cargo.toml  src

[havvy@rust-nightly:~/scratchspace/include_self]$ sublime .

// Replace main.rs with the file in the opening comment.

[havvy@rust-nightly:~/scratchspace/include_self]$ cargo run
   Compiling include_self v0.1.0 (file:///home/havvy/scratchspace/include_self)                                                                                                                                      
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/include_self`
Byte count: 170

[havvy@rust-nightly:~/scratchspace/include_self]$ cd src

[havvy@rust-nightly:~/scratchspace/include_self/src]$ touch main.rs

[havvy@rust-nightly:~/scratchspace/include_self/src]$ cargo run
   Compiling include_self v0.1.0 (file:///home/havvy/scratchspace/include_self)                                                                                                                                      
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 2', /checkout/src/libcore/slice/mod.rs:2046:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.30.0-nightly (9395f0af7 2018-09-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `include_self`.

To learn more, run the command again with --verbose.
