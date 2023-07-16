plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ff0e14829e1806ca0d4226595f7fdf3e8658758f and 1d5653bbb38882daa03048137dc5922a4959ef63
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
 error: trait objects without an explicit `dyn` are deprecated
error: test failed, to rerun pass '--test compile-test'
   --> $DIR/ice-3969.rs:25:17
    |
 LL |     for<'a> Dst<A + 'a>: Sized,
    |                 ^^^^^^ help: use `dyn`: `dyn A + 'a`
    |
    = note: `-D bare-trait-objects` implied by `-D warnings`
    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
 
-error: aborting due to 3 previous errors
+error: aborting due to previous error
---
To only update this specific test, also pass `--test-args crashes/ice-3969.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-3969.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-3969.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-31d96b843c92ffee.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-0285a3716fdfa0ac.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-dead5f2b179ae6e1.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0d9270169d1e3a9.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-738a932898af8104.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-432c9b3871214358.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bbe41a872bc8e443.so" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-3969.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":590,"byte_end":596,"line_start":25,"line_end":25,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"    for<'a> Dst<A + 'a>: Sized,","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":590,"byte_end":596,"line_start":25,"line_end":25,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"    for<'a> Dst<A + 'a>: Sized,","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":"dyn A + 'a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/crashes/ice-3969.rs:25:17\n   |\nLL |     for<'a> Dst<A + 'a>: Sized,\n   |                 ^^^^^^ help: use `dyn`: `dyn A + 'a`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!\n   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.0/src/lib.rs:105:22
