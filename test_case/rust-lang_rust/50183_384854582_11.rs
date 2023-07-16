\n\nThis will fail because the compiler does not know which instance of `Foo` to\ncall `bar` on. Change `Foo::bar()` to `Foo::<T>::bar()` to resolve the error.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/type-check/issue-40294.rs","byte_start":507,"byte_end":653,"line_start":15,"line_end":21,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn foo<'a,'b,T>(x: &'a T, y: &'b T) //~ ERROR type annotations required","highlight_start":1,"highlight_end":72},{"text":"    where &'a T : Foo,","highlight_start":1,"highlight_end":23},{"text":"          &'b T : Foo","highlight_start":1,"highlight_end":22},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    x.foo();","highlight_start":1,"highlight_end":13},{"text":"    y.foo();","highlight_start":1,"highlight_end":13},{"text":"}","highlight_start":1,"highlight_end":2}],"label":"cannot infer type for `&'a T`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0282]: type annotations needed\n  --> /checkout/src/test/ui/type-check/issue-40294.rs:15:1\n   |\nLL | / fn foo<'a,'b,T>(x: &'a T, y: &'b T) //~ ERROR type annotations required\nLL | |     where &'a T : Foo,\nLL | |           &'b T : Foo\nLL | | {\nLL | |     x.foo();\nLL | |     y.foo();\nLL | | }\n   | |_^ cannot infer type for `&'a T`\n\n"}
[00:48:32] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:32] {"message":"For more information about this error, try `rustc --explain E0282`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0282`.\n"}
[00:48:32] ------------------------------------------
[00:48:32] 
[00:48:32] thread '[ui] ui/type-check/issue-40294.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2963:9
[00:48:32] 
---
[00:48:32] 
[00:48:32] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:48:32] 
[00:48:32] 
[00:48:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -ZunstableFri, 27 Apr 2018 03:39:20 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
