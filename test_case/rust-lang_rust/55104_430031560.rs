plain
[00:44:55] .................................................................................................... 2000/4602
[00:44:59] .................................................................................................... 2100/4602
[00:45:02] .................................................................................................... 2200/4602
[00:45:06] .................................................................................................... 2300/4602
[00:45:10] .......................................F............................................................ 2400/4602
[00:45:17] .................................................................................................... 2600/4602
[00:45:20] .................................................................................................... 2700/4602
[00:45:23] .................................................................................................... 2800/4602
[00:45:27] .................................................................................................... 2900/4602
---
[00:46:20] 
[00:46:20] ---- [ui] ui/issues/issue-34229.rs stdout ----
[00:46:20] diff of stderr:
[00:46:20] 
[00:46:20] 1 error[E0277]: can't compare `Comparable` with `Comparable`
[00:46:20] +   --> $DIR/issue-34229.rs:2:46
[00:46:20] 3    |
[00:46:20] 3    |
[00:46:20] 4 LL | #[derive(PartialEq, PartialOrd)] struct Nope(Comparable);
[00:46:20] 5    |                                              ^^^^^^^^^^ no implementation for `Comparable < Comparable` and `Comparable > Comparable`
[00:46:20] 8    = note: required by `std::cmp::PartialOrd::partial_cmp`
[00:46:20] 9 
[00:46:20] 10 error: aborting due to previous error
[00:46:20] + 
[00:46:20] + 
[00:46:20] + For more information about this error, try `rustc --explain E0277`.
[00:46:20] 11 
[00:46:20] 
[00:46:20] 
[00:46:20] The actual stderr differed from the expected stderr.
[00:46:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34229/issue-34229.stderr
[00:46:20] To update references, rerun the tests and pass the `--bless` flag
[00:46:20] To only update this specific test, also pass `--test-args issues/issue-34229.rs`
[00:46:20] error: 1 errors occurred comparing output.
[00:46:20] status: exit code: 1
[00:46:20] status: exit code: 1
[00:46:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34229.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34229/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34229/auxiliary" "-A" "unused"
[00:46:20] ------------------------------------------
[00:46:20] 
[00:46:20] ------------------------------------------
[00:46:20] stderr:
[00:46:20] stderr:
[00:46:20] ------------------------------------------
[00:46:20] {"message":"can't compare `Comparable` with `Comparable`","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n