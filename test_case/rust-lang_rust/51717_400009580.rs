plain
[00:08:30] [RUSTC-TIMING] rustc_asan test:false 0.140
[00:08:30] [RUSTC-TIMING] rustc_tsan test:false 0.138
[00:08:31] [RUSTC-TIMING] rustc_msan test:false 0.109
[00:08:31] [RUSTC-TIMING] panic_unwind test:false 0.286
[00:08:36] warning: this feature has been stable since 1.28.0. Attribute no longer needed
[00:08:36]    --> libstd/lib.rs:337:21
[00:08:36]     |
[00:08:36] 337 |             feature(global_allocator))]
[00:08:36]     |
[00:08:36]     |
[00:08:36]     = note: #[warn(stable_features)] on by default
[00:08:44] [RUSTC-TIMING] std test:false 13.642
[00:08:44]     Finished release [optimized] target(s) in 46.82s
[00:08:44] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:08:44] travis_fold:end:stage0-std
---
[00:09:29] [RUSTC-TIMING] error_chain test:false 1.006
[00:09:29] [RUSTC-TIMING] xz2 test:false 0.823
[00:09:37] [RUSTC-TIMING] clap test:false 34.037
[00:09:37]    Compiling installer v0.0.0 (file:///checkout/src/tools/rust-installer)
[00:09:37] error: lint unused_doc_comment has been renamed to unused_doc_comments
[00:09:37]   --> tools/rust-installer/src/lib.rs:26:5
[00:09:37] 26 | /     error_chain!{
[00:09:37] 27 | |         foreign_links {
[00:09:37] 27 | |         foreign_links {
[00:09:37] 28 | |             Io(::std::io::Error);
[00:09:37] 29 | |             StripPrefix(::std::path::StripPrefixError);
[00:09:37] 30 | |             WalkDir(::walkdir::Error);
[00:09:37] 32 | |     }
[00:09:37]    | |_____^
[00:09:37]    |
[00:09:37]    |
[00:09:37]    = note: `-D renamed-and-removed-lints` implied by `-D warnings`
[00:09:37] 
[00:09:37] 
[00:09:37] error: lint unused_doc_comment has been renamed to unused_doc_comments
[00:09:37]   --> tools/rust-installer/src/lib.rs:26:5
[00:09:37] 26 | /     error_chain!{
[00:09:37] 27 | |         foreign_links {
[00:09:37] 27 | |         foreign_links {
[00:09:37] 28 | |             Io(::std::io::Error);
[00:09:37] 29 | |             StripPrefix(::std::path::StripPrefixError);
[00:09:37] 30 | |             WalkDir(::walkdir::Error);
[00:09:37] 32 | |     }
[00:09:37]    | |_____^
[00:09:37]    |
[00:09:37]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:09:37]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:09:37] 
[00:09:37] error: lint unused_doc_comment has been renamed to unused_doc_comments
[00:09:37]   --> tools/rust-installer/src/lib.rs:26:5
[00:09:37] 26 | /     error_chain!{
[00:09:37] 27 | |         foreign_links {
[00:09:37] 27 | |         foreign_links {
[00:09:37] 28 | |             Io(::std::io::Error);
[00:09:37] 29 | |             StripPrefix(::std::path::StripPrefixError);
[00:09:37] 30 | |             WalkDir(::walkdir::Error);
[00:09:37] 32 | |     }
[00:09:37]    | |_____^
[00:09:37]    |
[00:09:37]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:09:37]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:09:37] 
[00:09:37] error: lint unused_doc_comment has been renamed to unused_doc_comments
[00:09:37]   --> tools/rust-installer/src/lib.rs:26:5
[00:09:37] 26 | /     error_chain!{
[00:09:37] 27 | |         foreign_links {
[00:09:37] 27 | |         foreign_links {
[00:09:37] 28 | |             Io(::std::io::Error);
[00:09:37] 29 | |             StripPrefix(::std::path::StripPrefixError);
[00:09:37] 30 | |             WalkDir(::walkdir::Error);
[00:09:37] 32 | |     }
[00:09:37]    | |_____^
[00:09:37]    |
[00:09:37]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:09:37]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:09:37] 
[00:09:38] error: aborting due to 4 previous errors
[00:09:38] 
[00:09:38] [RUSTC-TIMING] installer test:false 0.327
[00:09:38] error: Could not compile `installer`.
[00:09:38] 
[00:09:38] Caused by:
[00:09:38]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name installer tools/rust-installer/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=79b26711e46dfe03 -C extra-filename=-79b26711e46dfe03 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps --extern clap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libclap-7119df03666f5ed3.rlib --extern error_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/liberror_chain-258cc1527e5f4ead.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libflate2-6cb5b65c68ad4173.rlib --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/librayon-c037d3fb037d75f6.rlib --extern tar=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtar-7cb0f036d782f810.rlib --extern walkdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libwalkdir-29d660ed8698803f.rlib --extern xz2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libxz2-bfee124a30acf342.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/build/backtrace-sys-665ec07f5372f8b9/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/build/miniz-sys-b2da0adee48ee564/out -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/build/lzma-sys-90504555ddbe4295/out/lib` (exit code: 101)
[00:09:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rust-installer/Cargo.toml" "--features" "" "--message-format" "json"
[00:09:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:09:38] Build completed unsuccessfully in 0:06:24
travis_time:end:27359d8d:start=1529942698405373841,finish=1529943276837702719,duration=578432328878

---
travis_time:end:0797a4cc:start=1529943277625083109,finish=1529943277633839276,duration=8756167
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02f775d0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0413c67a
$ dmesg | grep -i kill
