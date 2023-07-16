\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-35937.rs","byte_start":723,"byte_end":724,"line_start":29,"line_end":29,"column_start":8,"column_end":9,"is_primary":false,"text":[{"text":"fn bar(s: S) {","highlight_start":8,"highlight_end":9}],"label":"first assignment to `s.x`","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/did_you_mean/issue-35937.rs","byte_start":735,"byte_end":743,"line_start":30,"line_end":30,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    s.x += 1; //~ ERROR cannot assign","highlight_start":5,"highlight_end":13}],"label":"cannot assign twice to immutable variable","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0384]: cannot assign twice to immutable variable `s.x`\n  --> /checkout/src/test/ui/did_you_mean/issue-35937.rs:30:5\n   |\nLL | fn bar(s: S) {\n   |        - first assignment to `s.x`\nLL |     s.x += 1; //~ ERROR cannot assign\n   |     ^^^^^^^^ cannot assign twice to immutable variable\n\n"}
[00:49:37] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:49:37] {"message":"Some errors occurred: E0384, E0596.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0384, E0596.\n"}
[00:49:37] {"message":"For more information about an error, try `rustc --explain E0384`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0384`.\n"}
[00:49:37]
[00:49:37] ------------------------------------------
[00:49:37]
[00:49:37] thread '[ui (nll)] ui/did_you_mean/issue-35937.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[00:49:37]
[00:49:37] ---- [ui (nll)] ui/did_you_mean/issue-38147-1.rs stdout ----
[00:49:37]  diff of stderr:
[00:49:37]
[00:49:37] 4 LL |         self.s.push('x'); //~ ERROR cannot borrow data mutably
[00:49:37] 5    |         ^^^^^^ cannot borrow as mutable
[00:49:37] 6    |
[00:49:37] -    = note: Value not mutable causing this error: `*self`
[00:49:37] +    = note: the value which is causing this path not to be mutable is...: `*self`
---
[00:49:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'did_you_mean/issue-38147-1.rs'
[00:49:37]
[00:49:37] error: 1 errors occurred comparing output.
[00:49:37] status: exit code: 101
[00:49:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-38147-1.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-38147-1.stage2-x86_64-unknown-linux-gnu" "-Znll" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-38147-1.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:49:37] {"message":"cannot borrow immutable item `*self.s` as mutable","code":{"code":"E0596","explanation":"\nThis error occurs because you tried to mutably borrow a non-mutable variable.\n\nExample of erroneous code:\n\n