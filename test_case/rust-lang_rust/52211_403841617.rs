plain
[00:41:08]     Checking alloc v0.0.0 (file:///checkout/src/liballoc)
[00:41:08]     Checking std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:41:08]  Documenting alloc v0.0.0 (file:///checkout/src/liballoc)
[00:41:09]  Documenting std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:41:09] thread '<unnamed>' panicked at 'Unexpected item {:?} in path {:?} path', librustdoc/clean/mod.rs:4341:26
[00:41:09] 
[00:41:09] error: internal compiler error: unexpected panic
[00:41:09] 
[00:41:09] 
[00:41:09] error: Unrecognized option: 'crate-version'
[00:41:09] 
[00:41:09] error: Could not document `std_unicode`.
[00:41:09] Caused by:
[00:41:09] Caused by:
[00:41:09]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name std_unicode libstd_unicode/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-9cb094b550f36de1.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-b15f1ff96a8f614d.rmeta` (exit code: 101)
[00:41:09] warning: build failed, waiting for other jobs to finish...
[00:41:10] thread '<unnamed>' panicked at 'Unexpected item {:?} in path {:?} path', librustdoc/clean/mod.rs:4341:26
[00:41:10] 
[00:41:10] error: internal compiler error: unexpected panic
[00:41:10] 
[00:41:10] 
[00:41:10] error: Unrecognized option: 'crate-version'
[00:41:10] 
[00:41:10] error: Could not document `alloc`.
[00:41:10] Caused by:
[00:41:10] Caused by:
[00:41:10]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name alloc liballoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-9cb094b550f36de1.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-b15f1ff96a8f614d.rmeta` (exit code: 101)
[00:41:13] error: build failed
[00:41:13] 
[00:41:13] 
[00:41:13] 
[00:41:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--no-deps" "-p" "alloc" "-p" "core" "-p" "std" "-p" "std_unicode"
[00:41:13] 
[00:41:13] 
[00:41:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:41:13] Build completed unsuccessfully in 0:04:53
[00:41:13] Build completed unsuccessfully in 0:04:53
[00:41:13] Makefile:28: recipe for target 'all' failed
[00:41:13] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07560b4c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
