plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:018c2e38
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:50:47]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:50:55] error: found removed `do catch` syntax
[00:50:55]     --> libsyntax/parse/parser.rs:2084:40
[00:50:55]      |
[00:50:55] 2084 |                 let args: PResult<_> = do catch {
[00:50:55]      |
[00:50:55]      = help: Following RFC #2388, the new non-placeholder syntax is `try`
[00:50:55] 
[00:50:55] 
[00:50:56] error: unused import: `AngleBracketedArgs`
[00:50:56]    |
[00:50:56]    |
[00:50:56] 12 | use ast::{AngleBracketedArgs, ParenthesisedArgs, AttrStyle, BareFnTy};
[00:50:56]    |
[00:50:56]    = note: `-D unused-imports` implied by `-D warnings`
[00:50:56] 
[00:51:01] error: aborting due to 2 previous errors
[00:51:01] error: aborting due to 2 previous errors
[00:51:01] 
[00:51:01] error: Could not compile `syntax`.
[00:51:01] 
[00:51:01] Caused by:
[00:51:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=fe98e136868ea706 -C extra-filename=-fe98e136868ea706 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-541dc60b4e0c385f.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-74bdb6b2e9e1e397.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-39f86272178ed5ea.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-bb4b062c87975588.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-6eec6ff47e663d3c.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-7bf17d97e11a050f.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-718ee1ef825f3d33.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-718ee1ef825f3d33.rlib --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-f826bb5dac483ea6.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-94cc528051b60d6a.so` (exit code: 1)
[00:51:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:51:01] expected success, got: exit code: 101
[00:51:01] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:51:01] travis_fold:end:stage1-rustc

[00:51:01] travis_time:end:stage1-rustc:start=1536865004652936446,finish=1536865071351379012,duration=66698442566

---
travis_time:end:1a074488:start=1536865072677230432,finish=1536865072692484845,duration=15254413
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08d11f40
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e8cc2d5
travis_time:start:0e8cc2d5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01493e02
$ dmesg | grep -i kill
