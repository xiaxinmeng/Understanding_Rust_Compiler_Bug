plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:03821121
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:22:42]     Checking core v0.0.0 (/checkout/src/libcore)
[01:22:59]     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[01:22:59]     Checking compiler_builtins v0.1.5
[01:22:59]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966788', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294960413', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294960413', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966788', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966903', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966903', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966903', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966835', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966903', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966903', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966903', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966788', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966903', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966903', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966903', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966903', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966903', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966788', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966910', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 4294966910', /rustc/dc7f5dbbca74c71942899f41bd1cde5d243cbd6f/src/libcore/slice/mod.rs:2539:10
[01:23:00] error: internal compiler error: unexpected panic
[01:23:00] 
[01:23:00] error: Unrecognized option: 'markdown-css'
[01:23:00] 
[01:23:00] 
[01:23:00] error: Could not document `alloc`.
[01:23:00] 
[01:23:00] Caused by:
[01:23:00]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name alloc src/liballoc/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-3cfd1c4d36aee1de.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-c62f8bd170e1e48c.rmeta` (exit code: 1)
[01:23:00] 
[01:23:00] 
[01:23:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "alloc" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--index-page" "/checkout/src/doc/index.md"
[01:23:00] 
[01:23:00] 
[01:23:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:23:00] Build completed unsuccessfully in 1:15:51
---
travis_time:end:26f1d324:start=1553617284944611495,finish=1553617284960783483,duration=16171988
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b26f61e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21a03c4a
travis_time:start:21a03c4a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:27845eeb
$ dmesg | grep -i kill
