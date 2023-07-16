plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:0fdabd83e1d3faaa8e9cfd7c00031e3a92997344)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---

---- compile_test stdout ----
diff of stderr:

+error[E0080]: evaluation of `main::{constant#3}` failed
+   |
+   |
+LL |     const { &ARR[idx4()] }; // This should be linted, since `suppress-restriction-lint-in-const` default is false.
+   |              ^^^^^^^^^^^ index out of bounds: the length is 2 but the index is 4
+note: erroneous constant used
+  --> $DIR/indexing_slicing_index.rs:31:5
+   |
+   |
+LL |     const { &ARR[idx4()] }; // This should be linted, since `suppress-restriction-lint-in-const` default is false.
+
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:9:20
    |
    |
 LL | const REF: &i32 = &ARR[idx()]; // This should be linted, since `suppress-restriction-lint-in-const` default is false.
    |
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
    = note: the suggestion might not be applicable in constant blocks
    = note: `-D clippy::indexing-slicing` implied by `-D warnings`
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:10:24
    |
    |
 LL | const REF_ERR: &i32 = &ARR[idx4()]; // Ok, let rustc handle const contexts.
error: test failed, to rerun pass `--test compile-test`
    |
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
    = note: the suggestion might not be applicable in constant blocks
-
-error[E0080]: evaluation of `main::{constant#3}` failed
-   |
-   |
-LL |     const { &ARR[idx4()] }; // This should be linted, since `suppress-restriction-lint-in-const` default is false.
-   |              ^^^^^^^^^^^ index out of bounds: the length is 2 but the index is 4
-note: erroneous constant used
-  --> $DIR/indexing_slicing_index.rs:31:5
-   |
-   |
-LL |     const { &ARR[idx4()] }; // This should be linted, since `suppress-restriction-lint-in-const` default is false.
 
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:22:5
    |
    |
 LL |     x[index];
    |     ^^^^^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:30:14
    |
    |
 LL |     const { &ARR[idx()] }; // This should be linted, since `suppress-restriction-lint-in-const` default is false.
    |
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
    = note: the suggestion might not be applicable in constant blocks
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:31:14
    |
    |
 LL |     const { &ARR[idx4()] }; // This should be linted, since `suppress-restriction-lint-in-const` default is false.
    |
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
    = note: the suggestion might not be applicable in constant blocks
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:38:5
    |
 LL |     v[0];
 LL |     v[0];
    |     ^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:39:5
    |
 LL |     v[10];
 LL |     v[10];
    |     ^^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:40:5
    |
    |
 LL |     v[1 << 3];
    |
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:46:5
    |
 LL |     v[N];
 LL |     v[N];
    |     ^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:47:5
    |
 LL |     v[M];
 LL |     v[M];
    |     ^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
 error[E0080]: evaluation of constant value failed
   --> $DIR/indexing_slicing_index.rs:10:24
    |
    |
 LL | const REF_ERR: &i32 = &ARR[idx4()]; // Ok, let rustc handle const contexts.
    |                        ^^^^^^^^^^^ index out of bounds: the length is 2 but the index is 4
 error: aborting due to 12 previous errors
 
 For more information about this error, try `rustc --explain E0080`.
 
---
To only update this specific test, also pass `--test-args indexing_slicing_index.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/indexing_slicing_index.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/indexing_slicing_index.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-504e25b9109e124f.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ff2c2aaa2e33fd1f.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-a015ca6e4940c670.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/indexing_slicing_index.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"evaluation of `main::{constant#3}` failed","code":{"code":"E0080","explanation":"A constant value failed to get evaluated.\n\nErroneous code example:\n\n