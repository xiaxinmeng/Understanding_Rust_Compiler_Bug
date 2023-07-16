\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/uninhabited_enum_discriminant1.rs","byte_start":612,"byte_end":625,"line_start":14,"line_end":14,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"    A = X::A as isize, //~ ERROR enums without inhabited variants do not have discriminants","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0080]: constant evaluation of enum discriminant resulted in non-integer\n  --> /checkout/src/test/ui/uninhabited_enum_discriminant1.rs:14:9\n   |\nLL |     A = X::A as isize, //~ ERROR enums without inhabited variants do not have discriminants\n   |         ^^^^^^^^^^^^^\n\n"}
[00:41:45] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:41:45] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
---
[00:41:45] - error[E0080]: enums without inhabited variants do not have discriminants
[00:41:45] + error[E0080]: constant evaluation of enum discriminant resulted in non-integer
[00:41:45] 2   --> $DIR/uninhabited_enum_discriminant2.rs:14:9
[00:41:45] 3    |
[00:41:45] 4 LL |     B = A, //~ ERROR enums without inhabited variants do not have discriminants
[00:41:45]
[00:41:45]
[00:41:45] The actual stderr differed from the expected stderr.
[00:41:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited_enum_discriminant2.stderr
[00:41:45] To update references, run this command from build directory:
[00:41:45] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'uninhabited_enum_discriminant2.rs'
[00:41:45]
[00:41:45] error: 1 errors occurred comparing output.
[00:41:45] status: exit code: 101
[00:41:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/uninhabited_enum_discriminant2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited_enum_discriminant2.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited_enum_discriminant2.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:41:45] {"message":"constant evaluation of enum discriminant resulted in non-integer","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n