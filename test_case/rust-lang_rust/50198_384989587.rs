plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:22] 
[01:00:22] running 1376 tests
[01:00:27] ..................................................................................i.................
[01:00:35] .............F................i.....................................................................
[01:00:39] .........................................F..........................................................
[01:00:48] ....................................................................................................
[01:00:52] ....................................................................................................
[01:00:59] ....................................................................................................
[01:01:06] ....................................................................................................
---
[01:01:49] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'const-eval/index_out_of_bound.rs'
[01:01:49] 
[01:01:49] error: 1 errors occurred comparing output.
[01:01:49] status: exit code: 101
[01:01:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/index_out_of_bound.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/index_out_of_bound.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/index_out_of_bound.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:01:49] ------------------------------------------
[01:01:49] 
[01:01:49] ------------------------------------------
[01:01:49] stderr:
[01:01:49] stderr:
[01:01:49] ------------------------------------------
[01:01:49] {"message":"constant evaluation error","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n