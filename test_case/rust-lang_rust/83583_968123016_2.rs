
% cargo +nightly build
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s

% cargo +nightly test
   Doc-tests repro

running 1 test
test src/lib.rs - dummy (line 7) ... FAILED

failures:

---- src/lib.rs - dummy (line 7) stdout ----
error: cannot find type `Foo` in this scope
 --> src/lib.rs:12:8
  |
7 | struct Foo;
  |        ^^^ names from parent modules are not accessible without an explicit import
  |
  = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
