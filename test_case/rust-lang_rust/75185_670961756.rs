plain
   Compiling itertools v0.9.0
   Compiling unicode-normalization v0.1.12
   Compiling getopts v0.2.21
   Compiling rustc_lexer v0.1.0 (/checkout/src/librustc_lexer)
error[E0391]: cycle detected when determining which generic parameters are unused by `Pack::SHIFT`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/sharded-slab-0.0.9/src/lib.rs:605:5
    |
605 |     const SHIFT: usize = Self::Prev::SHIFT + Self::Prev::LEN;
    |
    |
    = note: ...which again requires determining which generic parameters are unused by `Pack::SHIFT`, completing the cycle
note: cycle used when determining which generic parameters are unused by `page::slot::<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/sharded-slab-0.0.9/src/page/slot.rs:51:1: 68:2>::LEN`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/sharded-slab-0.0.9/src/page/slot.rs:54:5
    |
54  |     const LEN: usize = (cfg::WIDTH - C::RESERVED_BITS) - Self::SHIFT;

   Compiling rustc_serialize v0.0.0 (/checkout/src/librustc_serialize)
error: aborting due to previous error


For more information about this error, try `rustc --explain E0391`.
error: could not compile `sharded-slab`.
To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:13:19
== clock drift check ==
  local time: Sat Aug  8 18:53:23 UTC 2020
  local time: Sat Aug  8 18:53:23 UTC 2020
  network time: Sat, 08 Aug 2020 18:53:23 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (5528) (python)
