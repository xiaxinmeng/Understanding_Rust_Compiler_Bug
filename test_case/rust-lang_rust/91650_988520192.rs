plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 11fb21fd0e4c42490d42f1baf6bc51516e5dc5f5 and 45b53b9d9ba81d984bfc5f1c43745fd29d53d5fb
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestY5lnzi/portable-simd.stage-id.stderr
To only update this specific test, also pass `--test-args portable-simd.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/portable-simd.rs" "-L" "/tmp/compiletestY5lnzi" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestY5lnzi/portable-simd.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestY5lnzi/portable-simd.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

---- compile_test stdout ----
diff of stderr:

 error: casting integer literal to `i32` is unnecessary
   --> $DIR/unnecessary_cast.rs:6:5
 LL |     1i32 as i32;
    |     ^^^^^^^^^^^ help: try: `1_i32`
    |
    |
    = note: `-D clippy::unnecessary-cast` implied by `-D warnings`
 
 error: casting float literal to `f32` is unnecessary
   --> $DIR/unnecessary_cast.rs:7:5
 LL |     1f32 as f32;
    |     ^^^^^^^^^^^ help: try: `1_f32`
 
 
 error: casting to the same type is unnecessary (`bool` -> `bool`)
   --> $DIR/unnecessary_cast.rs:8:5
 LL |     false as bool;
 LL |     false as bool;
    |     ^^^^^^^^^^^^^ help: try: `false`
-error: aborting due to 3 previous errors
-error: aborting due to 3 previous errors
+error: casting integer literal to `i8` is unnecessary
+  --> $DIR/unnecessary_cast.rs:25:5
+LL |     1 as std::os::raw::c_char;
+   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `1_i8`
+
+error: aborting due to 4 previous errors
---
To only update this specific test, also pass `--test-args unnecessary_cast.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unnecessary_cast.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/unnecessary_cast.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/unnecessary_cast.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
error: test failed, to rerun pass '--test compile-test'
------------------------------------------
{"message":"casting integer literal to `i32` is unnecessary","code":{"code":"clippy::unnecessary_cast","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_cast.rs","byte_start":110,"byte_end":121,"line_start":6,"line_end":6,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    1i32 as i32;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::unnecessary-cast` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_cast.rs","byte_start":110,"byte_end":121,"line_start":6,"line_end":6,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    1i32 as i32;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":"1_i32","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: casting integer literal to `i32` is unnecessary\n  --> tests/ui/unnecessary_cast.rs:6:5\n   |\nLL |     1i32 as i32;\n   |     ^^^^^^^^^^^ help: try: `1_i32`\n   |\n   = note: `-D clippy::unnecessary-cast` implied by `-D warnings`\n\n"}
{"message":"casting float literal to `f32` is unnecessary","code":{"code":"clippy::unnecessary_cast","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_cast.rs","byte_start":127,"byte_end":138,"line_start":7,"line_end":7,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    1f32 as f32;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_cast.rs","byte_start":127,"byte_end":138,"line_start":7,"line_end":7,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    1f32 as f32;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":"1_f32","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: casting float literal to `f32` is unnecessary\n  --> tests/ui/unnecessary_cast.rs:7:5\n   |\nLL |     1f32 as f32;\n   |     ^^^^^^^^^^^ help: try: `1_f32`\n\n"}
{"message":"casting to the same type is unnecessary (`bool` -> `bool`)","code":{"code":"clippy::unnecessary_cast","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_cast.rs","byte_start":144,"byte_end":157,"line_start":8,"line_end":8,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    false as bool;","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_cast.rs","byte_start":144,"byte_end":157,"line_start":8,"line_end":8,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    false as bool;","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":"false","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: casting to the same type is unnecessary (`bool` -> `bool`)\n  --> tests/ui/unnecessary_cast.rs:8:5\n   |\nLL |     false as bool;\n   |     ^^^^^^^^^^^^^ help: try: `false`\n\n"}
{"message":"casting integer literal to `i8` is unnecessary","code":{"code":"clippy::unnecessary_cast","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_cast.rs","byte_start":478,"byte_end":503,"line_start":25,"line_end":25,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    1 as std::os::raw::c_char;","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_cast.rs","byte_start":478,"byte_end":503,"line_start":25,"line_end":25,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    1 as std::os::raw::c_char;","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":"1_i8","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: casting integer literal to `i8` is unnecessary\n  --> tests/ui/unnecessary_cast.rs:25:5\n   |\nLL |     1 as std::os::raw::c_char;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `1_i8`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
