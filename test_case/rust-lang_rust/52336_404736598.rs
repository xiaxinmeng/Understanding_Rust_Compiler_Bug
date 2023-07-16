plain
[00:03:02]    Compiling syn v0.14.4
[00:03:14]    Compiling serde_json v1.0.22
[00:03:15]    Compiling serde_derive v1.0.69
[00:03:36]    Compiling tidy v0.1.0 (file:///checkout/src/tools/tidy)
[00:03:36] error: trait objects without an explicit `dyn` are deprecated
[00:03:36]    --> tools/tidy/src/features.rs:337:30
[00:03:36]     |
[00:03:36] 337 |                     mf: &mut FnMut(Result<(&str, Feature), &str>, &Path, usize)) {
[00:03:36]     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(Result<(&str, Feature), &str>, &Path, usize)`
[00:03:36]     |
[00:03:36]     = note: requested on the command line with `-D bare-trait-objects`
[00:03:36] 
[00:03:36] error: trait objects without an explicit `dyn` are deprecated
[00:03:36]   --> tools/tidy/src/lib.rs:84:42
[00:03:36]    |
[00:03:36] 84 | fn walk_many(paths: &[&Path], skip: &mut FnMut(&Path) -> bool, f: &mut FnMut(&Path)) {
[00:03:36]    |                                          ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(&Path) -> bool`
[00:03:36] 
[00:03:36] error: trait objects without an explicit `dyn` are deprecated
[00:03:36]   --> tools/tidy/src/lib.rs:84:72
[00:03:36]    |
[00:03:36] 84 | fn walk_many(paths: &[&Path], skip: &mut FnMut(&Path) -> bool, f: &mut FnMut(&Path)) {
[00:03:36]    |                                                                        ^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(&Path)`
[00:03:36] 
[00:03:36] error: trait objects without an explicit `dyn` are deprecated
[00:03:36]   --> tools/tidy/src/lib.rs:90:33
[00:03:36]    |
[00:03:36] 90 | fn walk(path: &Path, skip: &mut FnMut(&Path) -> bool, f: &mut FnMut(&Path)) {
[00:03:36]    |                                 ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(&Path) -> bool`
[00:03:36] 
[00:03:36] error: trait objects without an explicit `dyn` are deprecated
[00:03:36]   --> tools/tidy/src/lib.rs:90:63
[00:03:36]    |
[00:03:36] 90 | fn walk(path: &Path, skip: &mut FnMut(&Path) -> bool, f: &mut FnMut(&Path)) {
[00:03:36]    |                                                               ^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(&Path)`
[00:03:36] error: aborting due to 5 previous errors
[00:03:36] 
[00:03:36] 
[00:03:36] error: Could not compile `tidy`.
[00:03:36] Caused by:
[00:03:36] Caused by:
[00:03:36]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name tidy tools/tidy/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=9eb1600fedccd8e0 -C extra-filename=-9eb1600fedccd8e0 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libserde-df8ef05a77663569.rlib --extern serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps/libserde_derive-af6fb9239c0d1b55.so --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-075a641e614564a8.rlib` (exit code: 101)
[00:03:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--features" "" "--message-format" "json"
[00:03:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:36] Build completed unsuccessfully in 0:00:41
[00:03:36] Build completed unsuccessfully in 0:00:41
[00:03:36] Makefile:79: recipe for target 'tidy' failed
[00:03:36] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:023857b5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2ae6e1e0:start=1531461876108154850,finish=1531461876116575518,duration=8420668
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0808a7a8
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a22047d
$ dmesg | grep -i kill
