plain
[00:05:55]    Compiling flate2 v1.0.1
[00:05:56]    Compiling backtrace v0.3.9
[00:06:01]    Compiling rustc-rayon v0.1.1
[00:06:07]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:06:07] error: unused imports: `HashMap`, `HashSet`
[00:06:07]   --> librustc_data_structures/fx.rs:11:24
[00:06:07]    |
[00:06:07] 11 | use std::collections::{HashMap, HashSet};
[00:06:07]    |                        ^^^^^^^  ^^^^^^^
[00:06:07]    = note: `-D unused-imports` implied by `-D warnings`
[00:06:07] 
[00:06:07] error: unused import: `std::default::Default`
[00:06:07]   --> librustc_data_structures/fx.rs:12:5
---
[00:06:07] 
j/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-e9d9c61ba194d513.rlib --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-a4d022068ecc6660.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-5068b78226e8859d.rlib --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-fe22e88b3128db30.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5615a04cb21f88e7.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5615a04cb21f88e7.rlib --extern stable_deref_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-662adc546a1ea690.rlib` (exit code: 101)
[00:06:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:08] expected success, got: exit code: 101
[00:06:08] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:06:08] travis_fold:end:stage0-rustc

[00:06:08] travis_time:end:stage0-rustc:start=1532201586766330033,finish=1532201634892338887,duration=48126008854


[00:06:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:08] Build completed unsuccessfully in 0:01:53
[00:06:08] make: *** [all] Error 1
[00:06:08] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:019671a5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:163cebc0:start=1532201635416787571,finish=1532201635423450894,duration=6663323
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09b1dbe0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0726cadf
$ cat ./obj/build/x86_64-unknown-lin
