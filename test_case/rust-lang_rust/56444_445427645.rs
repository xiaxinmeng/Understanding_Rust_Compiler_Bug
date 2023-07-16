plain
[00:46:31] failures:
[00:46:31] 
[00:46:31] ---- [ui] ui/proc-macro/edition-imports-2018.rs stdout ----
[00:46:31] 
[00:46:31] error: test compilation failed although it shouldn't!
[00:46:31] status: exit code: 1
[00:46:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/edition-imports-2018.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/edition-imports-2018/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/edition-imports-2018/auxiliary" "-A" "unused"
[00:46:31] ------------------------------------------
[00:46:31] 
[00:46:31] ------------------------------------------
[00:46:31] stderr:
[00:46:31] stderr:
[00:46:31] ------------------------------------------
[00:46:31] {"message":"can't find crate for `edition_imports_2015`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n