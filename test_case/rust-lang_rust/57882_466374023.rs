plain
[01:40:42]    --> src/tools/cargo/src/cargo/core/features.rs:153:1
[01:40:42]     |
[01:40:42] 153 | / /// A listing of all features in Cargo
[01:40:42] 154 | | ///
[01:40:42] 155 | | /// "look here"
[01:40:42] ...   |
[01:40:42] ...   |
[01:40:42] 162 | | /// character is translated to `-` when specified in the `cargo-features`
[01:40:42] 163 | | /// manifest entry in `Cargo.toml`.
[01:40:42]     | |___________________________________^
[01:40:42] 164 | / features! {
[01:40:42] 165 | |     pub struct Features {
[01:40:42] 166 | |
[01:40:42] 167 | |         // A dummy feature that doesn't actually gate anything, but it's used in
[01:40:42] 198 | |     }
[01:40:42] 199 | | }
[01:40:42] 199 | | }
[01:40:42]     | |_- rustdoc does not generate documentation for macro expansions
[01:40:42]     = note: #[warn(unused_doc_comments)] on by default
[01:40:42]     = note: #[warn(unused_doc_comments)] on by default
[01:40:42]     = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion
[01:43:39] [RUSTC-TIMING] cargo test:false 176.675
[01:44:00] [RUSTC-TIMING] cargo test:false 21.703
[01:44:00]     Finished release [optimized] target(s) in 8m 49s
[01:44:00] travis_fold:end:stage2-cargo
---
[01:44:28]    Compiling cargo v0.35.0 (/checkout/src/tools/cargo)
[01:44:28] warning: unused doc comment
[01:44:28]    --> src/tools/cargo/tests/testsuite/resolve.rs:17:1
[01:44:28]     |
[01:44:28] 17  | / /// NOTE: proptest is a form of fuzz testing. It generates random input and makes sure that
[01:44:28] 18  | | /// certain universal truths are upheld. Therefore, it can pass when there is a problem,
[01:44:28] 19  | | /// but if it fails then there really is something wrong. When testing something as
[01:44:28] 20  | | /// complicated as the resolver, the problems can be very subtle and hard to generate.
[01:44:28] 21  | | /// We have had a history of these tests only failing on PRs long after a bug is introduced.
[01:44:28] 22  | | /// If you have one of these test fail please report it on #6258,
[01:44:28] 23  | | /// and if you did not change the resolver then feel free to retry without concern.
[01:44:28] 24  | / proptest! {
[01:44:28] 24  | / proptest! {
[01:44:28] 25  | |     #![proptest_config(ProptestConfig {
[01:44:28] 26  | |         max_shrink_iters:
[01:44:28] 27  | |             if env::var("CI").is_ok() || !atty::is(atty::Stream::Stderr) {
[01:44:28] 235 | |     }
[01:44:28] 236 | | }
[01:44:28] 236 | | }
[01:44:28]     | |_- rustdoc does not generate documentation for macro expansions
[01:44:28]     = note: #[warn(unused_doc_comments)] on by default
[01:44:28]     = note: #[warn(unused_doc_comments)] on by default
[01:44:28]     = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion
[01:44:28] error: unused doc comment
[01:44:28]    --> src/tools/cargo/src/cargo/core/features.rs:153:1
[01:44:28]     |
[01:44:28] 153 | / /// A listing of all features in Cargo
[01:44:28] 153 | / /// A listing of all features in Cargo
[01:44:28] 154 | | ///
[01:44:28] 155 | | /// "look here"
[01:44:28] ...   |
[01:44:28] ...   |
[01:44:28] 162 | | /// character is translated to `-` when specified in the `cargo-features`
[01:44:28] 163 | | /// manifest entry in `Cargo.toml`.
[01:44:28]     | |___________________________________^
[01:44:28] 164 | / features! {
[01:44:28] 165 | |     pub struct Features {
[01:44:28] 166 | |
[01:44:28] 167 | |         // A dummy feature that doesn't actually gate anything, but it's used in
[01:44:28] 198 | |     }
[01:44:28] 199 | | }
[01:44:28] 199 | | }
[01:44:28]     | |_- rustdoc does not generate documentation for macro expansions
[01:44:28] note: lint level defined here
[01:44:28]    --> src/tools/cargo/src/cargo/lib.rs:1:24
[01:44:28]     |
[01:44:28] 1   | #![cfg_attr(test, deny(warnings))]
[01:44:28] 1   | #![cfg_attr(test, deny(warnings))]
[01:44:28]     |                        ^^^^^^^^
[01:44:28]     = note: #[deny(unused_doc_comments)] implied by #[deny(warnings)]
[01:44:28]     = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion
[01:44:37] error: aborting due to previous error
[01:44:37] 
[01:44:37] [RUSTC-TIMING] cargo test:true 8.760
[01:44:37] error: Could not compile `cargo`.
---
[01:45:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static"
[01:45:51] expected success, got: exit code: 101
[01:45:51] 
[01:45:51] 
[01:45:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:45:51] Build completed unsuccessfully in 0:25:16
[01:45:51] make: *** [check-aux] Error 1
[01:45:51] Makefile:50: recipe for target 'check-aux' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14c24a34
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 22 12:01:29 UTC 2019
---
travis_time:end:11fb7f82:start=1550836890804003568,finish=1550836890811849101,duration=7845533
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:021d868a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13ad734a
travis_time:start:13ad734a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16f372a5
$ dmesg | grep -i kill
