plain
[00:05:36] 
[00:05:36] error[E0412]: cannot find type `Initializer` in this scope
[00:05:36]    --> libstd/sys/unix/fd.rs:275:37
[00:05:36]     |
[00:05:36] 275 |     unsafe fn initializer(&self) -> Initializer {
[00:05:36] help: possible candidate is found in another module, you can import it into scope
[00:05:36]     |
[00:05:36] 13  | use io::Initializer;
[00:05:36]     |
---
[00:05:38] For more information about an error, try `rustc --explain E0412`.
[00:05:38] error: Could not compile `std`.
[00:05:38] 
[00:05:38] Caused by:
[00:05:38]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=c29a53bc81e5db39 -C extra-filename=-c29a53bc81e5db39 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-eb136989b0a4d15b.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-3dc5fad39cb0df5f.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-21bc9e10c694662b.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknowKtravis_fold:end:after_failure.5
travis_time:start:16ee12d0
$ dmesg | grep -i kill
[   10.868734] init: failsafe main process (1092) killed by TERM signal
travis_time:end:16ee12d0:start=1536205020589319801,finish=1536205020601053446,duration=11733645
