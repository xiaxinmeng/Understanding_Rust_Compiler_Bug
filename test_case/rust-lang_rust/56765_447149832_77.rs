\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-42312.rs","byte_start":533,"byte_end":533,"line_start":14,"line_end":14,"column_start":29,"column_end":29,"is_primary":true,"text":[{"text":"    fn baz(_: Self::Target) where Self: Deref {}","highlight_start":29,"highlight_end":29}],"label":"doesn't have a size known at compile-time","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::marker::Sized` is not implemented for `<Self as std::ops::Deref>::Target`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider adding a `where <Self as std::ops::Deref>::Target: std::marker::Sized` bound","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"all function arguments must have a statically known size","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"unsized locals are gated as an unstable feature","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `<Self as std::ops::Deref>::Target` cannot be known at compilation time\n  --> /checkout/src/test/ui/issues/issue-42312.rs:14:29\n   |\nLL |     fn baz(_: Self::Target) where Self: Deref {}\n   |                             ^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `<Self as std::ops::Deref>::Target`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>\n   = help: consider adding a `where <Self as std::ops::Deref>::Target: std::marker::Sized` bound\n   = note: all function arguments must have a statically known size\n   = help: unsized locals are gated as an unstable feature\n\n"}
[00:49:31] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:49:31] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/issues/issue-42312.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] 
[00:49:31] 
[00:49:31] ---- [ui] ui/issues/issue-42060.rs stdout ----
[00:49:31] diff of stderr:
[00:49:31] 
[00:49:31] 11    |             ^ non-constant value
[00:49:31] 12 
[00:49:31] 13 error[E0516]: `typeof` is a reserved keyword but unimplemented
[00:49:31] -    |
[00:49:31] -    |
[00:49:31] - LL |     let other: typeof(thing) = thing; //~ ERROR attempt to use a non-constant value in a constant
[00:49:31] -    |                ^^^^^^^^^^^^^ reserved keyword
[00:49:31] - 
[00:49:31] - error[E0516]: `typeof` is a reserved keyword but unimplemented
[00:49:31] 21    |
[00:49:31] 21    |
[00:49:31] 22 LL |     <typeof(q)>::N //~ ERROR attempt to use a non-constant value in a constant
[00:49:31] 23    |      ^^^^^^^^^ reserved keyword
[00:49:31] + 
[00:49:31] + 
[00:49:31] + error[E0516]: `typeof` is a reserved keyword but unimplemented
[00:49:31] +   --> $DIR/issue-42060.rs:13:16
[00:49:31] +    |
[00:49:31] + LL |     let other: typeof(thing) = thing; //~ ERROR attempt to use a non-constant value in a constant
[00:49:31] +    |                ^^^^^^^^^^^^^ reserved keyword
[00:49:31] 25 error: aborting due to 4 previous errors
[00:49:31] 26 
[00:49:31] 
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42060/issue-42060.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args issues/issue-42060.rs`
[00:49:31] error: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
[00:49:31] status: exit code: 1
[00:49:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-42060.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42060/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42060/auxiliary" "-A" "unused"
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] ------------------------------------------
[00:49:31] stderr:
[00:49:31] stderr:
[00:49:31] ------------------------------------------
[00:49:31] {"message":"attempt to use a non-constant value in a constant","code":{"code":"E0435","explanation":"\nA non-constant value was used in a constant expression.\n\nErroneous code example:\n\n