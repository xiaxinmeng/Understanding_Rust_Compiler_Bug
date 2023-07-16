\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-15756.rs","byte_start":600,"byte_end":609,"line_start":17,"line_end":17,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"    &mut something","highlight_start":10,"highlight_end":19}],"label":"doesn't have a size known at compile-time","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::marker::Sized` is not implemented for `[T]`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"all local variables must have a statically known size","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `[T]` cannot be known at compilation time\n  --> /checkout/src/test/ui/issue-15756.rs:17:10\n   |\nLL |     &mut something\n   |          ^^^^^^^^^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `[T]`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>\n   = note: all local variables must have a statically known size\n\n"}
[00:43:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:30] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:43:30] ------------------------------------------
[00:43:30] 
[00:43:30] thread '[ui] ui/issue-15756.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:43:30] 
[00:43:30] 
[00:43:30] ---- [ui] ui/issue-17651.rs stdout ----
[00:43:30] diff of stderr:
[00:43:30] 
[00:43:30] 5    |         ^^^^^^^^ doesn't have a size known at compile-time
[00:43:30] 6    |
[00:43:30] 7    = help: the trait `std::marker::Sized` is not implemented for `[{integer}]`
[00:43:30] -    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types--sized>
[00:43:30] +    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>
[00:43:30] 9    = note: required by `<std::boxed::Box<T>>::new`
[00:43:30] 11 error: aborting due to previous error
[00:43:30] 
[00:43:30] 
[00:43:30] The actual stderr differed from the expected stderr.
[00:43:30] The actual stderr differed from the expected stderr.
[00:43:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-17651/issue-17651.stderr
[00:43:30] To update references, rerun the tests and pass the `--bless` flag
[00:43:30] To only update this specific test, also pass `--test-args issue-17651.rs`
[00:43:30] error: 1 errors occurred comparing output.
[00:43:30] status: exit code: 101
[00:43:30] status: exit code: 101
[00:43:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-17651.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-17651/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-17651/auxiliary" "-A" "unused"
[00:43:30] stdout:
[0ype,\n    // which *does* implement the Debug trait.\n    some_func(5i32);\n}\n