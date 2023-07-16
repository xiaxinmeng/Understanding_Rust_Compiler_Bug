\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/multispan.rs","byte_start":991,"byte_end":1005,"line_start":31,"line_end":31,"column_start":20,"column_end":34,"is_primary":true,"text":[{"text":"        return Err(Span::def_site()","highlight_start":20,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(proc_macro_def_site)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'proc_macro_def_site' (see issue #54724)\n  --> /checkout/src/test/ui-fulldeps/proc-macro/auxiliary/multispan.rs:31:20\n   |\nLL |         return Err(Span::def_site()\n   |                    ^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(proc_macro_def_site)] to the crate attributes to enable\n\n"}
[01:06:51] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:06:51] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] thread '[ui] ui-fulldeps/proc-macro/multispan.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[01:06:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:51] 
[01:06:51] ---- [ui] ui-fulldeps/proc-macro/three-equals.rs stdout ----
[01:06:51] 
[01:06:51] error: auxiliary build of "/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/three-equals.rs" failed to compile: 
[01:06:51] status: exit code: 1
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/three-equals.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/three-equals/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/three-equals/auxiliary"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] {"message":"use of unstable library feature 'proc_macro_def_site' (see issue #54724)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n