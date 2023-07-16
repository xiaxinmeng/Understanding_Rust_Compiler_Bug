\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-impl-trait.rs","byte_start":556,"byte_end":565,"line_start":12,"line_end":12,"column_start":14,"column_end":23,"is_primary":true,"text":[{"text":"fn main() -> impl Copy { }","highlight_start":14,"highlight_end":23}],"label":"`main` can only return types that implement `std::process::Termination`","suggested_replacement":null,"expansion":null}],"children":[{"message":"consider using `()`, or a `Result`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: `main` has invalid return type `impl std::marker::Copy`\n  --> /checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-impl-trait.rs:12:14: in fn main\n   |\nLL | fn main() -> impl Copy { }\n   |              ^^^^^^^^^ `main` can only return types that implement `std::process::Termination`\n   |\n   = help: consider using `()`, or a `Result`\n\n"}
[00:50:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:37] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/rfc-1937-termination-trait/termination-trait-impl-trait.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:50:37] 
[00:50:37] 
[00:50:37] ---- [ui] ui/span/method-and-field-eager-resolution.rs stdout ----
[00:50:37]  diff of stderr:
[00:50:37] 
[00:50:37] 1 error[E0282]: type annotations needed
[00:50:37] -   --> $DIR/method-and-field-eager-resolution.rs:15:5
[00:50:37] +   --> $DIR/method-and-field-eager-resolution.rs:15:5: in fn main
[00:50:37] 3    |
[00:50:37] 4 LL |     let mut x = Default::default();
[00:50:37] 5    |         ----- consider giving `x` a type
[00:50:37] 
[00:50:37] 9    = note: type must be known at this point
[00:50:37] 11 error[E0282]: type annotations needed
[00:50:37] -   --> $DIR/method-and-field-eager-resolution.rs:22:5
[00:50:37] -   --> $DIR/method-and-field-eager-resolution.rs:22:5
[00:50:37] +   --> $DIR/method-and-field-eager-resolution.rs:22:5: in fn foo
[00:50:37] 13    |
[00:50:37] 14 LL |     let mut x = Default::default();
[00:50:37] 15    |         ----- consider giving `x` a type
[00:50:37] 
[00:50:37] The actual stderr differed from the expected stderr.
[00:50:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/method-and-field-eager-resolution.stderr
[00:50:37] To update references, run this command from build directory:
[00:50:37] To update references, run this command from build directory:
[00:50:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'span/method-and-field-eager-resolution.rs'
[00:50:37] error: 1 errors occurred comparing output.
[00:50:37] status: exit code: 101
[00:50:37] status: exit code: 101
[00:50:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/method-and-field-eager-resolution.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/method-and-field-eager-resolution.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/method-and-field-eager-resolution.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] stderr:
[00:50:37] stderr:
[00:50:37] ------------------------------------------
[00:50:37] {"message":"type annotations needed","code":{"code":"E0282","explanation":"\nThis error indicates that type inference did not result in one unique possible\ntype, and extra information is required. In most cases this can be provided\nby adding a type annotation. Sometimes you need to specify a generic type\nparameter manually.\n\nA common example is the `collect` method on `Iterator`. It has a generic type\nparameter with a `FromIterator` bound, which for a `char` iterator is\nimplemented by `Vec` and `String` among others. Consider the following snippet\nthat reverses the characters of a string:\n\n