plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:2efbfac82b677cf89dacd548a824f7377c8d4e77)
Complete job name: PR (x86_64-gnu-tools, false, 1, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 5dd0e1b7ae7bcddce28658487602e8a077737a3e and 990b4b6cd06bcf275b4d690e8425013fbe18cb8c
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

-error: the inner value of this ManuallyDrop will not be dropped
-  --> $DIR/undropped_manually_drops.rs:14:5
-   |
-LL |     drop(std::mem::ManuallyDrop::new(S));
-   |
-   |
-   = help: to drop a `ManuallyDrop<T>`, use std::mem::ManuallyDrop::drop
-   = note: `-D clippy::undropped-manually-drops` implied by `-D warnings`
-error: the inner value of this ManuallyDrop will not be dropped
error: test failed, to rerun pass `--test compile-test`
error: test failed, to rerun pass `--test compile-test`
+error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes
    |
 LL |     drop(manual1);
    |     ^^^^^^^^^^^^^
    |
    |
-   = help: to drop a `ManuallyDrop<T>`, use std::mem::ManuallyDrop::drop
+note: argument has type `std::mem::ManuallyDrop<S>`
+   |
+LL |     drop(manual1);
+   |          ^^^^^^^
+   |          ^^^^^^^
+   = note: `-D clippy::drop-non-drop` implied by `-D warnings`
-error: aborting due to 2 previous errors
+error: aborting due to previous error
 
 
---
To only update this specific test, also pass `--test-args undropped_manually_drops.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/undropped_manually_drops.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/undropped_manually_drops.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ff2c2aaa2e33fd1f.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-312563538883ae22.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/undropped_manually_drops.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes","code":{"code":"clippy::drop_non_drop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/undropped_manually_drops.rs","byte_start":458,"byte_end":471,"line_start":15,"line_end":15,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    drop(manual1);","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"argument has type `std::mem::ManuallyDrop<S>`","code":null,"level":"note","spans":[{"file_name":"tests/ui/undropped_manually_drops.rs","byte_start":463,"byte_end":470,"line_start":15,"line_end":15,"column_start":10,"column_end":17,"is_primary":true,"text":[{"text":"    drop(manual1);","highlight_start":10,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`-D clippy::drop-non-drop` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: call to `std::mem::drop` with a value that does not implement `Drop`. Dropping such a type only extends its contained lifetimes\n  --> tests/ui/undropped_manually_drops.rs:15:5\n   |\nLL |     drop(manual1);\n   |     ^^^^^^^^^^^^^\n   |\nnote: argument has type `std::mem::ManuallyDrop<S>`\n  --> tests/ui/undropped_manually_drops.rs:15:10\n   |\nLL |     drop(manual1);\n   |          ^^^^^^^\n   = note: `-D clippy::drop-non-drop` implied by `-D warnings`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.9.0/src/lib.rs:111:22
