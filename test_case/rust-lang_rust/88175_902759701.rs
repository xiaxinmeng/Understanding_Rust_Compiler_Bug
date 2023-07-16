plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 521734787ecf80ff12df7ca5998f7ec0b3b7b2c9 and 0e5e218daad1665e37343ad00542780948c08abc
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
failures:

---- compile_test stdout ----
normalized stderr:
error: this `if` has identical blocks
  --> $DIR/ice-7410.rs:19:30
   |
LL |       if let Some(_) = Some(S) {
LL | |     } else {
   | |_____^
   |
   |
   = note: `#[deny(clippy::if_same_then_else)]` on by default
note: same as this
  --> $DIR/ice-7410.rs:20:12
   |
LL |       } else {
   |  ____________^
---
To only update this specific test, also pass `--test-args crashes/ice-7410.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/crashes/ice-7410.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/crashes/ice-7410.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-06539012b522c0cf.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-17d0421109b348d4.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-Clink-arg=-nostartfiles" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/crashes/ice-7410.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-7410.rs","byte_start":354,"byte_end":361,"line_start":19,"line_end":20,"column_start":30,"column_end":6,"is_primary":true,"text":[{"text":"    if let Some(_) = Some(S) {","highlight_start":30,"highlight_end":31},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(clippy::if_same_then_else)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/crashes/ice-7410.rs","byte_start":367,"byte_end":374,"line_start":20,"line_end":21,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/crashes/ice-7410.rs:19:30\n   |\nLL |       if let Some(_) = Some(S) {\n   |  ______________________________^\nLL | |     } else {\n   | |_____^\n   |\n   = note: `#[deny(clippy::if_same_then_else)]` on by default\nnote: same as this\n  --> tests/ui/crashes/ice-7410.rs:20:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |     }\n   | |_____^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
