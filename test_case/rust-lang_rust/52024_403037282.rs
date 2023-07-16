plain
[00:46:14] 
[00:46:14] ---- [ui (nll)] ui/existential_type.rs stdout ----
[00:46:14] diff of stderr:
[00:46:14] 
[00:46:14] 54    = help: consider adding a `where T: Trait` bound
[00:46:14] 55    = note: the return type of a function must have a statically known size
[00:46:14] 56 
[00:46:14] - error[E0310]: the parameter type `T` may not live long enough
[00:46:14] + warning: not reporting region error due to nll
[00:46:14] 58   --> $DIR/existential_type.rs:78:1
[00:46:14] 59    |
[00:46:14] 60 LL | existential type WrongGeneric<T>: 'static;
[00:46:14] 61    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:46:14] - ...
[00:46:14] - ...
[00:46:14] - LL | fn wrong_generic<T>(t: T) -> WrongGeneric<T> {
[00:46:14] -    |                  - help: consider adding an explicit lifetime bound `T: 'static`...
[00:46:14] -    |
[00:46:14] - note: ...so that the type `T` will meet its required lifetime bounds
[00:46:14] -   --> $DIR/existential_type.rs:78:1
[00:46:14] -    |
[00:46:14] - LL | existential type WrongGeneric<T>: 'static;
[00:46:14] 71 
[00:46:14] 72 error: could not find defining uses
[00:46:14] 73   --> $DIR/existential_type.rs:28:1
[00:46:14] 
[00:46:14] 
[00:46:14] 97 LL | | }
[00:46:14] 99 
[00:46:14] - error: aborting due to 9 previous errors
[00:46:14] + error: aborting due to 8 previous errors
[00:46:14] 101 
[00:46:14] 101 
[00:46:14] - Some errors occurred: E0277, E0308, E0310.
[00:46:14] + Some errors occurred: E0277, E0308.
[00:46:14] 103 For more information about an error, try `rustc --explain E0277`.
[00:46:14] 104 
[00:46:14] 
[00:46:14] 
[00:46:14] The actual stderr differed from the expected stderr.
[00:46:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_type.nll/existential_type.nll.stderr
[00:46:14] To update references, rerun the tests and pass the `--bless` flag
[00:46:14] To only update this specific test, also pass `--test-args existential_type.rs`
[00:46:14] error: 1 errors occurred comparing output.
[00:46:14] status: exit code: 101
[00:46:14] status: exit code: 101
[00:46:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential_type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_type.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_type.nll/auxiliary" "-A" "unused"
[00:46:14] ------------------------------------------
[00:46:14] 
[00:46:14] ------------------------------------------
[00:46:14] stderr:
[00:46:14] stderr:
[00:46:14] ------------------------------------------
[00:46:14] {"message":"defining existential type use differs from previous","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/existential_type.rs","byte_start":621,"byte_end":712,"line_start":23,"line_end":25,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn bar() -> Foo { //~ ERROR defining existential type use differs from previous","highlight_start":1,"highlight_end":80},{"text":"    42i32","highlight_start":1,"highlight_end":10},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"previous use here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/existential_type.rs","byte_start":593,"byte_end":619,"line_start":19,"line_end":21,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn foo() -> Foo {","highlight_start":1,"highlight_end":18},{"text":"    \"\"","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: defining existential type use differs from previous\n  --> /checkout/src/test/ui/existential_type.rs:23:1\n   |\nLL | / fn bar() -> Foo { //~ ERROR defining existential type use differs from previous\nLL | |     42i32\nLL | | }\n   | |_^\n   |\nnote: previous use here\n  --> /checkout/src/test/ui/existential_type.rs:19:1\n   |\nLL | / fn foo() -> Foo {\nLL | |     \"\"\nLL | | }\n   | |_^\n\n"}
[00:46:14] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n