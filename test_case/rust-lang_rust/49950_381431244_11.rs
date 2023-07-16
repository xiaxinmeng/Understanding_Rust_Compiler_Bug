\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-12511.rs","byte_start":467,"byte_end":480,"line_start":11,"line_end":11,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"trait t1 : t2 {","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"...which requires computing the supertraits of `t2`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issue-12511.rs","byte_start":486,"byte_end":499,"line_start":14,"line_end":14,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"trait t2 : t1 {","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires computing the supertraits of `t1`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing the supertraits of `t1`\n  --> /checkout/src/test/ui/issue-12511.rs:11:1\n   |\nLL | trait t1 : t2 {\n   | ^^^^^^^^^^^^^\n   |\nnote: ...which requires computing the supertraits of `t2`...\n  --> /checkout/src/test/ui/issue-12511.rs:14:1\n   |\nLL | trait t2 : t1 {\n   | ^^^^^^^^^^^^^\n   = note: ...which again requires computing the supertraits of `t1`, completing the cycle\n\n"}
[00:44:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:35] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
---
[00:44:35] 1 error[E0391]: cycle detected when const-evaluating `X::A::{{initializer}}`
[00:44:35] -    |
[00:44:35] - note: ...which requires computing layout of `X`...
[00:44:35] 4   --> $DIR/issue-23302-1.rs:14:9
[00:44:35] 5    |
[00:44:35] 6 LL |     A = X::A as isize,
[00:44:35]
[00:44:35] 7    |         ^^^^
[00:44:35] - note: ...which again requires const-evaluating `X::A::{{initializer}}`, completing the cycle
[00:44:35] +    |
[00:44:35] + note: ...which requires computing layout of `X`...
[00:44:35] +    = note: ...which again requires const-evaluating `X::A::{{initializer}}`, completing the cycle
---
[00:44:35] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-23302-1.rs'
[00:44:35]
[00:44:35] error: 1 errors occurred comparing output.
[00:44:35] status: exit code: 101
[00:44:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-23302-1.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-23302-1.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-23302-1.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:44:35] {"message":"cycle detected when const-evaluating `X::A::{{initializer}}`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n