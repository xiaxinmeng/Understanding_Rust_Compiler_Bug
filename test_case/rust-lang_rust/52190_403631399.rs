plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:38:33] 
[00:38:33] running 1554 tests
[00:38:39] ....................................F.......F..F...........FF.F...................................i.
[00:38:49] ....................................................................................................
[00:38:52] ....................................................................................................
[00:38:56] ....................................................................................................
[00:38:59] ....................................................................................................
[00:38:59] ....................................................................................................
[00:39:04] .................................F..........F.....................................F.................
[00:39:09] ..................................................................................FFF.F......F......
[00:39:15] ...................................FFF...FFF....F......F.FF.F.......................................
[00:39:20] ....................................................................................................
[00:39:26] ...................................i.......FF.FFFFFFFFFFFFFFFFFFF..FF.F.FFFFFF.FFF.FFF.FF.F..F.FFFFF
[00:39:30] .FF.....FFFFFF.FFFFFF..iF...........................................................................
[00:39:41] ....................................................................................................
[00:39:48] ....................................................................i...............................
[00:39:51] ......................................................
[00:39:51] failures:
[00:39:51] failures:
[00:39:51] 
[00:39:51] ---- [ui] ui/borrowck/borrowck-closures-two-mut.rs stdout ----
[00:39:51] diff of stderr:
[00:39:51] 
[00:39:51] 73 LL | }
[00:39:51] 74    | - first borrow ends here
[00:39:51] 75 
[00:39:51] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |     let c1 = to_fn_mut(|| x = 4);
[00:39:51] -    |                        -- - previous borrow occurs due to use of `x` in closure
[00:39:51] -    |                        first mutable borrow occurs here
[00:39:51] -    |                        first mutable borrow occurs here
[00:39:51] - LL |     let c2 = to_fn_mut(|| x = 5); //~ ERROR cannot borrow `x` as mutable more than once
[00:39:51] -    |                        ^^ - borrow occurs due to use of `x` in closure
[00:39:51] -    |                        second mutable borrow occurs here
[00:39:51] -    |                        second mutable borrow occurs here
[00/~ ERROR cannot borrow `x` as mutable more than once
[00:39:51] -    |                        ^^          - borrow occurs due to use of `x` in closure
[00:39:51] -    |                        second mutable borrow occurs here
[00:39:51] -    |                        second mutable borrow occurs here
[00:39:51] - LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:39:51] - LL |     drop((c1, c2));
[00:39:51] -    |           -- borrow later used here
[00:39:51] - 
[00:39:51] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |     let c1 = to_fn_mut(|| x = 5);
[00:39:51] -    |                        -- - previous borrow occurs due to use of `x` in closure
[00:39:51] -    |                        first mutable borrow occurs here
[00:39:51] -    |                        first mutable borrow occurs here
[00:39:51] - LL |     let c2 = to_fn_mut(|| { let _y = to_fn_mut(|| set(&mut x)); }); // (nested closure)
[00:39:51] -    |                        ^^                                  - borrow occurs due to use of `x` in closure
[00:39:51] -    |                        second mutable borrow occurs here
[00:39:51] - ...
[00:39:51] - ...
[00:39:51] - LL |     drop((c1, c2));
[00:39:51] -    |           -- borrow later used here
[00:39:51] - 
[00:39:51] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |     let c1 = to_fn_mut(|| set(&mut *x.f));
[00:39:51] -    |                        --           - previous borrow occurs due to use of `x` in closure
[00:39:51] -    |                        first mutable borrow occurs here
[00:39:51] -    |                        first mutable borrow occurs here
[00:39:51] - LL |     let c2 = to_fn_mut(|| set(&mut *x.f));
[00:39:51] -    |                        ^^           - borrow occurs due to use of `x` in closure
[00:39:51] -    |                        second mutable borrow occurs here
[00:39:51] - ...
[00:39:51] - ...
[00:39:51] - LL |     drop((c1, c2));
[00:39:51] -    |           -- borrow later used here
[00:39:51] - error: aborting due to 10 previous errors
[00:39:51] + error: aborting due to 5 previous errors
[00:39:51] 152 
[00:39:51] 153 For more information about this error, try `rustc --explain E0499`.
[00:39:51] 153 For more information about this error, try `rustc --explain E0499`.
[00:39:51] 154 
[00:39:51] 
[00:39:51] 
[00:39:51] The actual stderr differed from the expected stderr.
[00:39:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut/borrowck-closures-two-mut.stderr
[00:39:51] To update references, rerun the tests and pass the `--bless` flag
[00:39:51] To only update this specific test, also pass `--test-args borrowck/borrowck-closures-two-mut.rs`
[00:39:51] error: 1 errors occurred comparing output.
[00:39:51] status: exit code: 101
[00:39:51] status: exit code: 101
[00:39:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut/auxiliary" "-A" "unused"
[00:39:51] ------------------------------------------
[00:39:51] 
[00:39:51] ------------------------------------------
[00:39:51] stderr:
[00:39:51] stderr:
[00:39:51] ------------------------------------------
[00:39:51] {"message":"cannot borrow `x` as mutable more than once at a time (Ast)","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n