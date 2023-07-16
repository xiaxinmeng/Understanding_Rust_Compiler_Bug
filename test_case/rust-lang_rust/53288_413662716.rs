plain
[00:45:57] ....................................................................................................
[00:46:00] ....................................................................................................
[00:46:04] ........i...........................................................................................
[00:46:08] ..............................................................................................i.....
[00:46:11] ..i.................................................................................................
[00:46:15] ........ii.iii.......................F..............................................................
[00:46:19] ....................................................................................................
[00:46:22] ....................................................................................................
[00:46:24] ....................................................................................................
[00:46:26] ............................................................................................i.......
---
[00:47:49] 
[00:47:49] ---- [ui] ui/dropck/dropck-union.rs stdout ----
[00:47:49] diff of stderr:
[00:47:49] 
[00:47:49] 1 error[E0597]: `v` does not live long enough
[00:47:49] +   --> $DIR/dropck-union.rs:49:19
[00:47:49] 3    |
[00:47:49] 3    |
[00:47:49] 4 LL |     v.0.set(Some(&v)); //~ ERROR: `v` does not live long enough
[00:47:49] 5    |                   ^ borrowed value does not live long enough
[00:47:49] 
[00:47:49] The actual stderr differed from the expected stderr.
[00:47:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck-union/dropck-union.stderr
[00:47:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck-union/dropck-union.stderr
[00:47:49] To update references, rerun the tests and pass the `--bless` flag
[00:47:49] To only update this specific test, also pass `--test-args dropck/dropck-union.rs`
[00:47:49] error: 1 errors occurred comparing output.
[00:47:49] status: exit code: 1
[00:47:49] status: exit code: 1
[00:47:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dropck/dropck-union.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck-union/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck-union/auxiliary" "-A" "unused"
[00:47:49] ------------------------------------------
[00:47:49] 
[00:47:49] ------------------------------------------
[00:47:49] stderr:
[00:47:49] stderr:
[00:47:49] ------------------------------------------
[00:47:49] {"message":"`v` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n