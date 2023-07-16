plain
Cloning into 'rust-toolstate'...
{"nomicon":"test-pass","rustfmt":"test-pass","embedded-book":"test-pass","rustbook":"test-fail","edition-guide":"test-pass","miri":"test-pass","book":"test-pass","rust-by-example":"test-pass","cargo-miri":"test-fail","rls":"test-pass","reference":"test-pass"}[master 623f8ee] (linux CI update)
 1 file changed, 1 insertion(+)
To https://github.com/rust-lang-nursery/rust-toolstate
   b79cde7..623f8ee  master -> master
Build completed successfully in 0:00:04
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
[TIMING] Assemble { target_compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 0.000
[TIMING] StartupObjects { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.000
---
---- compile_test stdout ----

error: failed to compile fixed code
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/match_expr_like_matches_macro.fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7d596e70088d0ccb/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7d596e70088d0ccb/out/test_build_base/match_expr_like_matches_macro.stage-id" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-e3c044d770c3edb5.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3e5755171965ca17.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-45a3ec2898545ae5.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-efc540c81be69f24.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f8eb452cc9665d35.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7d596e70088d0ccb/out/test_build_base/match_expr_like_matches_macro.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unnecessary trailing semicolon","code":{"code":"redundant_semicolons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_expr_like_matches_macro.fixed","byte_start":674,"byte_end":675,"line_start":42,"line_end":42,"column_start":6,"column_end":7,"is_primary":true,"text":[{"text":"    };","highlight_start":6,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D redundant-semicolons` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove this semicolon","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_expr_like_matches_macro.fixed","byte_start":674,"byte_end":675,"line_start":42,"line_end":42,"column_start":6,"column_end":7,"is_primary":true,"text":[{"text":"    };","highlight_start":6,"highlight_end":7}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary trailing semicolon\n  --> tests/ui/match_expr_like_matches_macro.fixed:42:6\n   |\nLL |     };\n   |      ^ help: remove this semicolon\n   |\n   = note: `-D redundant-semicolons` implied by `-D warnings`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/lib.rs:105:22
