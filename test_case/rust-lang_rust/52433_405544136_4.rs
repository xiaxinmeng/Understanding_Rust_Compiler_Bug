\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-14366.rs","byte_start":492,"byte_end":498,"line_start":12,"line_end":12,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    let _x = \"test\" as &::std::any::Any;","highlight_start":14,"highlight_end":20}],"label":"doesn't have a size known at compile-time","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::marker::Sized` is not implemented for `str`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required for the cast to the object type `dyn std::any::Any`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `str` cannot be known at compilation time\n  --> /checkout/src/test/ui/issue-14366.rs:12:14\n   |\nLL |     let _x = \"test\" as &::std::any::Any;\n   |              ^^^^^^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `str`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>\n   = note: required for the cast to the object type `dyn std::any::Any`\n\n"}
[00:43:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:30] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:43:30] ------------------------------------------
[00:43:30] 
[00:43:30] thread '[ui] ui/issue-14366.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:43:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:43:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:43:30] 
[00:43:30] ---- [ui] ui/issue-15756.rs stdout ----
[00:43:30] diff of stderr:
[00:43:30] 
[00:43:30] 5    |          ^^^^^^^^^ doesn't have a size known at compile-time
[00:43:30] 6    |
[00:43:30] 7    = help: the trait `std::marker::Sized` is not implemented for `[T]`
[00:43:30] -    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types--sized>
[00:43:30] +    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>
[00:43:30] 9    = note: all local variables must have a statically known size
[00:43:30] 11 error: aborting due to previous error
[00:43:30] 
[00:43:30] 
[00:43:30] The actual stderr differed from the expected stderr.
[00:43:30] The actual stderr differed from the expected stderr.
[00:43:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-15756/issue-15756.stderr
[00:43:30] To update references, rerun the tests and pass the `--bless` flag
[00:43:30] To only update this specific test, also pass `--test-args issue-15756.rs`
[00:43:30] error: 1 errors occurred comparing output.
[00:43:30] status: exit code: 101
[00:43:30] status: exit code: 101
[00:43:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-15756.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-15756/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-15756/auxiliary" "-A" "unused"
[00:43:30] ------------------------------------------
[00:43:30] 
[00:43:30] ------------------------------------------
[00:43:30] stderr:
[00:43:30] stderr:
[00:43:30] ------------------------------------------
[00:43:30] {"message":"the size for values of type `[T]` cannot be known at compilation time","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n