plain
[00:47:09] ...................................................................................iii..............
[00:47:12] ......................................................................i..........i..................
[00:47:15] ........i...........................................................................................
[00:47:20] ....................................................................................................
[00:47:23] ........F.....i.......i.............................................................................
[00:47:29] ....................................................................................................
[00:47:31] ....................................................................................................
[00:47:33] ....................................................................................................
[00:47:35] ....................................................................................................
---
[00:47:51] ....................................................................................................
[00:47:55] ....................................................................................................
[00:47:57] ............................i.......................................................................
[00:48:00] ....................................................................................................
[00:48:03] .............................................................................iiiiiiiii..............
[00:48:08] i...................................................................................................
[00:48:12] ....................................................................................................
[00:48:15] ..........................................................i.........................................
[00:48:17] ....................................................................................................
---
[00:48:58] 
[00:48:58] ---- [ui (nll)] ui/consts/min_const_fn/promotion.rs stdout ----
[00:48:58] diff of stderr:
[00:48:58] 
[00:48:58] 12 error[E0597]: borrowed value does not live long enough
[00:48:58] 14    |
[00:48:58] 14    |
[00:48:58] - LL |     let a: &'static Cell<i32> = &foo4();
[00:48:58] + LL |     let a: &'static Cell<i32> = &foo4(); // doesn't error on HIR borrowck
[00:48:58] 16    |                                  ^^^^^^ temporary value does not live long enough
[00:48:58] 17 ...
[00:48:58] 18 LL | }
[00:48:58] 
[00:48:58] 23 error[E0597]: borrowed value does not live long enough
[00:48:58] 25    |
[00:48:58] 25    |
[00:48:58] - LL |     let a: &'static Option<Cell<i32>> = &foo5();
[00:48:58] + LL |     let a: &'static Option<Cell<i32>> = &foo5(); // doesn't error on HIR borrowck
[00:48:58] 27    |                                          ^^^^^^ temporary value does not live long enough
[00:48:58] - LL |     let a: &'static Option<Cell<i32>> = &foo6();
[00:48:58] + LL |     let a: &'static Option<Cell<i32>> = &foo6(); // doesn't error on HIR borrowck
[00:48:58] 29 LL | }
[00:48:58] 30    | - temporary value only lives until here
[00:48:58] 
[00:48:58] 
[00:48:58] 34 error[E0597]: borrowed value does not live long enough
[00:48:58] 36    |
[00:48:58] 36    |
[00:48:58] - LL |     let a: &'static Option<Cell<i32>> = &foo6();
[00:48:58] + LL |     let a: &'static Option<Cell<i32>> = &foo6(); // doesn't error on HIR borrowck
[00:48:58] 38    |                                          ^^^^^^ temporary value does not live long enough
[00:48:58] 39 LL | }
[00:48:58] 40    | - temporary value only lives until here
[00:48:58] 
[00:48:58] The actual stderr differed from the expected stderr.
[00:48:58] The actual stderr differed from the expected stderr.
[00:48:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/promotion.nll/promotion.nll.stderr
[00:48:58] To update references, rerun the tests and pass the `--bless` flag
[00:48:58] To only update this specific test, also pass `--test-args consts/min_const_fn/promotion.rs`
[00:48:58] error: 1 errors occurred comparing output.
[00:48:58] status: exit code: 1
[00:48:58] status: exit code: 1
[00:48:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/promotion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/promotion.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/promotion.nll/auxiliary" "-A" "unused"
[00:48:58] ------------------------------------------
[00:48:58] 
[00:48:58] ------------------------------------------
[00:48:58] stderr:
[00:48:58] stderr:
[00:48:58] ------------------------------------------
[00:48:58] {"message":"borrowed value does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n