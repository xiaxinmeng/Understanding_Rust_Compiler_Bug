
[01:23:56]    Compiling clippy_lints v0.0.170 (file:///Users/travis/build/rust-lang/rust/src/tools/clippy/clippy_lints)
[01:23:58] error[E0432]: unresolved import `rustc::mir::transform`
[01:23:58]   --> src/tools/clippy/clippy_lints/src/utils/mod.rs:12:17
[01:23:58]    |
[01:23:58] 12 | use rustc::mir::transform::MirSource;
[01:23:58]    |                 ^^^^^^^^^ Could not find `transform` in `mir`
[01:23:58] 
[01:24:02] error[E0432]: unresolved import `rustc::mir::transform`
[01:24:02]   --> src/tools/clippy/clippy_lints/src/utils/mod.rs:12:17
[01:24:02]    |
[01:24:02] 12 | use rustc::mir::transform::MirSource;
[01:24:02]    |                 ^^^^^^^^^ Could not find `transform` in `mir`
[01:24:02] 
[01:24:02] error: aborting due to previous error
[01:24:02] 
[01:24:02] error: Could not compile `clippy_lints`.
[01:24:02] warning: build failed, waiting for other jobs to finish...
[01:24:06] error: aborting due to previous error
[01:24:06] 
[01:24:06] error: Could not compile `clippy_lints`.
