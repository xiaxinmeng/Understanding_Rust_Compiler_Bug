plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:45:32] 
[00:45:32] running 6774 tests
[00:45:34] ....................................................................................................
[00:45:37] ..............................................................F.....................................
[00:45:43] ....................................................................................................
[00:45:45] ....................................................................................................
[00:45:49] .................i..................................................................................
[00:45:54] ....................................................................................................
---
[00:51:58] failures:
[00:51:58] 
[00:51:58] ---- [ui] ui/bastion-of-the-turbofish.rs stdout ----
[00:51:58] 
[00:51:58] error: test compilation failed although it shouldn't!
[00:51:58] status: exit code: 1
[00:51:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/bastion-of-the-turbofish.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bastion-of-the-turbofish/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bastion-of-the-turbofish/auxiliary" "-A" "unused"
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] ------------------------------------------
[00:51:58] stderr:
[00:51:58] stderr:
[00:51:58] ------------------------------------------
[00:51:58] {"message":"expected type, found local variable `woe`","code":{"code":"E0573","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/bastion-of-the-turbofish.rs","byte_start":2179,"byte_end":2182,"line_start":45,"line_end":45,"column_start":31,"column_end":34,"is_primary":true,"text":[{"text":"    let _: (bool, bool) = (oh<woe, is>(me));","highlight_start":31,"highlight_end":34}],"label":"not a type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0573]: expected type, found local variable `woe`\n  --> /checkout/src/test/ui/bastion-of-the-turbofish.rs:45:31\n   |\nLL |     let _: (bool, bool) = (oh<woe, is>(me));\n   |                               ^^^ not a type\n\n"}
[00:51:58] {"message":"expected type, found local variable `is`","code":{"code":"E0573","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/bastion-of-the-turbofish.rs","byte_start":2184,"byte_end":2186,"line_start":45,"line_end":45,"column_start":36,"column_end":38,"is_primary":true,"text":[{"text":"    let _: (bool, bool) = (oh<woe, is>(me));","highlight_start":36,"highlight_end":38}],"label":"did you mean `i8`?","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0573]: expected type, found local variable `is`\n  --> /checkout/src/test/ui/bastion-oethod.\n\nErroneous code examples:\n\n