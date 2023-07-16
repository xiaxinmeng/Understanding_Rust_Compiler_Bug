plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:37:46] 
[00:37:46] running 2117 tests
[00:38:03] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FFF.FFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFiF
[00:38:50] FFFF.FFFFFFFFFFFFFFFFFFFFF.FF......FFF..FF.FFF.....F.F.FFFFF.FFFiFFF..F.FFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:39:48] FFFFFFFFFFFFFFFFFFFFFFFFFFF.F.FFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:40:48] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:41:44] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFF
[00:42:31] FFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FF
[00:43:17] FFFFFFFFFFFFFFFFF.FFFFFFFFFF.FFFFFFFFFFFFFFFFF.F.FFFF.FFFFFFFFFF.FFFFF.FFFFFFFFFFF.F..FFFFFFFFFFFFFF
[00:44:01] FFFFFFFFF.F..FFFFFFFFFFFF..FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:44:48] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFiFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:45:31] FFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:46:17] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:47:04] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:47:55] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:48:35] FFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF..FFFFFFFFFFF..FFFFFFFFFFFF..F.FFFFFFFFFFFFFF.F..F.F
[00:49:19] F.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.
[00:50:02] FF.FF.....FF..FFFFF...F..FFFFFFFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFiFFFFFFF.
[00:50:14] FFFFFFFFFFFFFFFFF.FFFFFF.F.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFiFFFFFFFFFFFFFFFFFF
[00:50:14] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:50:14] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:50:14] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:50:14] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFiFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
[00:50:15] FFFFFFFFFFFFiFFFF
[00:50:15] 
[00:50:15] ---- [ui] ui/E0661.rs stdout ----
[00:50:15] 
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/E0661.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0661/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0661/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"output operand constraint lacks '=' or '+'","code":{"code":"E0661","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/E0661.rs","byte_start":543,"byte_end":546,"line_start":17,"line_end":17,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"    asm!(\"nop\" : \"r\"(a));","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0661]: output operand constraint lacks '=' or '+'\n  --> /checkout/src/test/ui/E0661.rs:17:18\n   |\nLL |     asm!(\"nop\" : \"r\"(a));\n   |                  ^^^\n\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/E0661.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:15] 
[00:50:15] ---- [ui] ui/E0662.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/E0662.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0662/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0662/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"input operand constraint contains '='","code":{"code":"E0662","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/E0662.rs","byte_start":563,"byte_end":570,"line_start":18,"line_end":18,"column_start":12,"column_end":19,"is_primary":true,"text":[{"text":"         : \"=test\"(\"a\") //~ ERROR E0662","highlight_start":12,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0662]: input operand constraint contains '='\n  --> /checkout/src/test/ui/E0662.rs:18:12\n   |\nLL |          : \"=test\"(\"a\") //~ ERROR E0662\n   |            ^^^^^^^\n\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/E0662.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/E0660.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/E0660.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0660/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0660/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"malformed inline assembly","code":{"code":"E0660","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/E0660.rs","byte_start":530,"byte_end":548,"line_start":17,"line_end":17,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    asm!(\"nop\" \"nop\");","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0660]: malformed inline assembly\n  --> /checkout/src/test/ui/E0660.rs:17:5\n   |\nLL |     asm!(\"nop\" \"nop\");\n   |     ^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:15] {"message":"malformed inline assembly","code":{"code":"E0660","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/E0660.rs","byte_start":574,"byte_end":602,"line_start":19,"line_end":19,"column_start":5,"column_end":33,"is_primary":true,"text":[{"text":"    asm!(\"nop\" \"nop\" : \"=r\"(a));","highlight_start":5,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0660]: malformed inline assembly\n  --> /checkout/src/test/ui/E0660.rs:19:5\n   |\nLL |     asm!(\"nop\" \"nop\" : \"=r\"(a));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/E0660.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/E0508.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/E0508.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0508/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0508/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"cannot move out of type `[NonCopy; 1]`, a non-copy array","code":{"code":"E0508","explanation":"\nA value was moved out of a non-copy fixed-size array.\n\nExample of erroneous code:\n\n