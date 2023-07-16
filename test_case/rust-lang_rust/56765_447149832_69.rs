\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-40288-2.rs","byte_start":1057,"byte_end":1065,"line_start":34,"line_end":34,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    out.head","highlight_start":5,"highlight_end":13}],"label":"lifetime `'a` required","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add explicit lifetime `'a` to the type of `y`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-40288-2.rs","byte_start":903,"byte_end":905,"line_start":28,"line_end":28,"column_start":58,"column_end":60,"is_primary":true,"text":[{"text":"fn lifetime_transmute_struct<'a, T: ?Sized>(x: &'a T, y: &T) -> &'a T {","highlight_start":58,"highlight_end":60}],"label":null,"suggested_replacement":"&'a T","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0621]: explicit lifetime required in the type of `y`\n  --> /checkout/src/test/ui/issues/issue-40288-2.rs:34:5\n   |\nLL | fn lifetime_transmute_struct<'a, T: ?Sized>(x: &'a T, y: &T) -> &'a T {\n   |                                                          -- help: add explicit lifetime `'a` to the type of `y`: `&'a T`\n...\nLL |     out.head\n   |     ^^^^^^^^ lifetime `'a` required\n\n"}
[00:49:31] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:31] {"message":"For more information about this error, try `rustc --explain E0621`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0621`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/issues/issue-40288-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] 
[00:49:31] 
[00:49:31] ---- [ui] ui/issues/issue-42312.rs stdout ----
[00:49:31] diff of stderr:
[00:49:31] 
[00:49:31] - error[E0277]: the size for values of type `<Self as std::ops::Deref>::Target` cannot be known at compilation time
[00:49:31] -   --> $DIR/issue-42312.rs:14:29
[00:49:31] + error[E0277]: the size for values of type `(dyn std::string::ToString + 'static)` cannot be known at compilation time
[00:49:31] +   --> $DIR/issue-42312.rs:18:23
[00:49:31] 3    |
[00:49:31] - LL |     fn baz(_: Self::Target) where Self: Deref {}
[00:49:31] -    |                             ^ doesn't have a size known at compile-time
[00:49:31] + LL | pub fn f(_: ToString) {}
[00:49:31] +    |                       ^ doesn't have a size known at compile-time
[00:49:31] 6    |
[00:49:31] -    = help: the trait `std::marker::Sized` is not implemented for `<Self as std::ops::Deref>::Target`
[00:49:31] +    = help: the trait `std::marker::Sized` is not implemented for `(dyn std::string::ToString + 'static)`
[00:49:31] 8    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:49:31] -    = help: consider adding a `where <Self as std::ops::Deref>::Target: std::marker::Sized` bound
[00:49:31] 10    = note: all function arguments must have a statically known size
[00:49:31] 12 
[00:49:31] 
[00:49:31] 
[00:49:31] - error[E0277]: the size for values of type `(dyn std::string::ToString + 'static)` cannot be known at compilation time
[00:49:31] -   --> $DIR/issue-42312.rs:18:23
[00:49:31] + error[E0277]: the size for values of type `<Self as std::ops::Deref>::Target` cannot be known at compilation time
[00:49:31] +   --> $DIR/issue-42312.rs:14:29
[00:49:31] 15    |
[00:49:31] - LL | pub fn f(_: ToString) {}
[00:49:31] -    |                       ^ doesn't have a size known at compile-time
[00:49:31] + LL |     fn baz(_: Self::Target) where Self: Deref {}
[00:49:31] +    |                             ^ doesn't have a size known at compile-time
[00:49:31] 18    |
[00:49:31] -    = help: the trait `std::marker::Sized` is not implemented for `(dyn std::string::ToString + 'static)`
[00:49:31] +    = help: the trait `std::marker::Sized` is not implemented for `<Self as std::ops::Deref>::Target`
[00:49:31] 20    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:49:31] +    = help: consider adding a `where <Self as std::ops::Deref>::Target: std::marker::Sized` bound
[00:49:31] 21    = note: all function arguments must have a statically known size
[00:49:31] 23 
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42312/issue-42312.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args issues/issue-42312.rs`
[00:49:31] error: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
[00:49:31] status: exit code: 1
[00:49:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-42312.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42312/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42312/auxiliary" "-A" "unused"
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] ------------------------------------------
[00:49:31] stderr:
[00:49:31] stderr:
[00:49:31] ------------------------------------------
[00:49:31] {"message":"the size for values of type `(dyn std::string::ToString + 'static)` cannot be known at compilation time","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n