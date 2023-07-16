plain
[00:38:58]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:38:58]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:38:58]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:38:58]  Documenting std v0.0.0 (file:///checkout/src/libstd)
[00:38:59] error: internal compiler error: librustc_metadata/decoder.rs:638: impossible case reached
[00:38:59] thread '<unnamed>' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
[00:38:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:38:59] 
[00:38:59] 
[00:38:59] error: Unrecognized option: 'crate-version'
[00:38:59] 
[00:38:59] error: Could not document `alloc`.
[00:38:59] Caused by:
[00:38:59] Caused by:
[00:38:59]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name alloc liballoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-3f2813246f4f3b6c.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-f918368761ce09e6.rmeta` (exit code: 101)
[00:39:07] warning: [flat_map] cannot be resolved, ignoring it...
[00:39:07] 
[00:39:08] warning: [Pin] cannot be resolved, ignoring it...
[00:39:08] 
[00:39:08] 
[00:39:09] error: internal compiler error: librustc_metadata/decoder.rs:638: impossible case reached
[00:39:09] thread '<unnamed>' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
[00:39:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:39:09] 
[00:39:09] 
[00:39:09] error: Unrecognized option: 'crate-version'
[00:39:09] 
[00:39:09] error: Could not document `std`.
[00:39:09] Caused by:
[00:39:09] Caused by:
[00:39:09]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name std libstd/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-73e0cb59ca1930ef.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-622805fe98ea6633.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-6a62fe92ffb8d189.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-da236c2d40b44dff.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-82e5816dcdcd394f.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-21e9f7cf5a4f2348.rmeta --extern rustc_got: exit code: 101
[00:39:09] 
[00:39:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:39:09] Build completed unsuccessfully in 0:05:46
[00:39:09] Build completed unsuccessfully in 0:05:46
[00:39:09] make: *** [all] Error 1
[00:39:09] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09f69453
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
