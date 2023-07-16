\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-20605.rs","byte_start":544,"byte_end":551,"line_start":12,"line_end":12,"column_start":17,"column_end":24,"is_primary":true,"text":[{"text":"    for item in *things { *item = 0 }","highlight_start":17,"highlight_end":24}],"label":"doesn't have a size known at compile-time","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::marker::Sized` is not implemented for `dyn std::iter::Iterator<Item=&mut u8>`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required by `std::iter::IntoIterator::into_iter`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `dyn std::iter::Iterator<Item=&mut u8>` cannot be known at compilation time\n  --> /checkout/src/test/ui/issue-20605.rs:12:17\n   |\nLL |     for item in *things { *item = 0 }\n   |                 ^^^^^^^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `dyn std::iter::Iterator<Item=&mut u8>`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>\n   = note: required by `std::iter::IntoIterator::into_iter`\n\n"}
[00:43:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:30] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:43:30] ------------------------------------------
[00:43:30] 
[00:43:30] thread '[ui] ui/issue-20605.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:43:30] 
[00:43:30] 
[00:43:30] ---- [ui] ui/issue-22874.rs stdout ----
[00:43:30] diff of stderr:
[00:43:30] 
[00:43:30] 5    |     ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
[00:43:30] 6    |
[00:43:30] 7    = help: the trait `std::marker::Sized` is not implemented for `[std::string::String]`
[00:43:30] -    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types--sized>
[00:43:30] +    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>
[00:43:30] 9    = note: slice and array elements must have `Sized` type
[00:43:30] 11 error: aborting due to previous error
[00:43:30] 
[00:43:30] 
[00:43:30] The actual stderr differed from the expected stderr.
[00:43:30] The actual stderr differed from the expected stderr.
[00:43:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-22874/issue-22874.stderr
[00:43:30] To update references, rerun the tests and pass the `--bless` flag
[00:43:30] To only update this specific test, also pass `--test-args issue-22874.rs`
[00:43:30] error: 1 errors occurred comparing output.
[00:43:30] status: exit code: 101
[00:43:30] status: exit code: 101
[00:43:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-22874.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-22874/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-22874/auxiliary" "-A" "unused"
[00:43:30] ------------------------------------------
[00:43:30] 
[00:43:30] ------------------------------------------
[00:43:30] stderr:
[00:43:30] stderr:
[00:43:30] ------------------------------------------
[00:43:30] {"message":"the size for values of type `[std::string::String]` cannot be known at compilation time","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n