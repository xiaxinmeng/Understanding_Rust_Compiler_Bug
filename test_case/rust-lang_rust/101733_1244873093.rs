plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 52e003a6e93940ae49cbfc806c72ed5b0217cf4e and 1f5c38668fbce2dc6ff45ae6783245569f1ecced
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
 error: using tabs in doc comments is not recommended
   --> $DIR/ice-5835.rs:3:10
    |
 LL |     /// 位    
-   |           ^^^^ help: consider using four spaces per tab
+   |           ^^^^ help: consider using four spaces per tab: `    `
    |
    = note: `-D clippy::tabs-in-doc-comments` implied by `-D warnings`
 error: aborting due to previous error
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-5835.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crashes/ice-5835.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-5835.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-5835.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-14dbc812a1f5dba0.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-4554cde6a1339e03.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c6aa3eacac0eeebb.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-47229815ed3188f9.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-eed8221ad604f845.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-a74d54f3750f2f75.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4b46e2e2788394f2.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-e92c80438a94fde5.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-5835.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"using tabs in doc comments is not recommended","code":{"code":"clippy::tabs_in_doc_comments","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-5835.rs","byte_start":45,"byte_end":46,"line_start":3,"line_end":3,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"    /// 位\t","highlight_start":10,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::tabs-in-doc-comments` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using four spaces per tab","code":null,"level":"help","spans":[{"file_name":"tests/ui/crashes/ice-5835.rs","byte_start":45,"byte_end":46,"line_start":3,"line_end":3,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"    /// 位\t","highlight_start":10,"highlight_end":11}],"label":null,"suggested_replacement":"    ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using tabs in doc comments is not recommended\n  --> tests/ui/crashes/ice-5835.rs:3:10\n   |\nLL |     /// 位    \n   |           ^^^^ help: consider using four spaces per tab: `    `\n   |\n   = note: `-D clippy::tabs-in-doc-comments` implied by `-D warnings`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: using tabs in doc comments is not recommended
   --> $DIR/tabs_in_doc_comments.rs:12:9
    |
 LL |     ///     - First String:
-   |         ^^^^ help: consider using four spaces per tab
+   |         ^^^^ help: consider using four spaces per tab: `    `
    |
    = note: `-D clippy::tabs-in-doc-comments` implied by `-D warnings`
 error: using tabs in doc comments is not recommended
   --> $DIR/tabs_in_doc_comments.rs:13:9
    |
    |
 LL |     ///         - needs to be inside here
-   |         ^^^^^^^^ help: consider using four spaces per tab
+   |         ^^^^^^^^ help: consider using four spaces per tab: `        `
 error: using tabs in doc comments is not recommended
   --> $DIR/tabs_in_doc_comments.rs:16:9
    |
 LL |     ///     - Second String:
 LL |     ///     - Second String:
-   |         ^^^^ help: consider using four spaces per tab
+   |         ^^^^ help: consider using four spaces per tab: `    `
 error: using tabs in doc comments is not recommended
   --> $DIR/tabs_in_doc_comments.rs:17:9
    |
    |
 LL |     ///         - needs to be inside here
-   |         ^^^^^^^^ help: consider using four spaces per tab
+   |         ^^^^^^^^ help: consider using four spaces per tab: `        `
 error: using tabs in doc comments is not recommended
   --> $DIR/tabs_in_doc_comments.rs:8:5
    |
 LL | ///     - first        one
 LL | ///     - first        one
-   |     ^^^^ help: consider using four spaces per tab
+   |     ^^^^ help: consider using four spaces per tab: `    `
 error: using tabs in doc comments is not recommended
   --> $DIR/tabs_in_doc_comments.rs:8:13
    |
 LL | ///     - first        one
 LL | ///     - first        one
-   |                ^^^^^^^^ help: consider using four spaces per tab
+   |                ^^^^^^^^ help: consider using four spaces per tab: `        `
 error: using tabs in doc comments is not recommended
   --> $DIR/tabs_in_doc_comments.rs:9:5
    |
 LL | ///     - second    one
 LL | ///     - second    one
-   |     ^^^^ help: consider using four spaces per tab
+   |     ^^^^ help: consider using four spaces per tab: `    `
 error: using tabs in doc comments is not recommended
   --> $DIR/tabs_in_doc_comments.rs:9:14
    |
 LL | ///     - second    one
 LL | ///     - second    one
-   |                 ^^^^ help: consider using four spaces per tab
+   |                 ^^^^ help: consider using four spaces per tab: `    `
 error: aborting due to 8 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/tabs_in_doc_comments.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args tabs_in_doc_comments.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/tabs_in_doc_comments.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/tabs_in_doc_comments.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-14dbc812a1f5dba0.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-4554cde6a1339e03.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c6aa3eacac0eeebb.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-47229815ed3188f9.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-eed8221ad604f845.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-a74d54f3750f2f75.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4b46e2e2788394f2.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-e92c80438a94fde5.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/tabs_in_doc_comments.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"using tabs in doc comments is not recommended","code":{"code":"clippy::tabs_in_doc_comments","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":190,"byte_end":191,"line_start":12,"line_end":12,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    /// \t- First String:","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::tabs-in-doc-comments` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using four spaces per tab","code":null,"level":"help","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":190,"byte_end":191,"line_start":12,"line_end":12,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    /// \t- First String:","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"    ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using tabs in doc comments is not recommended\n  --> tests/ui/tabs_in_doc_comments.rs:12:9\n   |\nLL |     ///     - First String:\n   |         ^^^^ help: consider using four spaces per tab: `    `\n   |\n   = note: `-D clippy::tabs-in-doc-comments` implied by `-D warnings`\n\n"}
{"message":"using tabs in doc comments is not recommended","code":{"code":"clippy::tabs_in_doc_comments","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":215,"byte_end":217,"line_start":13,"line_end":13,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    /// \t\t- needs to be inside here","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using four spaces per tab","code":null,"level":"help","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":215,"byte_end":217,"line_start":13,"line_end":13,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    /// \t\t- needs to be inside here","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"        ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using tabs in doc comments is not recommended\n  --> tests/ui/tabs_in_doc_comments.rs:13:9\n   |\nLL |     ///         - needs to be inside here\n   |         ^^^^^^^^ help: consider using four spaces per tab: `        `\n\n"}
{"message":"using tabs in doc comments is not recommended","code":{"code":"clippy::tabs_in_doc_comments","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":285,"byte_end":286,"line_start":16,"line_end":16,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    /// \t- Second String:","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using four spaces per tab","code":null,"level":"help","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":285,"byte_end":286,"line_start":16,"line_end":16,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    /// \t- Second String:","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"    ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using tabs in doc comments is not recommended\n  --> tests/ui/tabs_in_doc_comments.rs:16:9\n   |\nLL |     ///     - Second String:\n   |         ^^^^ help: consider using four spaces per tab: `    `\n\n"}
{"message":"using tabs in doc comments is not recommended","code":{"code":"clippy::tabs_in_doc_comments","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":311,"byte_end":313,"line_start":17,"line_end":17,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    /// \t\t- needs to be inside here","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using four spaces per tab","code":null,"level":"help","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":311,"byte_end":313,"line_start":17,"line_end":17,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    /// \t\t- needs to be inside here","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"        ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using tabs in doc comments is not recommended\n  --> tests/ui/tabs_in_doc_comments.rs:17:9\n   |\nLL |     ///         - needs to be inside here\n   |         ^^^^^^^^ help: consider using four spaces per tab: `        `\n\n"}
{"message":"using tabs in doc comments is not recommended","code":{"code":"clippy::tabs_in_doc_comments","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":116,"byte_end":117,"line_start":8,"line_end":8,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"/// \t- first\t\tone","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using four spaces per tab","code":null,"level":"help","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":116,"byte_end":117,"line_start":8,"line_end":8,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"/// \t- first\t\tone","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":"    ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using tabs in doc comments is not recommended\n  --> tests/ui/tabs_in_doc_comments.rs:8:5\n   |\nLL | ///     - first        one\n   |     ^^^^ help: consider using four spaces per tab: `    `\n\n"}
{"message":"using tabs in doc comments is not recommended","code":{"code":"clippy::tabs_in_doc_comments","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":124,"byte_end":126,"line_start":8,"line_end":8,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"/// \t- first\t\tone","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using four spaces per tab","code":null,"level":"help","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":124,"byte_end":126,"line_start":8,"line_end":8,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"/// \t- first\t\tone","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":"        ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using tabs in doc comments is not recommended\n  --> tests/ui/tabs_in_doc_comments.rs:8:13\n   |\nLL | ///     - first        one\n   |                ^^^^^^^^ help: consider using four spaces per tab: `        `\n\n"}
{"message":"using tabs in doc comments is not recommended","code":{"code":"clippy::tabs_in_doc_comments","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":134,"byte_end":135,"line_start":9,"line_end":9,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"/// \t- second\tone","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using four spaces per tab","code":null,"level":"help","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":134,"byte_end":135,"line_start":9,"line_end":9,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"/// \t- second\tone","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":"    ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using tabs in doc comments is not recommended\n  --> tests/ui/tabs_in_doc_comments.rs:9:5\n   |\nLL | ///     - second    one\n   |     ^^^^ help: consider using four spaces per tab: `    `\n\n"}
{"message":"using tabs in doc comments is not recommended","code":{"code":"clippy::tabs_in_doc_comments","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":143,"byte_end":144,"line_start":9,"line_end":9,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"/// \t- second\tone","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using four spaces per tab","code":null,"level":"help","spans":[{"file_name":"tests/ui/tabs_in_doc_comments.rs","byte_start":143,"byte_end":144,"line_start":9,"line_end":9,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"/// \t- second\tone","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":"    ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using tabs in doc comments is not recommended\n  --> tests/ui/tabs_in_doc_comments.rs:9:14\n   |\nLL | ///     - second    one\n   |                 ^^^^ help: consider using four spaces per tab: `    `\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22
