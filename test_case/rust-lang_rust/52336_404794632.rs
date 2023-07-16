plain
[00:43:21]    Compiling failure_derive v0.1.1
[00:43:25]    Compiling failure v0.1.1
[00:43:25]    Compiling rustfix v0.3.1
[00:43:27]    Compiling compiletest v0.0.0 (file:///checkout/src/tools/compiletest)
[00:43:27] error: trait objects without an explicit `dyn` are deprecated
[00:43:27]    --> tools/compiletest/src/header.rs:413:61
[00:43:27]     |
[00:43:27] 413 | fn iter_header(testfile: &Path, cfg: Option<&str>, it: &mut FnMut(&str)) {
[00:43:27]     |                                                             ^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(&str)`
[00:43:27]     |
[00:43:27]     = note: requested on the command line with `-D bare-trait-objects`
[00:43:27] 
[00:43:27] error: trait objects without an explicit `dyn` are deprecated
[00:43:27]   --> tools/compiletest/src/read2.rs:48:20
[00:43:27]    |
[00:43:27] 48 |         data: &mut FnMut(bool, &mut Vec<u8>, bool),
[00:43:27]    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(bool, &mut Vec<u8>, bool)`
[00:43:28] error: aborting due to 2 previous errors
[00:43:28] 
[00:43:28] error: Could not compile `compiletest`.
[00:43:28] 
[00:43:28] 
[00:43:28] Caused by:
[00:43:28]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name compiletest tools/compiletest/src/main.rs --color always --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 -C metadata=e61127a869a0112d -C extra-filename=-e61127a869a0112d --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps --extern diff=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libdiff-dc42cc1fc85d825b.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libenv_logger-cd53db7d36bd9ba4.rlib --extern filetime=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libfiletime-be37fe52ca647811.rlib --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libgetopts-84ce83e489b7e5a5.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/liblibc-ed5e56cb6a1a6a91.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/liblog-49caa0603b85251c.rlib --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libregex-4e38f3f16fc8f863.rlib --extern rustfix=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/librustfix-91ca9ba11d59b0f8.rlib --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libserde-df8ef05a77663569.rlib --extern serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps/libserde_derive-af6fb9239c0d1b55.so --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-075a641e614564a8.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/build/backtrace-sys-e7051d4409bf3a37/out` (exit code: 101)
[00:43:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/compiletest/Cargo.toml" "--features" "" "--message-format" "json"
[00:43:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:28] Build completed unsuccessfully in 0:00:22
[00:43:28] Build completed unsuccessfully in 0:00:22
[00:43:28] make: *** [check] Error 1
[00:43:28] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bef9ecf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
