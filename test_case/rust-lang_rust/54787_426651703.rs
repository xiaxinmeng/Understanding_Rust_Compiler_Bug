plain
[00:50:08] ...........i........................................................................................ 1600/4545
[00:50:11] .................................................................................................... 1700/4545
[00:50:14] .................................................................................................... 1800/4545
[00:50:18] ....................................i............................................................... 1900/4545
[00:50:21] .......................................................................F............................ 2000/4545
[00:50:29] .................................................................................................... 2200/4545
[00:50:34] .................................................................................................... 2300/4545
[00:50:37] .................................................................................................... 2400/4545
[00:50:41] .................................................................................................... 2500/4545
---
[00:51:50] 
[00:51:50] 8    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:51:50] 9    = note: required by `std::iter::IntoIterator::into_iter`
[00:51:50] 10 
[00:51:50] - error: aborting due to previous error
[00:51:50] + error[E0277]: the size for values of type `dyn std::iter::Iterator<Item=&mut u8>` cannot be known at compilation time
[00:51:50] +   --> $DIR/issue-20605.rs:12:17
[00:51:50] +    |
[00:51:50] + LL |     for item in *things { *item = 0 }
[00:51:50] +    |
[00:51:50] +    |
[00:51:50] +    = help: the trait `std::marker::Sized` is not implemented for `dyn std::iter::Iterator<Item=&mut u8>`
[00:51:50] +    = note: all local variables must have a statically known size
[00:51:50] +    = help: unsized locals are gated as an unstable feature
[00:51:50] + 
[00:51:50] + error: aborting due to 2 previous errors
[00:51:50] + error: aborting due to 2 previous errors
[00:51:50] 12 
[00:51:50] 13 For more information about this error, try `rustc --explain E0277`.
[00:51:50] 14 
[00:51:50] 
[00:51:50] 
[00:51:50] The actual stderr differed from the expected stderr.
[00:51:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20605/issue-20605.stderr
[00:51:50] To update references, rerun the tests and pass the `--bless` flag
[00:51:50] To only update this specific test, also pass `--test-args issues/issue-20605.rs`
[00:51:50] error: 1 errors occurred comparing output.
[00:51:50] status: exit code: 1
[00:51:50] status: exit code: 1
[00:51:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20605.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20605/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20605/auxiliary" "-A" "unused"
[00:51:50] ------------------------------------------
[00:51:50] 
[00:51:50] ------------------------------------------
[00:51:50] stderr:
[00:51:50] stderr:
[00:51:50] ------------------------------------------
[00:51:50] {"message":"the size for values of type `dyn std::iter::Iterator<Item=&mut u8>` cannot be known at compilation time","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n