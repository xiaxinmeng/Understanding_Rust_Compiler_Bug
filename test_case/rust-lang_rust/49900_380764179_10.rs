\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-38147-1.rs","byte_start":674,"byte_end":680,"line_start":27,"line_end":27,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        self.s.push('x'); //~ ERROR cannot borrow data mutably","highlight_start":9,"highlight_end":15}],"label":"cannot borrow as mutable","suggested_replacement":null,"expansion":null}],"children":[{"message":"the value which is causing this path not to be mutable is...: `*self`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow immutable item `*self.s` as mutable\n  --> /checkout/src/test/ui/did_you_mean/issue-38147-1.rs:27:9\n   |\nLL |         self.s.push('x'); //~ ERROR cannot borrow data mutably\n   |         ^^^^^^ cannot borrow as mutable\n   |\n   = note: the value which is causing this path not to be mutable is...: `*self`\n\n"}
[00:49:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:37] {"message":"For more information about this error, try `rustc --explain E0596`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0596`.\n"}
[00:49:37]
[00:49:37] ------------------------------------------
[00:49:37]
[00:49:37] thread '[ui (nll)] ui/did_you_mean/issue-38147-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[00:49:37]
[00:49:37] ---- [ui (nll)] ui/did_you_mean/issue-38147-4.rs stdout ----
[00:49:37]  diff of stderr:
[00:49:37]
[00:49:37] 4 LL |     f.s.push('x'); //~ ERROR cannot borrow data mutably
[00:49:37] 5    |     ^^^ cannot borrow as mutable
[00:49:37] 6    |
[00:49:37] -    = note: Value not mutable causing this error: `*f`
[00:49:37] +    = note: the value which is causing this path not to be mutable is...: `*f`
---
[00:49:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'did_you_mean/issue-38147-4.rs'
[00:49:37]
[00:49:37] error: 1 errors occurred comparing output.
[00:49:37] status: exit code: 101
[00:49:37] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:486:22
[00:49:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-38147-4.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-38147-4.stage2-x86_64-unknown-linux-gnu" "-Znll" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-38147-4.stag `*f`\n\n"}
[00:49:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:37] {"message":"For more information about this error, try `rustc --explain E0596`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0596`.\n"}
[00:49:37]
[00:49:37] ------------------------------------------
[00:49:37]
[00:49:37] thread '[ui (nll)] ui/did_you_mean/issue-38147-4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[00:49:37]
[00:49:37] ---- [ui (nll)] ui/did_you_mean/issue-39544.rs stdout ----
[00:49:37]  diff of stderr:
[00:49:37]
[00:49:37] 4 LL |     let _ = &mut z.x; //~ ERROR cannot borrow
[00:49:37] 5    |             ^^^^^^^^ cannot borrow as mutable
[00:49:37] 6    |
[00:49:37] -    = note: Value not mutable causing this error: `z`
[00:49:37] +    = note: the value which is causing this path not to be mutable is...: `z`
[00:49:37] 8
[00:49:37] 9 error[E0596]: cannot borrow immutable item `self.x` as mutable
[00:49:37] 10   --> $DIR/issue-39544.rs:26:17
[00:49:37]
[00:49:37] 12 LL |         let _ = &mut self.x; //~ ERROR cannot borrow
[00:49:37] 13    |                 ^^^^^^^^^^^ cannot borrow as mutable
[00:49:37] 14    |
[00:49:37] -    = note: Value not mutable causing this error: `*self`
[00:49:37] +    = note: the value which is causing this path not to be mutable is...: `*self`
[00:49:37] 16
[00:49:37] 17 error[E0596]: cannot borrow immutable item `self.x` as mutable
[00:49:37] 18   --> $DIR/issue-39544.rs:30:17
[00:49:37]
[00:49:37] 20 LL |         let _ = &mut self.x; //~ ERROR cannot borrow
[00:49:37] 21    |                 ^^^^^^^^^^^ cannot borrow as mutable
[00:49:37] 22    |
[00:49:37] -    = note: Value not mutable causing this error: `*self`
[00:49:37] +    = note: the value which is causing this path not to be mutable is...: `*self`
[00:49:37] 24
[00:49:37] 25 error[E0596]: cannot borrow immutable item `other.x` as mutable
[00:49:37] 26   --> $DIR/issue-39544.rs:31:17
[00:49:37]
[00:49:37] 28 LL |         let _ = &mut other.x; //~ ERROR cannot borrow
[00:49:37] 29    |                 ^^^^^^^^^^^^ cannot borrow as mutable
[00:49:37] 30    |
[00:49:37] -    = note: Value not mutable causing this error: `*other`
[00:49:37] +    = note: the value which is causing this path not to be mutable is...: `*other`
[00:49:37] 32
[00:49:37] 33 error[E0596]: cannot borrow immutable item `self.x` as mutable
[00:49:37] 34   --> $DIR/issue-39544.rs:35:17
[00:49:37]
[00:49:37] 36 LL |         let _ = &mut self.x; //~ ERROR cannot borrow
[00:49:37] 37    |                 ^^^^^^^^^^^ cannot borrow as mutable
[00:49:37] 38    |
[00:49:37] -    = note: Value not mutable causing this error: `*self`
[00:49:37] +    = note: the value which is causing this path not to be mutable is...: `*self`
[00:49:37] 40
[00:49:37] 41 error[E0596]: cannot borrow immutable item `other.x` as mutable
[00:49:37] 42   --> $DIR/issue-39544.rs:36:17
[00:49:37]
[00:49:37] 44 LL |         let _ = &mut other.x; //~ ERROR cannot borrow
[00:49:37] 45    |                 ^^^^^^^^^^^^ cannot borrow as mutable
[00:00:49:37] 72
[00:49:37] 73 error[E0596]: cannot borrow immutable item `z.x` as mutable
[00:49:37] 74   --> $DIR/issue-39544.rs:51:13
[00:49:37]
[00:49:37] 76 LL |     let _ = &mut z.x; //~ ERROR cannot borrow
[00:49:37] 77    |             ^^^^^^^^ cannot borrow as mutable
[00:49:37] 78    |
[00:49:37] -    = note: Value not mutable causing this error: `z`
[00:49:37] +    = note: the value which is causing this path not to be mutable is...: `z`
[00:49:37] 80
[00:49:37] 81 error[E0596]: cannot borrow immutable item `w.x` as mutable
[00:49:37] 82   --> $DIR/issue-39544.rs:52:13
[00:49:37]
[00:49:37] 84 LL |     let _ = &mut w.x; //~ ERROR cannot borrow
[00:49:37] 85    |             ^^^^^^^^ cannot borrow as mutable
[00:49:37] 86    |
[00:49:37] -    = note: Value not mutable causing this error: `*w`
[00:49:37] +    = note: the value which is causing this path not to be mutable is...: `*w`
[00:49:37] 88
[00:49:37] 89 error[E0594]: cannot assign to immutable item `*x.0`
[00:49:37] 90   --> $DIR/issue-39544.rs:58:5
[00:49:37]
[00:49:37]
[00:49:37] The actual stderr differed from the expected stderr.
[00:49:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-39544.stderr
[00:49:37] To update references, run this command from build directory:
[00:49:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'did_you_mean/issue-39544.rs'
[00:49:37]
[00:49:37] error: 1 errors occurred comparing output.
[00:49:37] status: exit code: 101
[00:49:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-39544.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-39544.stage2-x86_64-unknown-linux-gnu" "-Znll" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-39544.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:49:37] {"message":"cannot borrow immutable item `z.x` as mutable","code":{"code":"E0596","explanation":"\nThis error occurs because you tried to mutably borrow a non-mutable variable.\n\nExample of erroneous code:\n\n