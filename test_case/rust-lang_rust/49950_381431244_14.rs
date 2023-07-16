\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-23302-3.rs","byte_start":467,"byte_end":484,"line_start":11,"line_end":11,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"const A: i32 = B;","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"...which requires checking which parts of `A` are promotable to static...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issue-23302-3.rs","byte_start":482,"byte_end":483,"line_start":11,"line_end":11,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"const A: i32 = B;","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires const checking if rvalue is promotable to static `B`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issue-23302-3.rs","byte_start":486,"byte_end":503,"line_start":13,"line_end":13,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"const B: i32 = A;","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires checking which parts of `B` are promotable to static...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issue-23302-3.rs","byte_start":501,"byte_end":502,"line_start":13,"line_end":13,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"const B: i32 = A;","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires const checking if rvalue is promotable to static `A`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when const ch: ...which requires processing `Foo::B::{{initializer}}`...
[00:44:35] 4   --> $DIR/issue-36163.rs:14:9
[00:44:35] 5    |
[00:44:35] 6 LL |     B = A,
[00:44:35]
[00:44:35] 7    |         ^
[00:44:35] - note: ...which requires const-evaluating `A`...
[00:44:35] +    |
[00:44:35] + note: ...which requires processing `Foo::B::{{initializer}}`...
[00:44:35] 9   --> $DIR/issue-36163.rs:11:1
[00:44:35] 10    |
[00:44:35] 11 LL | const A: isize = Foo::B as isize;
[00:44:35]
[00:44:35] 12    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:44:35] - note: ...which requires computing layout of `Foo`...
[00:44:35] + note: ...which requires const-evaluating `A`...
[00:44:35] 14   --> $DIR/issue-36163.rs:11:18
[00:44:35] 15    |
[00:44:35] 16 LL | const A: isize = Foo::B as isize;
[00:44:35]
[00:44:35] 17    |                  ^^^^^^
[00:44:35] - note: ...which again requires const-evaluating `Foo::B::{{initializer}}`, completing the cycle
[00:44:35] + note: ...which requires computing layout of `Foo`...
[00:44:35] +    = note: ...which again requires const-evaluating `Foo::B::{{initializer}}`, completing the cycle
---
[00:44:35] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-36163.rs'
[00:44:35]
[00:44:35] error: 1 errors occurred comparing output.
[00:44:35] status: exit code: 101
[00:44:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-36163.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-36163.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-36163.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:44:35] {"message":"cycle detected when const-evaluating `Foo::B::{{initializer}}`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n