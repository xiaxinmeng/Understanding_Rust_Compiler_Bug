plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 96ddd32c4bfb1d78f0cd03eb068b1710a8cebeef and 8619074187e28cd4b0ae1e89d1d060ab79ea68ed
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error[E0080]: evaluation of `main::{constant#3}` failed
    |
error: test failed, to rerun pass `--test compile-test`
error: test failed, to rerun pass `--test compile-test`
 LL |     const { &ARR[idx4()] }; // Ok, let rustc handle const contexts.
    |              ^^^^^^^^^^^ index out of bounds: the length is 2 but the index is 4
+note: erroneous constant used
+  --> $DIR/indexing_slicing_index.rs:31:5
+   |
+   |
+LL |     const { &ARR[idx4()] }; // Ok, let rustc handle const contexts.
+
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:22:5
    |
    |
 LL |     x[index];
    |     ^^^^^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
    = note: `-D clippy::indexing-slicing` implied by `-D warnings`
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
 error: aborting due to 8 previous errors
 
 For more information about this error, try `rustc --explain E0080`.
 
---
To only update this specific test, also pass `--test-args indexing_slicing_index.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/indexing_slicing_index.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/indexing_slicing_index.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f474b2dfd384a595.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-4ace64e065702c69.so" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f2da228ff241ec97.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-f03750e49676bd17.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-5ac9fbce59ab185e.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/indexing_slicing_index.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"evaluation of `main::{constant#3}` failed","code":{"code":"E0080","explanation":"A constant value failed to get evaluated.\n\nErroneous code example:\n\n