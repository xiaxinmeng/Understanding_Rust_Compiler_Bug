plain
[00:57:58] ....................................................................................................
[00:58:01] .....................................................i..............................................
[00:58:04] ....................................................................................................
[00:58:07] ....................................................................................................
[00:58:10] .iiiiiiiii..........................................................................................
[00:58:17] ....................................................................................................
[00:58:20] .....................................................................................i..............
[00:58:23] ....................................................................................................
[00:58:26] ........................................i.i..ii.....................................................
[00:58:26] ........................................i.i..ii.....................................................
[00:58:29] ....................................................................................................
[00:58:32] ........................................................i......................i....................
[00:58:36] ....................................................................................................
[00:58:39] ............................................................................................i..i..i.
[00:58:41] ....................i...i.............ii.......................ii........................iiiii......
[00:58:45] .....................................................................iiiii......................i...
[00:58:51] .........................................F..........................................................
[00:59:07] ....................................................................................................
[00:59:15] ....................................................................................................
[00:59:23] ....................................................................................................
[00:59:37] ....................................................................................................
---
[01:03:52] .........i.ii.ii.ii.............................i...................................................
[01:03:52] .........
[01:03:52] failures:
[01:03:52] 
[01:03:52] ---- [ui (nll)] ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs stdout ----
[01:03:52] 
[01:03:52] 
[01:03:52] 9    = note: move occurs because `a` has type `NoCopy`, which does not implement the `Copy` trait
[01:03:52] 11 
[01:03:52] - error[E0382]: borrow of moved value: `a`
[01:03:52] -   --> $DIR/dbg-macro-move-semantics.rs:11:18
[01:03:52] -    |
[01:03:52] -    |
[01:03:52] - LL |     let _ = dbg!(a);
[01:03:52] -    |             ------- value moved here
[01:03:52] - LL |     let _ = dbg!(a);
[01:03:52] -    |                  ^ value borrowed here after move
[01:03:52] -    |
[01:03:52] -    = note: move occurs because `a` has type `NoCopy`, which does not implement the `Copy` trait
[01:03:52] - 
[01:03:52] - error[E0382]: use of moved value: `a`
[01:03:52] -   --> $DIR/dbg-macro-move-semantics.rs:11:13
[01:03:52] -    |
[01:03:52] -    |
[01:03:52] - LL |     let _ = dbg!(a);
[01:03:52] -    |             ------- value moved here
[01:03:52] - LL |     let _ = dbg!(a);
[01:03:52] -    |             ^^^^^^^ value used here after move
[01:03:52] -    |
[01:03:52] -    = note: move occurs because `a` has type `NoCopy`, which does not implement the `Copy` trait
[01:03:52] - 
[01:03:52] - error: aborting due to 3 previous errors
[01:03:52] + error: aborting due to previous error
[01:03:52] 35 
[01:03:52] 35 
[01:03:52] 36 For more information about this error, try `rustc --explain E0382`.
[01:03:52] 37 
[01:03:52] 
[01:03:52] 
[01:03:52] The actual stderr differed from the expected stderr.
[01:03:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.nll/dbg-macro-move-semantics.nll.stderr
[01:03:52] To update references, rerun the tests and pass the `--bless` flag
[01:03:52] To only update this specific test, also pass `--test-args rfc-2361-dbg-macro/dbg-macro-move-semantics.rs`
[01:03:52] error: 1 errors occurred comparing output.
[01:03:52] status: exit code: 1
[01:03:52] status: exit code: 1
[01:03:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.nll/auxiliary" "-A" "unused"
[01:03:52] ------------------------------------------
[01:03:52] 
[01:03:52] ------------------------------------------
[01:03:52] stderr:
[01:03:52] stderr:
[01:03:52] ------------------------------------------
[01:03:52] {"message":"use of moved value: `a`","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n