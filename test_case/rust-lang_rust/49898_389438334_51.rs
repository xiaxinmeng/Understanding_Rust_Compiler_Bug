\n\nThis syntax specifies that we want the X type from MyTrait, as made concrete in\nMyStruct. The reason that we cannot simply use `MyStruct::X` is that MyStruct\nmight implement two different traits with identically-named associated types.\nThis syntax allows disambiguation between the two.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-types-in-ambiguous-context.rs","byte_start":679,"byte_end":690,"line_start":21,"line_end":21,"column_start":23,"column_end":34,"is_primary":true,"text":[{"text":"    fn grab(&self) -> Grab::Value;","highlight_start":23,"highlight_end":34}],"label":"ambiguous associated type","suggested_replacement":null,"expansion":null}],"children":[{"message":"specify the type using the syntax `<Type as Grab>::Value`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0223]: ambiguous associated type\n  --> /checkout/src/test/ui/associated-types-in-ambiguous-context.rs:21:23: in trait Grab::grab\n   |\nLL |     fn grab(&self) -> Grab::Value;\n   |                       ^^^^^^^^^^^ ambiguous associated type\n   |\n   = note: specify the type using the syntax `<Type as Grab>::Value`\n\n"}
[00:48:30] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:48:30] {"message":"For more information about this error, try `rustc --explain E0223`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0223`.\n"}
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] thread '[ui] ui/associated-types-in-ambiguous-context.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:48:30] 
[00:48:30] 
[00:48:30] ---- [ui] ui/augmented-assignments.rs stdout ----
[00:48:30]  diff of stderr:
[00:48:30] 
[00:48:30] 1 error[E0596]: cannot borrow immutable local variable `y` as mutable
[00:48:30] -   --> $DIR/augmented-assignments.rs:30:5
[00:48:30] +   --> $DIR/augmented-assignments.rs:30:5: in fn main
[00:48:30] 3    |
[00:48:30] 4 LL |     let y = Int(2);
[00:48:30] 5    |         - consider changing this to `mut y`
[00:48:30] 
[00:48:30] 8    |     ^ cannot borrow mutably
[00:48:30] 9 
[00:48:30] 10 error[E0382]: use of moved value: `x`
[00:48:30] -   --> $DIR/augmented-assignments.rs:23:5
[00:48:30] +   --> $DIR/augmented-assignments.rs:23:5: in fn main
[00:48:30] 12    |
[00:48:30] 13 LL |     x   //~ error: use of moved value: `x`
[00:48:30] 14    |     ^ value used here after move
[00:48:30] 
[00:48:30] The actual stderr differed from the expected stderr.
[00:48:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/augmented-assignments.stderr
[00:48:30] To update references, run this command from build directory:
[00:48:30] To update references, run this command from build directory:
[00:48:30] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'augmented-assignments.rs'
[00:48:30] error: 1 errors occurred comparing output.
[00:48:30] status: exit code: 101
[00:48:30] status: exit code: 101
[00:48:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/augmented-assignments.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/augmented-assignments.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/augmented-assignments.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] ------------------------------------------
[00:48:30] stderr:
[00:48:30] stderr:
[00:48:30] ------------------------------------------
[00:48:30] {"message":"cannot borrow immutable local variable `y` as mutable","code":{"code":"E0596","explanation":"\nThis error occurs because you tried to mutably borrow a non-mutable variable.\n\nExample of erroneous code:\n\n