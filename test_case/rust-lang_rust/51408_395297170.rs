plain
[00:02:38]  Downloading cpp_demangle v0.2.9
[00:02:38]  Downloading object v0.9.0
[00:02:39]  Downloading intervaltree v0.2.3
[00:02:39]  Downloading lazycell v1.0.0
[00:02:39]  Downloading gimli v0.16.0
[00:02:39]  Downloading rustc-demangle v0.1.7
[00:02:39]  Downloading rustc-ap-rustc_cratesio_shim v147.0.0
[00:02:39]  Downloading proc-macro2 v0.4.6
[00:02:39]  Downloading chalk-macros v0.1.0
---
[00:02:41]  Downloading datafrog v0.1.0
[00:02:41]  Downloading rls-span v0.4.0
[00:02:41]  Downloading rls-data v0.16.0
[00:02:41]  Downloading rand v0.3.22
[00:02:42]  Downloading uuid v0.6.5
[00:02:42]  Downloading goblin v0.0.15
[00:02:42]  Downloading scroll v0.9.0
[00:02:42]  Downloading backtrace-sys v0.1.22
[00:02:42]  Downloading polonius-engine v0.5.0
[00:02:42]  Downloading either v1.5.0
[00:02:42]  Downloading log_settings v0.1.1
---
[00:02:58] Caused by:
[00:02:58]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name cfg_if /cargo/registry/src/github.com-1ecc6299db9ec823/cfg-if-0.1.2/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=540c185278ab8f78 -C extra-filename=-540c185278ab8f78 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cap-lints allow` (exit code: 101)
[00:02:58] warning: build failed, waiting for other jobs to finish...
[00:03:19] error: build failed
[00:03:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:19] expected success, got: exit code: 101
[00:03:19] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:19] travis_fold:end:stage0-std

[00:03:19] travis_time:end:stage0-std:start=1528349088372726257,finish=1528349109847452381,duration=21474726124


[00:03:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:19] Build completed unsuccessfully in 0:00:22
[00:03:19] Makefile:79: recipe for target 'tidy' failed
[00:03:19] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2e4054c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
