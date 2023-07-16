plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:45:55] 
[00:45:55] running 1561 tests
[00:45:58] ....................................F.......F.F...........F..FF...................................i.
[00:46:05] ....................................................................................................
[00:46:07] ....................................................................................................
[00:46:09] ....................................................................................................
[00:46:11] ....................................................................................................
[00:46:11] ....................................................................................................
[00:46:15] ..................................F..........F......................................F...............
[00:46:18] ....................................................................................F.FFF......F....
[00:46:21] ......................................FFF...FFF...F......F..FF..F...................................
[00:46:24] ....................................................................................................
[00:46:28] ......................................i........FF.FFFFFFFFFFFFFFFFFFF..FF.FF.FFFFF.FFF.FFFF.F.FF...F
[00:46:31] FFFF.FF.....FFFFFFF.FFFFF.Fi........................................................................
[00:46:38] ....................................................................................................
[00:46:38] ....................................................................................................
[00:46:42] ...........................................F...............................i........................
[00:46:44] failures:
[00:46:44] 
[00:46:44] ---- [ui] ui/borrowck/borrowck-closures-two-mut.rs stdout ----
[00:46:44] diff of stderr:
[00:46:44] diff of stderr:
[00:46:44] 
[00:46:44] 73 LL | }
[00:46:44] 74    | - first borrow ends here
[00:46:44] 75 
[00:46:44] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:46:44] -    |
[00:46:44] -    |
[00:46:44] - LL |     let c1 = to_fn_mut(|| x = 4);
[00:46:44] -    |                        -- - previous borrow occurs due to use of `x` in closure
[00:46:44] -    |                        first mutable borrow occurs here
[00:46:44] -    |                        first mutable borrow occurs here
[00:46:44] - LL |     let c2 = to_fn_mut(|| x = 5); //~ ERROR cannot borrow `x` as mutable more than once
[00:46:44] -    |                        ^^ - borrow occurs due to use of `x` in closure
[00:46:44] -    |                        second mutable borrow occurs here
[00:46:44] -    |                        second mutable borrow occurs here
[00:46:44] - LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:46:44] - LL |     drop((c1, c2));
[00:46:44] -    |           -- borrow later used here
[00:46:44] - 
[00:46:44] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:46:44] -    |
[00:46:44] -    |
[00:46:44] - LL |     let c1 = to_fn_mut(|| set(&mut x));
[00:46:44] -    |                        --          - previous borrow occurs due to use of `x` in closure
[00:46:44] -    |                        first mutable borrow occurs here
[00:46:44] -    |                        first mutable borrow occurs here
[00:46:44] - LL |     let c2 = to_fn_mut(|| set(&mut x)); //~ ERROR cannot borrow `x` as mutable more than once
[00:46:44] -    |                        ^^          - borrow occurs due to use of `x` in closure
[00:46:44] -    |                        second mutable borrow occurs here
[00:46:44] -    |                        second mutable borrow occurs here
[00:46:44] - LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:46:44] - LL |     drop((c1, c2));
[00:46:44] -    |           -- borrow later used here
[00:46:44] - 
[00:46:44] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:46:44] -    |
[00:46:44] -    |
[00:46:44] - LL |     let c1 = to_fn_mut(|| x = 5);
[00:46:44] -    |                        -- - previous borrow occurs due to use of `x` in closure
[00:46:44] -    |                        first mutable borrow occurs here
[00:46:44] -    |                        first mutable borrow occurs here
[00:46:44] - LL |     let c2 = to_fn_mut(|| set(&mut x)); //~ ERROR cannot borrow `x` as mutable more than once
[00:46:44] -    |                        ^^          - borrow occurs due to use of `x` in closure
[00:46:44] -    |                        second mutable borrow occurs here
[00:46:44] -    |                        second mutable borrow occurs here
[00:46:44] - LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:46:44] - LL |     drop((c1, c2));
[00:46:44] -    |           -- borrow later used here
[00:46:44] - 
[00:46:44] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:46:44] -    |
[00:46:44] -    |
[00:46:44] - LL |     let c1 = to_fn_mut(|| x = 5);
[00:46:44] -    |                        -- - previous borrow occurs due to use of `x` in closure
[00:46:44] -    |                        first mutable borrow occurs here
[00:46:44] -    |                        first mutable borrow occurs here
[00:46:44] - LL |     let c2 = to_fn_mut(|| { let _y = to_fn_mut(|| set(&mut x)); }); // (nested closure)
[00:46:44] -    |                        ^^                                  - borrow occurs due to use of `x` in closure
[00:46:44] -    |                        second mutable borrow occurs here
[00:46:44] - ...
[00:46:44] - ...
[00:46:44] - LL |     drop((c1, c2));
[00:46:44] -    |           -- borrow later used here
[00:46:44] - 
[00:46:44] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:46:44] -    |
[00:46:44] -    |
[00:46:44] - LL |     let c1 = to_fn_mut(|| set(&mut *x.f));
[00:46:44] -    |                        --           - previous borrow occurs due to use of `x` in closure
[00:46:44] -    |                        first mutable borrow occurs here
[00:46:44] -    |                        first mutable borrow occurs here
[00:46:44] - LL |     let c2 = to_fn_mut(|| set(&mut *x.f));
[00:46:44] -    |                        ^^           - borrow occurs due to use of `x` in closure
[00:46:44] -    |                        second mutable borrow occurs here
[00:46:44] - ...
[00:46:44] - ...
[00:46:44] - LL |     drop((c1, c2));
[00:46:44] -    |           -- borrow later used here
[00:46:44] - error: aborting due to 10 previous errors
[00:46:44] + error: aborting due to 5 previous errors
[00:46:44] 152 
[00:46:44] 153 For more information about this error, try `rustc --explain E0499`.
[00:46:44] 153 For more information about this error, try `rustc --explain E0499`.
[00:46:44] 154 
[00:46:44] 
[00:46:44] 
[00:46:44] The actual stderr differed from the expected stderr.
[00:46:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut/borrowck-closures-two-mut.stderr
[00:46:44] To update references, rerun the tests and pass the `--bless` flag
[00:46:44] To only update this specific test, also pass `--test-args borrowck/borrowck-closures-two-mut.rs`
[00:46:44] error: 1 errors occurred comparing output.
[00:46:44] status: exit code: 101
[00:46:44] status: exit code: 101
[00:46:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut/auxiliary" "-A" "unused"
[00:46:44] ------------------------------------------
[00:46:44] 
[00:46:44] ------------------------------------------
[00:46:44] stderr:
[00:46:44] stderr:
[00:46:44] ------------------------------------------
[00:46:44] {"message":"cannot borrow `x` as mutable more than once at a time (Ast)","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n