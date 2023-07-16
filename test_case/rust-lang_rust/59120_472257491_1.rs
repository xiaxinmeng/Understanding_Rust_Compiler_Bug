\n\nPlease be sure that a file corresponding to the module exists. If you\nwant to use a module named `file_that_doesnt_exist`, you need to have a file\nnamed `file_that_doesnt_exist.rs` or `file_that_doesnt_exist/mod.rs` in the\nsame directory.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-26873-multifile.rs","byte_start":121,"byte_end":142,"line_start":8,"line_end":8,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"mod issue_26873_multifile;","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"name the file either issue_26873_multifile.rs or issue_26873_multifile/mod.rs inside the directory \"/checkout/src/test/run-pass/issues\"","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0583]: file not found for module `issue_26873_multifile`\n  --> /checkout/src/test/run-pass/issues/issue-26873-multifile.rs:8:5\n   |\nLL | mod issue_26873_multifile;\n   |     ^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: name the file either issue_26873_multifile.rs or issue_26873_multifile/mod.rs inside the directory \"/checkout/src/test/run-pass/issues\"\n\n"}
[01:16:07] {"message":"For more information about this error, try `rustc --explain E0583`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0583`.\n"}
[01:16:07] 
[01:16:07] ------------------------------------------
[01:16:07] 
[01:16:07] 
[01:16:07] thread '[run-pass] run-pass/issues/issue-26873-multifile.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:16:07] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:16:07] 
[01:16:07] ---- [run-pass] run-pass/issues/issue-3136-b.rs stdout ----
[01:16:07] 
[01:16:07] error: aux-build `/checkout/src/test/run-pass/issues/auxiliary/issue_3136_a.rc` source not found
[01:16:07] thread '[run-pass] run-pass/issues/issue-3136-b.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2043:9
[01:16:07] ---- [run-pass] run-pass/issues/issue-38190.rs stdout ----
[01:16:07] 
[01:16:07] 
[01:16:07] error: test compilation failed although it shouldn't!
[01:16:07] status: exit code: 1
[01:16:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-38190.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-38190/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-38190/auxiliary"
[01:16:07] ------------------------------------------
[01:16:07] 
[01:16:07] ------------------------------------------
[01:16:07] stderr:
[01:16:07] stderr:
[01:16:07] ------------------------------------------
[01:16:07] {"message":"file not found for module `issue_38190`","code":{"code":"E0583","explanation":"\nA file wasn't found for an out-of-line module.\n\nErroneous code example:\n\n