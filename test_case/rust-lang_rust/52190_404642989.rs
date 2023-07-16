plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:35] 
[00:43:35] running 1560 tests
[00:43:38] .....................................F......F..F............FFF...................................i.
[00:43:44] ....................................................................................................
[00:43:46] ....................................................................................................
[00:43:49] ....................................................................................................
[00:43:51] ....................................................................................................
[00:43:51] ....................................................................................................
[00:43:54] .................................F...........F.......................................F..............
[00:43:57] ....................................................................................F.FF.F.....F....
[00:44:00] .....................................F.FF...FFF....F.....F....FF.F..................................
[00:44:03] ....................................................................................................
[00:44:06] ......................................i........FFF.FFFFFFFFFFFFFFFFFF.F.F.FF.FFFFF.FFF.FFF.FF.F...FF
[00:44:09] FFFFF.F.....FFFFFFFF.FFFF..iF.......................................................................
[00:44:16] ....................................................................................................
occurs here
occurs here
[00:44:22] - LL |     let c2 = to_fn_mut(|| set(&mut x)); //~ ERROR cannot borrow `x` as mutable more than once
[00:44:22] -    |                        ^^          - borrow occurs due to use of `x` in closure
[00:44:22] -    |                        second mutable borrow occurs here
[00:44:22] -    |                        second mutable borrow occurs here
[00:44:22] - LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:44:22] - LL |     drop((c1, c2));
[00:44:22] -    |           -- borrow later used here
[00:44:22] - 
[00:44:22] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:44:22] -    |
[00:44:22] -    |
[00:44:22] - LL |     let c1 = to_fn_mut(|| x = 5);
[00:44:22] -    |                        -- - previous borrow occurs due to use of `x` in closure
[00:44:22] -    |                        first mutable borrow occurs here
[00:44:22] -    |                        first mutable borrow occurs here
[00:44:22] - LL |     let c2 = to_fn_mut(|| { let _y = to_fn_mut(|| set(&mut x)); }); // (nested closure)
[00:44:22] -    |                        ^^                                  - borrow occurs due to use of `x` in closure
[00:44:22] -    |                        second mutable borrow occurs here
[00:44:22] - ...
[00:44:22] - ...
[00:44:22] - LL |     drop((c1, c2));
[00:44:22] -    |           -- borrow later used here
[00:44:22] - 
[00:44:22] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:44:22] -    |
[00:44:22] -    |
[00:44:22] - LL |     let c1 = to_fn_mut(|| set(&mut *x.f));
[00:44:22] -    |                        --           - previous borrow occurs due to use of `x` in closure
[00:44:22] -    |                        first mutable borrow occurs here
[00:44:22] -    |                        first mutable borrow occurs here
[00:44:22] - LL |     let c2 = to_fn_mut(|| set(&mut *x.f));
[00:44:22] -    |                        ^^           - borrow occurs due to use of `x` in closure
[00:44:22] -    |                        second mutable borrow occurs here
[00:44:22] - ...
[00:44:22] - ...
[00:44:22] - LL |     drop((c1, c2));
[00:44:22] -    |           -- borrow later used here
[00:44:22] - error: aborting due to 10 previous errors
[00:44:22] + error: aborting due to 5 previous errors
[00:44:22] 152 
[00:44:22] 153 For more information about this error, try `rustc --explain E0499`.
[00:44:22] 153 For more information about this error, try `rustc --explain E0499`.
[00:44:22] 154 
[00:44:22] 
[00:44:22] 
[00:44:22] The actual stderr differed from the expected stderr.
[00:44:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut/borrowck-closures-two-mut.stderr
[00:44:22] To update references, rerun the tests and pass the `--bless` flag
[00:44:22] To only update this specific test, also pass `--test-args borrowck/borrowck-closures-two-mut.rs`
[00:44:22] error: 1 errors occurred comparing output.
[00:44:22] status: exit code: 101
[00:44:22] status: exit code: 101
[00:44:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut/auxiliary" "-A" "unused"
[00:44:22] ------------------------------------------
[00:44:22] 
[00:44:22] ------------------------------------------
[00:44:22] stderr:
[00:44:22] stderr:
[00:44:22] ------------------------------------------
[00:44:22] {"message":"cannot borrow `x` as mutable more than once at a time (Ast)","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n