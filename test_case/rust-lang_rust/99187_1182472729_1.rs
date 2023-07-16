\n\nDelete the offending feature attribute.\n"},"level":"error","spans":[{"file_name":"tests/ui/derive.rs","byte_start":11,"byte_end":26,"line_start":1,"line_end":1,"column_start":12,"column_end":27,"is_primary":true,"text":[{"text":"#![feature(untagged_unions)]","highlight_start":12,"highlight_end":27}],"label":"feature has been removed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"unions with `Copy` and `ManuallyDrop` fields are stable; there is no intent to stabilize more","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0557]: feature has been removed\n  --> tests/ui/derive.rs:1:12\n   |\nLL | #![feature(untagged_unions)]\n   |            ^^^^^^^^^^^^^^^ feature has been removed\n   |\n   = note: unions with `Copy` and `ManuallyDrop` fields are stable; there is no intent to stabilize more\n\n"}
{"message":"For more information about this error, try `rustc --explain E0557`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0557`.\n"}

------------------------------------------

---
-   |     ^^
+LL | #![feature(untagged_unions)]
+   |            ^^^^^^^^^^^^^^^ feature has been removed
    |
-   = note: `-D clippy::no-effect` implied by `-D warnings`
+   = note: unions with `Copy` and `ManuallyDrop` fields are stable; there is no intent to stabilize more
-error: statement with no effect
-  --> $DIR/no_effect.rs:96:5
-   |
-LL |     s2;
---
-   |     ^^^^^^^^^
-
-error: statement with no effect
-  --> $DIR/no_effect.rs:99:5
-   |
-LL |     Struct { field: 0 };
-   |     ^^^^^^^^^^^^^^^^^^^^
-error: statement with no effect
-  --> $DIR/no_effect.rs:100:5
-   |
-   |
-LL |     Struct { ..s };
-   |     ^^^^^^^^^^^^^^^
-error: statement with no effect
-  --> $DIR/no_effect.rs:101:5
-   |
-   |
-LL |     Union { a: 0 };
-   |     ^^^^^^^^^^^^^^^
-error: statement with no effect
-  --> $DIR/no_effect.rs:102:5
-   |
-LL |     Enum::Tuple(0);
-LL |     Enum::Tuple(0);
-   |     ^^^^^^^^^^^^^^^
-
-error: statement with no effect
-  --> $DIR/no_effect.rs:103:5
-   |
-LL |     Enum::Struct { field: 0 };
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
-error: statement with no effect
-  --> $DIR/no_effect.rs:104:5
-   |
-LL |     5 + 6;
---
-   |     ^^^^^^^^^
-
-error: statement with no effect
-  --> $DIR/no_effect.rs:115:5
-   |
-LL |     [42, 55][1];
-   |     ^^^^^^^^^^^^
-error: statement with no effect
-  --> $DIR/no_effect.rs:116:5
-   |
-   |
-LL |     (42, 55).1;
-   |     ^^^^^^^^^^^
-error: statement with no effect
-  --> $DIR/no_effect.rs:117:5
-   |
-   |
-LL |     [42; 55];
-   |     ^^^^^^^^^
-error: statement with no effect
-  --> $DIR/no_effect.rs:118:5
-   |
-   |
-LL |     [42; 55][13];
-   |     ^^^^^^^^^^^^^
-error: statement with no effect
-  --> $DIR/no_effect.rs:120:5
-   |
-   |
-LL |     || x += 5;
-   |     ^^^^^^^^^^
-error: statement with no effect
-  --> $DIR/no_effect.rs:122:5
-   |
-   |
-LL |     FooString { s: s };
-   |     ^^^^^^^^^^^^^^^^^^^
-
-error: binding to `_` prefixed variable with no side-effect
-   |
-   |
-LL |     let _unused = 1;
-   |     ^^^^^^^^^^^^^^^^
-   |
-   = note: `-D clippy::no-effect-underscore-binding` implied by `-D warnings`
-
-error: binding to `_` prefixed variable with no side-effect
-   |
-   |
-LL |     let _penguin = || println!("Some helpful closure");
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: binding to `_` prefixed variable with no side-effect
-   |
-   |
-LL |     let _duck = Struct { field: 0 };
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: binding to `_` prefixed variable with no side-effect
-   |
-   |
-LL |     let _cat = [2, 4, 6, 8][2];
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
-error: aborting due to 30 previous errors
-
+For more information about this error, try `rustc --explain E0557`.
 
---
To only update this specific test, also pass `--test-args no_effect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/no_effect.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/no_effect.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0c795f7a8756f15a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-61ecef333190a996.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-ff530e80824c3a36.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-b92911696ae4394a.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-aceff80e643e9fe7.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-dcc59fbd39a40970.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7d13fa063f867ef0.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-46430df947b7dad0.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/no_effect.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"feature has been removed","code":{"code":"E0557","explanation":"A feature attribute named a feature that has been removed.\n\nErroneous code example:\n\n