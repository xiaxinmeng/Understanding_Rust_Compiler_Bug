plain
---- compile_test stdout ----

error: failed to compile fixed code
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/bytes_nth.fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/bytes_nth.stage-id" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-84b0775b3c81c97f.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f0962f37786a4888.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-cd40a3b060be150c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b396e06d7d6d4d75.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-4cd0b4140eeb9c8d.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/bytes_nth.stage-id.aux"
stdout:
------------------------------------------

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
{"message":"unused borrow that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/bytes_nth.fixed","byte_start":163,"byte_end":183,"line_start":9,"line_end":9,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    &s.as_bytes().get(3);","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D unused-must-use` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unused borrow that must be used\n  --> tests/ui/bytes_nth.fixed:9:5\n   |\nLL |     &s.as_bytes().get(3);\n   |     ^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D unused-must-use` implied by `-D warnings`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
