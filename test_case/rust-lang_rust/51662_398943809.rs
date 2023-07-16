plain
[00:09:51]    Compiling error-chain v0.11.0
[00:09:52] [RUSTC-TIMING] error_chain test:false 0.796
[00:10:01] [RUSTC-TIMING] clap test:false 38.273
[00:10:01]    Compiling installer v0.0.0 (file:///checkout/src/tools/rust-installer)
[00:10:01] error: lint unused_doc_comment has been renamed to unused_doc_comments
[00:10:01]   --> tools/rust-installer/src/lib.rs:26:5
[00:10:01] 26 | /     error_chain!{
[00:10:01] 27 | |         foreign_links {
[00:10:01] 27 | |         foreign_links {
[00:10:01] 28 | |             Io(::std::io::Error);
[00:10:01] 29 | |             StripPrefix(::std::path::StripPrefixError);
[00:10:01] 30 | |             WalkDir(::walkdir::Error);
[00:10:01] 32 | |     }
[00:10:01]    | |_____^
[00:10:01]    |
[00:10:01]    |
[00:10:01]    = note: `-D renamed-and-removed-lints` implied by `-D warnings`
[00:10:01] 
[00:10:01] 
[00:10:01] error: lint unused_doc_comment has been renamed to unused_doc_comments
[00:10:01]   --> tools/rust-installer/src/lib.rs:26:5
[00:10:01] 26 | /     error_chain!{
[00:10:01] 27 | |         foreign_links {
[00:10:01] 27 | |         foreign_links {
[00:10:01] 28 | |             Io(::std::io::Error);
[00:10:01] 29 | |             StripPrefix(::std::path::StripPrefixError);
[00:10:01] 30 | |             WalkDir(::walkdir::Error);
[00:10:01] 32 | |     }
[00:10:01]    | |_____^
[00:10:01]    |
[00:10:01]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:10:01]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:10:01] 
[00:10:01] error: lint unused_doc_comment has been renamed to unused_doc_comments
[00:10:01]   --> tools/rust-installer/src/lib.rs:26:5
[00:10:01] 26 | /     error_chain!{
[00:10:01] 27 | |         foreign_links {
[00:10:01] 27 | |         foreign_links {
[00:10:01] 28 | |             Io(::std::io::Error);
[00:10:01] 29 | |             StripPrefix(::std::path::StripPrefixError);
[00:10:01] 30 | |             WalkDir(::walkdir::Error);
[00:10:01] 32 | |     }
[00:10:01]    | |_____^
[00:10:01]    |
[00:10:01]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:10:01]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:10:01] 
[00:10:01] error: lint unused_doc_comment has been renamed to unused_doc_comments
[00:10:01]   --> tools/rust-installer/src/lib.rs:26:5
[00:10:01] 26 | /     error_chain!{
[00:10:01] 27 | |         foreign_links {
[00:10:01] 27 | |         foreign_links {
[00:10:01] 28 | |             Io(::std::io::Error);
[00:10:01] 29 | |             StripPrefix(::std::path::StripPrefixError);
[00:10:01] 30 | |             WalkDir(::walkdir::Error);
[00:10:01] 32 | |     }
[00:10:01]    | |_____^
[00:10:01]    |
[00:10:01]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:10:01]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:10:01] 
[00:10:02] error: aborting due to 4 previous errors
[00:10:02] 
[00:10:02] [RUSTC-TIMING] installer test:false 0.377
[00:10:02] error: Could not compile `installer`.
[00:10:02] 
[00:10:02] Caused by:
[00:10:02]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name installer tools/rust-installer/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=282df450198d29ea -C extra-filename=-282df450198d29ea --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps --extern tar=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtar-b0fc252bbbcbe34b.rlib --extern clap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libclap-8ae07bd506a60ca2.rlib --extern xz2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libxz2-96fe9467e0277c19.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libflate2-a9da7703d4f87d9d.rlib --extern error_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/liberror_chain-3cf24a04c929ca46.rlib --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/librayon-0ba38b9c4cb97a2a.rlib --extern walkdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libwalkdir-3d73f1661928dc40.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/build/backtrace-sys-b694aac2c244ef25/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/build/miniz-sys-0a0159574092a14e/out -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/build/lzma-sys-33f83d14bc6bc00c/out/lib` (exit code: 101)
[00:10:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rust-installer/Cargo.toml" "--features" "" "--message-format" "json"
[00:10:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:10:02] Build completed unsuccessfully in 0:06:43
travis_time:end:16683fb0:start=1529542274271784341,finish=1529542876570360898,duration=602298576557

---
travis_time:end:1656ad5d:start=1529542877200516321,finish=1529542877207242976,duration=6726655
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0af96ac7
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:28a524a2
$ dmesg | grep -i kill
