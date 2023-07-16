\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/issue-45983.rs","byte_start":572,"byte_end":587,"line_start":17,"line_end":17,"column_start":14,"column_end":29,"is_primary":true,"text":[{"text":"    give_any(|y| x = Some(y));","highlight_start":14,"highlight_end":29}],"label":"cannot borrow as mutable","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0596]: cannot borrow immutable item `x` as mutable\n  --> /checkout/src/test/ui/borrowck/issue-45983.rs:17:14\n   |\nLL |     give_any(|y| x = Some(y));\n   |              ^^^^^^^^^^^^^^^ cannot borrow as mutable\n\n"}
[00:49:37] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:49:37] {"message":"Some errors occurred: E0594, E0596.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0594, E0596.\n"}
[00:49:37] {"message":"For more information about an error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0594`.\n"}
[00:49:37]
[00:49:37] ------------------------------------------
[00:49:37]
[00:49:37] thread '[ui (nll)] ui/borrowck/issue-45983.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[00:49:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:37]
[00:49:37] ---- [ui (nll)] ui/did_you_mean/issue-35937.rs stdout ----
[00:49:37]  diff of stderr:
[00:49:37]
[00:49:37] 4 LL |     f.v.push("cat".to_string()); //~ ERROR cannot borrow
[00:49:37] 5    |     ^^^ cannot borrow as mutable
[00:49:37] 6    |
[00:49:37] -    = note: Value not mutable causing this error: `f`
[00:49:37] +    = note: the value which is causing this path not to be mutable is...: `f`
[00:49:37] 8
[00:49:37] 9 error[E0384]: cannot assign twice to immutable variable `s.x`
[00:49:37] 10   --> $DIR/issue-35937.rs:26:5
[00:49:37]
[00:49:37]
[00:49:37] The actual stderr differed from the expected stderr.
[00:49:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-35937.stderr
[00:49:37] To update references, run this command from build directory:
[00:49:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'did_you_mean/issue-35937.rs'
[00:49:37]
[00:49:37] error: 1 errors occurred comparing output.
[00:49:37] status: exit code: 101
[00:49:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-35937.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-35937.stage2-x86_64-unknown-linux-gnu" "-Znll" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-35937.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:49:37] {"message":"cannot borrow immutable item `f.v` as mutable","code":{"code":"E0596","explanation":"\nThis error occurs because you tried to mutably borrow a non-mutable variable.\n\nExample of erroneous code:\n\n