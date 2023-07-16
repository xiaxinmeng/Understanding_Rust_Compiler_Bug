plain
[00:45:42] failures:
[00:45:42] 
[00:45:42] ---- [ui] ui/proc-macro/edition-imports-2018.rs stdout ----
[00:45:42] 
[00:45:42] error: test compilation failed although it shouldn't!
[00:45:42] status: exit code: 1
[00:45:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/edition-imports-2018.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/edition-imports-2018/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/edition-imports-2018/auxiliary" "-A" "unused"
[00:45:42] ------------------------------------------
[00:45:42] 
[00:45:42] ------------------------------------------
[00:45:42] stderr:
[00:45:42] stderr:
[00:45:42] ------------------------------------------
[00:45:42] {"message":"can't find crate for `edition_imports_2015`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n