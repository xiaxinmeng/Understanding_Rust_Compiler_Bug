plain
[00:05:25]    Compiling ena v0.9.2
[00:05:25]    Compiling jobserver v0.1.11
[00:05:27]    Compiling parking_lot_core v0.2.14
[00:05:27]    Compiling tempdir v0.3.7
[00:05:29] error[E0432]: unresolved import `LinkerFlavor`
[00:05:29]   --> librustc_target/spec/aarch64_unknown_openbsd.rs:11:5
[00:05:29] 11 | use LinkerFlavor;
[00:05:29] 11 | use LinkerFlavor;
[00:05:29]    |     ^^^^^^^^^^^^ no `LinkerFlavor` in the root
[00:05:29] 
[00:05:29] error[E0432]: unresolved import `target`
[00:05:29]   --> librustc_target/spec/aarch64_unknown_openbsd.rs:12:5
[00:05:29]    |
[00:05:29] 12 | use target::{Target, TargetResult};
[00:05:29]    |     ^^^^^^ Maybe a missing `extern crate target;`?
[00:05:29]    Compiling rustc_apfloat v0.0.0 (file:///checkout/src/librustc_apfloat)
[00:05:30]    Compiling rls-span v0.4.0
[00:05:31] error: aborting due to 2 previous errors
[00:05:31] 
[00:05:31] 
[00:05:31] For more information about this error, try `rustc --explain E0432`.
[00:05:31] error: Could not compile `rustc_target`.
[00:05:31] 
[00:05:31] Caused by:
[00:05:31]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_target librustc_target/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="jemalloc" -C metadata=aed9d8ab86b35123 -C extra-filename=-aed9d8ab86b35123 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-401bddd0d1809e53.rlib --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-a7895f6d3b02c33b.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-4f0866e958f59455.rlib` (exit code: 101)
Sat, 12 May 2018 08:06:31 GMT
travis_time:end:07ffbbee:start=1526112391244265537,finish=1526112391306706563,duration=62441026

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
