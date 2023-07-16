plain
[00:39:28]    Compiling ucd-util v0.1.1
[00:39:28]    Compiling pulldown-cmark v0.1.2
[00:39:29]    Compiling utf8-ranges v1.0.0
[00:39:29]    Compiling bitflags v0.9.1
[00:39:29] thread 'main' panicked at 'byte index 8544 is not a char boundary; it is inside '☃' (bytes 8542..8545) of `/*!
[00:39:29] Crate `utf8-ranges` converts ranges of Unicode scalar values to equivalent
[00:39:29] ranges of UTF-8 bytes. This is useful for constructing byte based automatons
[00:39:29] that need to embed UTF-8 decoding.
[00:39:29] 
[00:39:29] See the documentation on the `Utf8Sequences` iterator for more d`[...]', libcore/str/mod.rs:2111:5
[00:39:29] 
[00:39:29] error: internal compiler error: unexpected panic
[00:39:29] 
[00:39:29] 
[00:39:29] note: the compiler unexpectedly panicked. this is a bug.
[00:39:29] 
[00:39:29] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:39:29] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:39:29] 
[00:39:29] 
[00:39:29] note: compiler flags: -C opt-level=3 -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:39:29] 
[00:39:29] note: some of the compiler flags provided by cargo are hidden
[00:39:29] 
[00:39:29] error: Could not compile `utf8-ranges`.
nu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
79448 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
79444 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
76768 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
