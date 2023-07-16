plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 350cca3b6a89d08fe2a3309be5233e7c8a2274c9 and c16a3656a711a5670e9e9fbc5b7f5f8b40a06eac
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

+error: this operation will panic at runtime
+  --> $DIR/indexing_slicing_index.rs:23:5
+   |
+LL |     x[4]; // Ok, let rustc's `unconditional_panic` lint handle `usize` indexing on arrays.
+   |     ^^^^ index out of bounds: the length is 4 but the index is 4
+   = note: `#[deny(unconditional_panic)]` on by default
+
+error: this operation will panic at runtime
+  --> $DIR/indexing_slicing_index.rs:24:5
+  --> $DIR/indexing_slicing_index.rs:24:5
+   |
+LL |     x[1 << 3]; // Ok, let rustc's `unconditional_panic` lint handle `usize` indexing on arrays.
+   |     ^^^^^^^^^ index out of bounds: the length is 4 but the index is 8
+error: this operation will panic at runtime
+  --> $DIR/indexing_slicing_index.rs:29:5
+   |
+   |
+LL |     x[const { idx4() }]; // Ok, let rustc's `unconditional_panic` lint handle `usize` indexing on arrays.
+   |     ^^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 4 but the index is 4
+
 error[E0080]: evaluation of `main::{constant#3}` failed
    |
    |
 LL |     const { &ARR[idx4()] }; // Ok, let rustc handle const contexts.
    |              ^^^^^^^^^^^ index out of bounds: the length is 2 but the index is 4
 error[E0080]: erroneous constant used
   --> $DIR/indexing_slicing_index.rs:31:5
    |
    |
 LL |     const { &ARR[idx4()] }; // Ok, let rustc handle const contexts.
 
+error: this operation will panic at runtime
+  --> $DIR/indexing_slicing_index.rs:44:5
+   |
+   |
+LL |     x[N]; // Ok, let rustc's `unconditional_panic` lint handle `usize` indexing on arrays.
+   |     ^^^^ index out of bounds: the length is 4 but the index is 15
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:22:5
    |
 LL |     x[index];
 LL |     x[index];
    |     ^^^^^^^^
    |
    = note: `-D clippy::indexing-slicing` implied by `-D warnings`
    = help: consider using `.get(n)` or `.get_mut(n)` instead
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
-error: aborting due to 8 previous errors
+error: aborting due to 12 previous errors
 
 For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args indexing_slicing_index.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/indexing_slicing_index.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/indexing_slicing_index.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-68b3adac889f3bfe.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4b46e2e2788394f2.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-56bce9bcc023120a.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-eed8221ad604f845.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-4554cde6a1339e03.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c6aa3eacac0eeebb.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-e993ea424d40e3e9.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-14dbc812a1f5dba0.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/indexing_slicing_index.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":626,"byte_end":630,"line_start":23,"line_end":23,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    x[4]; // Ok, let rustc's `unconditional_panic` lint handle `usize` indexing on arrays.","highlight_start":5,"highlight_end":9}],"label":"index out of bounds: the length is 4 but the index is 4","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(unconditional_panic)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this operation will panic at runtime\n  --> tests/ui/indexing_slicing_index.rs:23:5\n   |\nLL |     x[4]; // Ok, let rustc's `unconditional_panic` lint handle `usize` indexing on arrays.\n   |     ^^^^ index out of bounds: the length is 4 but the index is 4\n   |\n   = note: `#[deny(unconditional_panic)]` on by default\n\n"}
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":717,"byte_end":726,"line_start":24,"line_end":24,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    x[1 << 3]; // Ok, let rustc's `unconditional_panic` lint handle `usize` indexing on arrays.","highlight_start":5,"highlight_end":14}],"label":"index out of bounds: the length is 4 but the index is 8","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will panic at runtime\n  --> tests/ui/indexing_slicing_index.rs:24:5\n   |\nLL |     x[1 << 3]; // Ok, let rustc's `unconditional_panic` lint handle `usize` indexing on arrays.\n   |     ^^^^^^^^^ index out of bounds: the length is 4 but the index is 8\n\n"}
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":960,"byte_end":979,"line_start":29,"line_end":29,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    x[const { idx4() }]; // Ok, let rustc's `unconditional_panic` lint handle `usize` indexing on arrays.","highlight_start":5,"highlight_end":24}],"label":"index out of bounds: the length is 4 but the index is 4","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will panic at runtime\n  --> tests/ui/indexing_slicing_index.rs:29:5\n   |\nLL |     x[const { idx4() }]; // Ok, let rustc's `unconditional_panic` lint handle `usize` indexing on arrays.\n   |     ^^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 4 but the index is 4\n\n"}
{"message":"evaluation of `main::{constant#3}` failed","code":{"code":"E0080","explanation":"A constant value failed to get evaluated.\n\nErroneous code example:\n\n