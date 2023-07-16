\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-35988.rs","byte_start":482,"byte_end":490,"line_start":12,"line_end":12,"column_start":7,"column_end":15,"is_primary":true,"text":[{"text":"    V([Box<E>]),","highlight_start":7,"highlight_end":15}],"label":"doesn't have a size known at compile-time","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::marker::Sized` is not implemented for `[std::boxed::Box<E>]`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"no field of an enum variant may have a dynamically sized type","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `[std::boxed::Box<E>]` cannot be known at compilation time\n  --> /checkout/src/test/ui/issue-35988.rs:12:7\n   |\nLL |     V([Box<E>]),\n   |       ^^^^^^^^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `[std::boxed::Box<E>]`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>\n   = note: no field of an enum variant may have a dynamically sized type\n\n"}
[00:43:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:30] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:43:30] ------------------------------------------
[00:43:30] 
[00:43:30] thread '[ui] ui/issue-35988.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:43:30] 
[00:43:30] 
[00:43:30] ---- [ui] ui/issue-38954.rs stdout ----
[00:43:30] diff of stderr:
[00:43:30] 
[00:43:30] 5    |                       ^ doesn't have a size known at compile-time
[00:43:30] 6    |
[00:43:30] 7    = help: the trait `std::marker::Sized` is not implemented for `str`
[00:43:30] -    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types--sized>
[00:43:30] +    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>
[00:43:30] 10 error: aborting due to previous error
[00:43:30] 11 
[00:43:30] 
[00:43:30] 
[00:43:30] 
[00:43:30] The actual stderr differed from the expected stderr.
[00:43:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-38954/issue-38954.stderr
[00:43:30] To update references, rerun the tests and pass the `--bless` flag
[00:43:30] To only update this specific test, also pass `--test-args issue-38954.rs`
[00:43:30] error: 1 errors occurred comparing output.
[00:43:30] status: exit code: 101
[00:43:30] status: exit code: 101
[00:43:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-38954.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-38954/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-38954/auxiliary" "-A" "unused"
[00:43:30] ------------------------------------------
[00:43:30] 
[00:43:30] ------------------------------------------
[00:43:30] stderr:
[00:43:30] stderr:
[00:43:30] ------------------------------------------
[00:43:30] {"message":"the size for values of type `str` cannot be known at compilation time","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n