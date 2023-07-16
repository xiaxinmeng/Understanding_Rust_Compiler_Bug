\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-23302-2.rs","byte_start":567,"byte_end":571,"line_start":14,"line_end":14,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"    A = Y::B as isize,","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"...which requires computing layout of `Y`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issue-23302-2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2015 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires const-evaluating `Y::A::{{initializer}}`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when const-evaluating `Y::A::{{initializer}}`\n  --> /checkout/src/test/ui/issue-23302-2.rs:14:9\n   |\nLL |     A = Y::B as isize,\n   |         ^^^^\n   |\nnote: ...which requires computing layout of `Y`...\n   = note: ...which again requires const-evaluating `Y::A::{{initializer}}`, completing the cycle\n\n"}
[00:44:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:35] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
---
[00:44:35] 1 error[E0391]: cycle detected when const checking if rvalue is promotable to static `A`
[00:44:35] -    |
[00:44:35] - note: ...which requires checking which parts of `A` are promotable to static...
[00:44:35] 4   --> $DIR/issue-23302-3.rs:11:1
[00:44:35] 5    |
[00:44:35] 6 LL | const A: i32 = B;
[00:44:35]
[00:44:35] 7    | ^^^^^^^^^^^^^^^^^
[00:44:35] - note: ...which requires const checking if rvalue is promotable to static `B`...
[00:44:35] +    |
[00:44:35] + note: ...which requires checking which parts of `A` are promotable to static...
[00:44:35] 9   --> $DIR/issue-23302-3.rs:11:16
[00:44:35] 10    |
[00:44:35] 11 LL | const A: i32 = B;
[00:44:35]
[00:44:35] 12    |                ^
[00:44:35] - note: ...which requires checking which parts of `B` are promotable to static...
[00:44:35] + note: ...which requires const checking if rvalue is promotable to static `B`...
[00:44:35] 14   --> $DIR/issue-23302-3.rs:13:1
[00:44:35] 15    |
[00:44:35] 16 LL | const B: i32 = A;
[00:44:35]
[00:44:35] 17    | ^^^^^^^^^^^^^^^^^
[00:44:35] - note: ...which again requires const checking if rvalue is promotable to static `A`, completing the cycle
[00:44:35] + note: ...which requires checking which parts of `B` are promotable to static...
[00:44:35] 19   --> $DIR/issue-23302-3.rs:13:16
[00:44:35] 20    |
[00:44:35] 21 LL | const B: i32 = A;
[00:44:35]
[00:44:35] 22    |                ^
[00:44:35] +    = note: ...which again requires const checking if rvalue is promotable to static `A`, completing the cycle
---
[00:44:35] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-23302-3.rs'
[00:44:35]
[00:44:35] error: 1 errors occurred comparing output.
[00:44:35] status: exit code: 101
[00:44:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-23302-3.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-23302-3.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-23302-3.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:44:35] {"message":"cycle detected when const checking if rvalue is promotable to static `A`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n