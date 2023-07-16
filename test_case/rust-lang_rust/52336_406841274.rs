plain
[00:07:53]    Compiling xz2 v0.1.5
[00:07:54] [RUSTC-TIMING] xz2 test:false 0.533
[00:07:54] [RUSTC-TIMING] clap test:false 31.506
[00:07:54]    Compiling installer v0.0.0 (file:///checkout/src/tools/rust-installer)
[00:07:54] error: trait objects without an explicit `dyn` are deprecated
[00:07:54]   --> tools/rust-installer/src/lib.rs:27:5
[00:07:54] 27 | /     error_chain!{
[00:07:54] 28 | |         foreign_links {
[00:07:54] 28 | |         foreign_links {
[00:07:54] 29 | |             Io(::std::io::Error);
[00:07:54] 30 | |             StripPrefix(::std::path::StripPrefixError);
[00:07:54] 31 | |             WalkDir(::walkdir::Error);
[00:07:54] 33 | |     }
[00:07:54] 33 | |     }
[00:07:54]    | |_____^ help: use `dyn`: `dyn :: std :: error :: Error + Send + 'static`
[00:07:54]    |
[00:07:54]    = note: requested on the command line with `-D bare-trait-objects`
[00:07:54] 
[00:07:54] 
[00:07:54] error: trait objects without an explicit `dyn` are deprecated
[00:07:54]   --> tools/rust-installer/src/lib.rs:27:5
[00:07:54] 27 | /     error_chain!{
[00:07:54] 28 | |         foreign_links {
[00:07:54] 28 | |         foreign_links {
[00:07:54] 29 | |             Io(::std::io::Error);
[00:07:54] 30 | |             StripPrefix(::std::path::StripPrefixError);
[00:07:54] 31 | |             WalkDir(::walkdir::Error);
[00:07:54] 33 | |     }
[00:07:54] 33 | |     }
[00:07:54]    | |_____^ help: use `dyn`: `dyn :: std :: error :: Error + Send`
[00:07:54]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:07:54] 
[00:07:54] 
[00:07:54] error: trait objects without an explicit `dyn` are deprecated
[00:07:54]   --> tools/rust-installer/src/lib.rs:27:5
[00:07:54] 27 | /     error_chain!{
[00:07:54] 28 | |         foreign_links {
[00:07:54] 28 | |         foreign_links {
[00:07:54] 29 | |             Io(::std::io::Error);
[00:07:54] 30 | |             StripPrefix(::std::path::StripPrefixError);
[00:07:54] 31 | |             WalkDir(::walkdir::Error);
[00:07:54] 33 | |     }
[00:07:54] 33 | |     }
[00:07:54]    | |_____^ help: use `dyn`: `dyn (:: std :: error :: Error)`
[00:07:54]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:07:54] 
[00:07:55] error: aborting due to 3 previous errors
[00:07:55] 
[00:07:55] 
[00:07:55] [RUSTC-TIMING] installer test:false 0.314
[00:07:55] error: Could not compile `installer`.
[00:07:55] 
[00:07:55] Caused by:
[00:07:55]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name installer tools/rust-installer/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=2cd4cd204e007c60 -C extra-filename=-2cd4cd204e007c60 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps --extern clap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libclap-901738dea5f72eda.rlib --extern error_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/liberror_chain-16f6fc613f26e16e.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libflate2-2b513184d9d612c2.rlib --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/librayon-c921d099f6467eb0.rlib --extern tar=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libtar-d33b2f7548394009.rlib --extern walkdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libwalkdir-ade8f658253ac77b.rlib --extern xz2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libxz2-ff8dc798a2f4c9b7.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7205e6adb9d66e6b/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/build/miniz-sys-27571ee36438df44/out -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/build/lzma-sys-e7849bb6ccad13a2/out/lib` (exit code: 101)
[00:07:55] expected success, got: exit code: 101
[00:07:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:07:55] Build completed unsuccessfully in 0:05:19
travis_time:end:01d713d2:start=1532234953028800212,finish=1532235428393824661,duration=475365024449
---
travis_time:end:0a1e20ea:start=1532235428889116170,finish=1532235428897794974,duration=8678804
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b29d7f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1756d988
travis_time:start:1756d988
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00dc63b2
$ dmesg | grep -i kill
