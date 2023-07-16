plain

 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:19:10
    |
 LL |     b || diverge();
    |
    |
    = note: `-D clippy::diverging-sub-expression` implied by `-D warnings`
 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:20:10
    |
    |
 LL |     b || A.foo();
 
 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:29:26
    |
---
 
 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:33:26
    |
 LL |             3 => true || diverge(),
 
 error: sub-expression diverges
-  --> $DIR/diverging_sub_expression.rs:36:30
-   |
-   |
-LL |                 _ => true || panic!("boo"),
-   |
-   = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
-
-error: sub-expression diverges
---
To only update this specific test, also pass `--test-args diverging_sub_expression.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/diverging_sub_expression.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/diverging_sub_expression.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-5e33bd3c9ca65da0.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/diverging_sub_expression.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":378,"byte_end":387,"line_start":19,"line_end":19,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"    b || diverge();","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::diverging-sub-expression` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:19:10\n   |\nLL |     b || diverge();\n   |          ^^^^^^^^^\n   |\n   = note: `-D clippy::diverging-sub-expression` implied by `-D warnings`\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":398,"byte_end":405,"line_start":20,"line_end":20,"column_start":10,"column_end":17,"is_primary":true,"text":[{"text":"    b || A.foo();","highlight_start":10,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:20:10\n   |\nLL |     b || A.foo();\n   |          ^^^^^^^\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":576,"byte_end":582,"line_start":29,"line_end":29,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"            6 => true || return,","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:29:26\n   |\nLL |             6 => true || return,\n   |                          ^^^^^^\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":609,"byte_end":617,"line_start":30,"line_end":30,"column_start":26,"column_end":34,"is_primary":true,"text":[{"text":"            7 => true || continue,","highlight_start":26,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:30:26\n   |\nLL |             7 => true || continue,\n   |                          ^^^^^^^^\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":696,"byte_end":705,"line_start":33,"line_end":33,"column_start":26,"column_end":35,"is_primary":true,"text":[{"text":"            3 => true || diverge(),","highlight_start":26,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:33:26\n   |\nLL |             3 => true || diverge(),\n   |                          ^^^^^^^^^\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":850,"byte_end":855,"line_start":38,"line_end":38,"column_start":26,"column_end":31,"is_primary":true,"text":[{"text":"            _ => true || break,","highlight_start":26,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:38:26\n   |\nLL |             _ => true || break,\n   |                          ^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.10.0/src/lib.rs:111:22
