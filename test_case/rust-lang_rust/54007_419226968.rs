plain
[00:47:13] ....................................................................................................
[00:47:16] ....................i...............................................................................
[00:47:19] ....................................................................................................
[00:47:22] ....................................................................................................
[00:47:25] ..................................................................F.................................
[00:47:31] ....................................................................................................
[00:47:34] ....................................................................................................
[00:47:38] .......................................................................................i............
[00:47:41] ....................................................................................................
---
[00:47:50] 
[00:47:50] ---- [ui] ui/removing-extern-crate.rs stdout ----
[00:47:50] diff of stderr:
[00:47:50] 
[00:47:50] - warning: unused extern crate
[00:47:50] + warning: `extern crate` is not idiomatic in the new edition
[00:47:50] 3    |
[00:47:50] 3    |
[00:47:50] 4 LL | extern crate std as foo;
[00:47:50] 
[00:47:50] -    | ^^^^^^^^^^^ckout/src/test/ui/removing-extern-crate.rs:16:9\n   |\nLL | #![warn(rust_2018_idioms)]\n   |         ^^^^^^^^^^^^^^^^\n   = note: #[warn(unused_extern_crates)] implied by #[warn(rust_2018_idioms)]\n\n"}
[00:47:50] {"message":"unused extern crate","code":{"code":"unused_extern_crates","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/removing-extern-crate.rs","byte_start":632,"byte_end":650,"line_start":20,"line_end":20,"column_start":1,"column_end":19,"is_primary":true,"text":[{"text":"extern crate core;","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove it","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/removing-extern-crate.rs","byte_start":632,"byte_end":650,"line_start":20,"line_end":20,"column_start":1,"column_end":19,"is_primary":true,"text":[{"text":"extern crate core;","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused extern crate\n  --> /checkout/src/test/ui/removing-extern-crate.rs:20:1\n   |\nLL | extern crate core;\n   | ^^^^^^^^^^^^^^^^^^ help: remove it\n\n"}
[00:47:50] {"message":"`extern crate` is not idiomatic in the new edition","code":{"code":"unused_extern_crates","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/removing-extern-crate.rs","byte_start":670,"byte_end":694,"line_start":23,"line_end":23,"column_start":5,"column_end":29,"is_primary":tru":5,"column_end":22,"is_primary":true,"text":[{"text":"    extern crate std;","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":"use std;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: `extern crate` is not idiomatic in the new edition\n  --> /checkout/src/test/ui/removing-extern-crate.rs:24:5\n   |\nLL |     extern crate std;\n   |     ^^^^^^^^^^^^^^^^^ help: convert it to a `use`\n\n"}
[00:47:50] ------------------------------------------
[00:47:50] 
[00:47:50] thread '[ui] ui/removing-extern-crate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:47:50] 
[00:47:50] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:47:50] 
[00:47:50] 
[00:47:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linThu, 06 Sep 2018 20:17:12 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
