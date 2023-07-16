plain

 error: this operation will panic at runtime
   --> $DIR/modulo_one.rs:11:5
    |
 LL |     i32::MIN % (-1); // also caught by rustc
-   |     ^^^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow
+   |     ^^^^^^^^^^^^^^^ attempt to compute `i32::MIN % -1_i32`, which would overflow
    = note: `#[deny(unconditional_panic)]` on by default
 
 error: this operation will panic at runtime
   --> $DIR/modulo_one.rs:21:5
   --> $DIR/modulo_one.rs:21:5
    |
 LL |     INT_MIN % NEG_ONE; // also caught by rustc
-   |     ^^^^^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow
+   |     ^^^^^^^^^^^^^^^^^ attempt to compute `i64::MIN % -1_i64`, which would overflow
 error: this operation will panic at runtime
   --> $DIR/modulo_one.rs:22:5
    |
    |
 LL |     INT_MIN % STATIC_NEG_ONE; // ONLY caught by rustc
error: test failed, to rerun pass `--test compile-test`
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow
+   |     ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i64::MIN % -1_i64`, which would overflow
 
 error: any number modulo 1 will be 0
    |
 LL |     10 % 1;
    |     ^^^^^^
    |
    |
    = note: `-D clippy::modulo-one` implied by `-D warnings`
 
 error: any number modulo -1 will panic/overflow or result in 0
    |
 LL |     10 % -1;
    |     ^^^^^^^
 
 
 error: any number modulo -1 will panic/overflow or result in 0
    |
    |
 LL |     i32::MIN % (-1); // also caught by rustc
 
 
 error: any number modulo 1 will be 0
    |
 LL |     2 % ONE;
    |     ^^^^^^^
 
 
 error: any number modulo -1 will panic/overflow or result in 0
    |
    |
 LL |     2 % NEG_ONE;
 
 
 error: any number modulo -1 will panic/overflow or result in 0
    |
    |
 LL |     INT_MIN % NEG_ONE; // also caught by rustc
 
 error: aborting due to 9 previous errors
 
 
---
To only update this specific test, also pass `--test-args modulo_one.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/modulo_one.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/modulo_one.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-25f7a610b7fafce5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/modulo_one.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":235,"byte_end":250,"line_start":11,"line_end":11,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    i32::MIN % (-1); // also caught by rustc","highlight_start":5,"highlight_end":20}],"label":"attempt to compute `i32::MIN % -1_i32`, which would overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(unconditional_panic)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this operation will panic at runtime\n  --> tests/ui/modulo_one.rs:11:5\n   |\nLL |     i32::MIN % (-1); // also caught by rustc\n   |     ^^^^^^^^^^^^^^^ attempt to compute `i32::MIN % -1_i32`, which would overflow\n   |\n   = note: `#[deny(unconditional_panic)]` on by default\n\n"}
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":495,"byte_end":512,"line_start":21,"line_end":21,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    INT_MIN % NEG_ONE; // also caught by rustc","highlight_start":5,"highlight_end":22}],"label":"attempt to compute `i64::MIN % -1_i64`, which would overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will panic at runtime\n  --> tests/ui/modulo_one.rs:21:5\n   |\nLL |     INT_MIN % NEG_ONE; // also caught by rustc\n   |     ^^^^^^^^^^^^^^^^^ attempt to compute `i64::MIN % -1_i64`, which would overflow\n\n"}
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":542,"byte_end":566,"line_start":22,"line_end":22,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    INT_MIN % STATIC_NEG_ONE; // ONLY caught by rustc","highlight_start":5,"highlight_end":29}],"label":"attempt to compute `i64::MIN % -1_i64`, which would overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will panic at runtime\n  --> tests/ui/modulo_one.rs:22:5\n   |\nLL |     INT_MIN % STATIC_NEG_ONE; // ONLY caught by rustc\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i64::MIN % -1_i64`, which would overflow\n\n"}
{"message":"any number modulo 1 will be 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":198,"byte_end":204,"line_start":8,"line_end":8,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    10 % 1;","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::modulo-one` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: any number modulo 1 will be 0\n  --> tests/ui/modulo_one.rs:8:5\n   |\nLL |     10 % 1;\n   |     ^^^^^^\n   |\n   = note: `-D clippy::modulo-one` implied by `-D warnings`\n\n"}
{"message":"any number modulo -1 will panic/overflow or result in 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":210,"byte_end":217,"line_start":9,"line_end":9,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    10 % -1;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo -1 will panic/overflow or result in 0\n  --> tests/ui/modulo_one.rs:9:5\n   |\nLL |     10 % -1;\n   |     ^^^^^^^\n\n"}
{"message":"any number modulo -1 will panic/overflow or result in 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":235,"byte_end":250,"line_start":11,"line_end":11,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    i32::MIN % (-1); // also caught by rustc","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo -1 will panic/overflow or result in 0\n  --> tests/ui/modulo_one.rs:11:5\n   |\nLL |     i32::MIN % (-1); // also caught by rustc\n   |     ^^^^^^^^^^^^^^^\n\n"}
{"message":"any number modulo 1 will be 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":377,"byte_end":384,"line_start":17,"line_end":17,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    2 % ONE;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo 1 will be 0\n  --> tests/ui/modulo_one.rs:17:5\n   |\nLL |     2 % ONE;\n   |     ^^^^^^^\n\n"}
{"message":"any number modulo -1 will panic/overflow or result in 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":432,"byte_end":443,"line_start":19,"line_end":19,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    2 % NEG_ONE;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo -1 will panic/overflow or result in 0\n  --> tests/ui/modulo_one.rs:19:5\n   |\nLL |     2 % NEG_ONE;\n   |     ^^^^^^^^^^^\n\n"}
{"message":"any number modulo -1 will panic/overflow or result in 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":495,"byte_end":512,"line_start":21,"line_end":21,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    INT_MIN % NEG_ONE; // also caught by rustc","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo -1 will panic/overflow or result in 0\n  --> tests/ui/modulo_one.rs:21:5\n   |\nLL |     INT_MIN % NEG_ONE; // also caught by rustc\n   |     ^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.10.0/src/lib.rs:111:22
