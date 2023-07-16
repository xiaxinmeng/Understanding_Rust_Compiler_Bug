plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 2451f42c1deb9379d5e8e5fa86b0bf857ae048ec and f6d7ad93468b2c2e8a638888986f0fd514938f4e
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
---- compile_test stdout ----

error: failed to compile fixed code
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/if_let_some_result.fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/if_let_some_result.stage-id" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-a4851abe566518b2.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-a05ee8fc41b4648a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/if_let_some_result.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"function is never used: `str_to_int`","code":{"code":"dead_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_let_some_result.fixed","byte_start":57,"byte_end":67,"line_start":5,"line_end":5,"column_start":4,"column_end":14,"is_primary":true,"text":[{"text":"fn str_to_int(x: &str) -> i32 {","highlight_start":4,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D dead-code` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: function is never used: `str_to_int`\n  --> tests/ui/if_let_some_result.fixed:5:4\n   |\nLL | fn str_to_int(x: &str) -> i32 {\n   |    ^^^^^^^^^^\n   |\n   = note: `-D dead-code` implied by `-D warnings`\n\n"}
{"message":"function is never used: `str_to_int_ok`","code":{"code":"dead_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_let_some_result.fixed","byte_start":138,"byte_end":151,"line_start":9,"line_end":9,"column_start":4,"column_end":17,"is_primary":true,"text":[{"text":"fn str_to_int_ok(x: &str) -> i32 {","highlight_start":4,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: function is never used: `str_to_int_ok`\n  --> tests/ui/if_let_some_result.fixed:9:4\n   |\nLL | fn str_to_int_ok(x: &str) -> i32 {\n   |    ^^^^^^^^^^^^^\n\n"}
error: test failed, to rerun pass '--test compile-test'
{"message":"function is never used: `strange_some_no_else`","code":{"code":"dead_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_let_some_result.fixed","byte_start":239,"byte_end":259,"line_start":14,"line_end":14,"column_start":4,"column_end":24,"is_primary":true,"text":[{"text":"fn strange_some_no_else(x: &str) -> i32 {","highlight_start":4,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: function is never used: `strange_some_no_else`\n  --> tests/ui/if_let_some_result.fixed:14:4\n   |\nLL | fn strange_some_no_else(x: &str) -> i32 {\n   |    ^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"function is never used: `negative`","code":{"code":"dead_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_let_some_result.fixed","byte_start":386,"byte_end":394,"line_start":23,"line_end":23,"column_start":4,"column_end":12,"is_primary":true,"text":[{"text":"fn negative() {","highlight_start":4,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: function is never used: `negative`\n  --> tests/ui/if_let_some_result.fixed:23:4\n   |\nLL | fn negative() {\n   |    ^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
