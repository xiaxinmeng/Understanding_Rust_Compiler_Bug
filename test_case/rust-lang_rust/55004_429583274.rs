plain
[00:58:41] 
[00:58:41] ---- [ui] ui/issues/issue-54410.rs stdout ----
[00:58:41] diff of stderr:
[00:58:41] 
[00:58:41] - error[E0277]: the size for values of type `[i8]` cannot be known at compilation time
[00:58:41] + error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
[00:58:41] 3    |
[00:58:41] 3    |
[00:58:41] 4 LL |     pub static mut symbol: [c_char];
[00:58:41] 5    |                            ^^^^^^^^ doesn't have a size known at compile-time
[00:58:41] 6    |
[00:58:41] -    = help: the trait `std::marker::Sized` is not implemented for `[i8]`
[00:58:41] +    = help: the trait `std::marker::Sized` is not implemented for `[u8]`
[00:58:41] +    = help: the trait `std::marker::Sized` is not implemented for `[u8]`
[00:58:41] 8    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:58:41] 9 
[00:58:41] 10 error: aborting due to previous error
[00:58:41] 
[00:58:41] 
[00:58:41] The actual stderr differed from the expected stderr.
[00:58:41] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54410/issue-54410.stderr
[00:58:41] To update references, rerun the tests and pass the `--bless` flag
[00:58:41] To only update this specific test, also pass `--test-args issues/issue-54410.rs`
[00:58:41] error: 1 errors occurred comparing output.
[00:58:41] status: exit code: 1
[00:58:41] status: exit code: 1
[00:58:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-54410.rs" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54410/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54410/auxiliary" "-A" "unused"
[00:58:41] ------------------------------------------
[00:58:41] 
[00:58:41] ------------------------------------------
[00:58:41] stderr:
[00:58:41] stderr:
[00:58:41] ------------------------------------------
[00:58:41] {"message":"the size for values of type `[u8]` cannot be known at compilation time","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n