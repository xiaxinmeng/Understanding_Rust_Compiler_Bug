plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f8abed9ed48bace6be0087bcd44ed534e239b8d8 and fd296a0bbe6ac22947b8cbf957a054c00e15bcbb
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
    |
 LL |             fn foo() {}
    |             ^^^^^^^^^^^
    |
    = note: `-D clippy::same-name-method` implied by `-D warnings`
 note: existing `foo` defined here
    |
 LL |             fn foo() {}
    |             ^^^^^^^^^^^
 
---
+  --> $DIR/same_name_method.rs:30:18
+   |
+LL |         #[derive(Clone)]
+   |                  ^^^^^
+   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
+error: method's name is the same as an existing method in a trait
   --> $DIR/same_name_method.rs:44:13
    |
 LL |             fn foo() {}
---
    |
 note: existing `foo` defined here
   --> $DIR/same_name_method.rs:61:9
    |
 LL |         impl T1 for S {}
 
 error: method's name is the same as an existing method in a trait
   --> $DIR/same_name_method.rs:70:13
    |
    |
 LL |             fn foo() {}
    |             ^^^^^^^^^^^
    |
 note: existing `foo` defined here
   --> $DIR/same_name_method.rs:73:9
    |
 LL |         impl T1 for S {}
-
-error: method's name is the same as an existing method in a trait
-  --> $DIR/same_name_method.rs:34:13
-   |
-   |
-LL |             fn clone() {}
-   |             ^^^^^^^^^^^^^
-   |
-note: existing `clone` defined here
-  --> $DIR/same_name_method.rs:30:18
-   |
-LL |         #[derive(Clone)]
-   |                  ^^^^^
-   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
 error: aborting due to 5 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/same_name_method.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args same_name_method.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/same_name_method.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/same_name_method.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/same_name_method.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"method's name is the same as an existing method in a trait","code":{"code":"clippy::same_name_method","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/same_name_method.rs","byte_start":256,"byte_end":267,"line_start":20,"line_end":20,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"            fn foo() {}","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::same-name-method` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"existing `foo` defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/same_name_method.rs","byte_start":315,"byte_end":326,"line_start":24,"line_end":24,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"            fn foo() {}","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: method's name is the same as an existing method in a trait\n  --> tests/ui/same_name_method.rs:20:13\n   |\nLL |             fn foo() {}\n   |             ^^^^^^^^^^^\n   |\n   = note: `-D clippy::same-name-method` implied by `-D warnings`\nnote: existing `foo` defined here\n  --> tests/ui/same_name_method.rs:24:13\n   |\nLL |             fn foo() {}\n   |             ^^^^^^^^^^^\n\n"}
{"message":"method's name is the same as an existing method in a trait","code":{"code":"clippy::same_name_method","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/same_name_method.rs","byte_start":440,"byte_end":453,"line_start":34,"line_end":34,"column_start":13,"column_end":26,"is_primary":true,"text":[{"text":"            fn clone() {}","highlight_start":13,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"existing `clone` defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/same_name_method.rs","byte_start":384,"byte_end":389,"line_start":30,"line_end":30,"column_start":18,"column_end":23,"is_primary":true,"text":[{"text":"        #[derive(Clone)]","highlight_start":18,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/same_name_method.rs","byte_start":384,"byte_end":389,"line_start":30,"line_end":30,"column_start":18,"column_end":23,"is_primary":false,"text":[{"text":"        #[derive(Clone)]","highlight_start":18,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(Clone)]","def_site_span":{"file_name":"/checkout/library/core/src/clone.rs","byte_start":4850,"byte_end":4909,"line_start":139,"line_end":141,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"pub macro Clone($item:item) {","highlight_start":1,"highlight_end":1},{"text":"    /* compiler built-in */","highlight_start":1,"highlight_end":1},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: method's name is the same as an existing method in a trait\n  --> tests/ui/same_name_method.rs:34:13\n   |\nLL |             fn clone() {}\n   |             ^^^^^^^^^^^^^\n   |\nnote: existing `clone` defined here\n  --> tests/ui/same_name_method.rs:30:18\n   |\nLL |         #[derive(Clone)]\n   |                  ^^^^^\n   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"method's name is the same as an existing method in a trait","code":{"code":"clippy::same_name_method","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/same_name_method.rs","byte_start":578,"byte_end":589,"line_start":44,"line_end":44,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"            fn foo() {}","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"existing `foo` defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/same_name_method.rs","byte_start":649,"byte_end":660,"line_start":48,"line_end":48,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"            fn foo() {}","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: method's name is the same as an existing method in a trait\n  --> tests/ui/same_name_method.rs:44:13\n   |\nLL |             fn foo() {}\n   |             ^^^^^^^^^^^\n   |\nnote: existing `foo` defined here\n  --> tests/ui/same_name_method.rs:48:13\n   |\nLL |             fn foo() {}\n   |             ^^^^^^^^^^^\n\n"}
{"message":"method's name is the same as an existing method in a trait","code":{"code":"clippy::same_name_method","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/same_name_method.rs","byte_start":775,"byte_end":786,"line_start":58,"line_end":58,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"            fn foo() {}","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"existing `foo` defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/same_name_method.rs","byte_start":806,"byte_end":822,"line_start":61,"line_end":61,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"        impl T1 for S {}","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: method's name is the same as an existing method in a trait\n  --> tests/ui/same_name_method.rs:58:13\n   |\nLL |             fn foo() {}\n   |             ^^^^^^^^^^^\n   |\nnote: existing `foo` defined here\n  --> tests/ui/same_name_method.rs:61:9\n   |\nLL |         impl T1 for S {}\n   |         ^^^^^^^^^^^^^^^^\n\n"}
{"message":"method's name is the same as an existing method in a trait","code":{"code":"clippy::same_name_method","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/same_name_method.rs","byte_start":943,"byte_end":954,"line_start":70,"line_end":70,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"            fn foo() {}","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"existing `foo` defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/same_name_method.rs","byte_start":974,"byte_end":990,"line_start":73,"line_end":73,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"        impl T1 for S {}","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: method's name is the same as an existing method in a trait\n  --> tests/ui/same_name_method.rs:70:13\n   |\nLL |             fn foo() {}\n   |             ^^^^^^^^^^^\n   |\nnote: existing `foo` defined here\n  --> tests/ui/same_name_method.rs:73:9\n   |\nLL |         impl T1 for S {}\n   |         ^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
