plain
Resolving deltas: 100% (607738/607738), completed with 4798 local objects.
---
[00:02:13] configure: rust.quiet-tests     := True
---
[00:41:27] ...........................F.....FF..F....................................i.........................
[00:41:34] .................i..................................................................................
[00:41:38] ....................................................................................................
[00:41:42] ....................................................................................................
[00:41:46] ....................................................................................................
[00:41:51] .....................................................................F.........F....................
[00:41:57] ....................................................................................................
[00:42:04] ........................................FF...FFF....................................................
[00:42:11] ...........F................................................................................i.......
[00:42:19] F................................F......F........................i..................................
---
[00:42:43] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Ast)
[00:42:43] + error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:42:43] 2   --> $DIR/borrowck-closures-two-mut.rs:24:24
[00:42:43] 3    |
[00:42:43] 4 LL |     let c1 = to_fn_mut(|| x = 4);
[00:42:43]
[00:42:43] 14    | - first borrow ends here
[00:42:43] 15
[00:42:43] 16 error[E0499]: cannot borrow `x` as mutable more than once at a time (Ast)
[00:42:43] -   --> $DIR/borrowck-closures-two-mut.rs:35:24
[00:42:43] +   --> $DIR/borrowck-closures-two-mut.rs:24:24
[00:42:43] 18    |
[00:42:43] - LL |     let c1 = to_fn_mut(|| set(&mut x));
[00:42:43] -    |                        --          - previous borrow occurs due to use of `x` in closure
[00:42:43] + LL |     let c1 = to_fn_mut(|| x = 4);
[00:42:43] +    |                        -- - previous borrow occurs due to use of `x` in closure
[00:42:43] 21    |                        |
[00:42:43] 22    |                        first mutable borrow occurs here
[00:42:43] - LL |     let c2 = to_fn_mut(|| set(&mut x)); //~ ERROR cannot borrow `x` as mutable more than once
[00:42:43] -    |                        ^^          - borrow occurs due to use of `x` in closure
[00:42:43] + LL |     let c2 = to_fn_mut(|| x = 5); //~ ERROR cannot borrow `x` as mutable more than once
[00:42:43] +    |                        ^^ - borrow occurs due to use of `x` in closure
[00:42:43] 25    |                        |
[00:42:43] 26    |                        second mutable borrow occurs here
[00:42:43] 27 LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:42:43]
[00:42:43] 28 LL | }
[00:42:43] 29    | - first borrow ends here
[00:42:43] 30
[00:42:43] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Ast)
[00:42:43] -   --> $DIR/borrowck-closures-two-mut.rs:42:24
[00:42:43] + error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:42:43] +   --> $DIR/borrowck-closures-two-mut.rs:35:24
[00:42:43] 33    |
[00:42:43] - LL |     let c1 = to_fn_mut(|| x = 5);
[00:42:43] -    |                        -- - previous borrow occurs due to use of `x` in closure
[00:42:43] + LL |     let c1 = to_fn_mut(|| set(&mut x));
[00:42:43] +    |                        --          - previous borrow occurs due to use of `x` in closure
[00:42:43] 36    |                        |
[00:42:43] 37    |                        first mutable borrow occurs here
[00:42:43] 38 LL |     let c2 = to_fn_mut(|| set(&mut x)); //~ ERROR cannot borrow `x` as mutable more than once
[00:42:43]
[00:42:43] 44    | - first borrow ends here
[00:42:43] 45
[00:42:43] 46 error[E0499]: cannot borrow `x` as mutable more than once at a time (Ast)
[00:42:43] -   --> $DIR/borrowck-closures-two-mut.rs:49:24
[00:42:43] +   --> $DIR/borrowck-closures-two-mut.rs:35:24
[00:42:43] 48    |
[00:42:43] - LL |     let c1 = to_fn_mut(|| x = 5);
[00:42:43] -    |                        -- - previous borrow occurs due to use of `x` in closure
[00:42:43] + LL |     let c1 = to_fn_mut(|| set(&mut x));
[00:42:43] +    |                        --          - previous borrow occurs due to use of `x` in closure
[00:42:43] 51    |                        |
[00:42:43] 52    |                        first mutable borrow occurs here
[00:42:43] - LL |     let c2 = to_fn_mut(|| { let _y = to_fn_mut(|| set(&mut x)); }); // (nested closure)
[00:42:43] -    |                        ^^                                  - borrow occurs due to use of `x` in closure
[00:42:43] + LL |     let c2 = to_fn_mut(|| set(&mut x)); //~ ERROR cannot borrow `x` as mutable more than once
[00:42:43] +    |                        ^^          - borrow occurs due to use of `x` in closure
[00:42:43] 55    |                        |
[00:42:43] 56    |                        second mutable borrow occurs here
[00:42:43] - ...
[00:42:43] + LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:42:43] 58 LL | }
[00:42:43] 59    | - first borrow ends here
[00:42:43] 60
[00:42:43]
[00:42:43] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Ast)
[00:42:43] -   --> $DIR/borrowck-closures-two-mut.rs:61:24
[00:42:43] -    |
[00:42:43] - LL |     let c1 = to_fn_mut(|| set(&mut *x.f));
[00:42:43] -    |                        --           - previous borrow occurs due to use of `x` in closure
[00:42:43] -    |                        |
[00:42:43] -    |                        first mutable borrow occurs here
[00:42:43] - LL |     let c2 = to_fn_mut(|| set(&mut *x.f));
[00:42:43] -    |                        ^^           - borrow occurs due to use of `x` in closure
[00:42:43] -    |                        |
[00:42:43] -    |                        second mutable borrow occurs here
[00:42:43] - ...
[00:42:43] - LL | }
[00:42:43] -    | - first borrow ends here
[00:42:43] -
[00:42:43] 76 error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:42:43] -   --> $DIR/borrowck-closures-two-mut.rs:24:24
[00:42:43] +   --> $DIR/borrowck-closures-two-mut.rs:42:24
[00:42:43] 78    |
[00:42:43] - LL |     let c1 = to_fn_mut(|| x = 4);
[00:42:43] + LL |     let c1 = to_fn_mut(|| x = 5);
[00:42:43] 80    |                        -- - previous borrow occurs due to use of `x` in closure
[00:42:43] 81    |                        |
[00:42:43] 82    |                        first mutable borrow occurs here
[00:42:43]
[00:42:43] - LL |     let c2 = to_fn_mut(|| x = 5); //~ ERROR cannot borrow `x` as mutable more than once
[00:42:43] -    |                        ^^ - borrow occurs due to use of `x` in closure
[00:42:43] + LL |     let c2 = to_fn_mut(|| set(&mut x)); //~ ERROR cannot borrow `x` as mutable more than once
[00:42:43] +    |                        ^^          - borrow occurs due to use of `x` in closure
[00:42:43] 85    |                        |
[00:42:43] 86    |                        second mutable borrow occurs here
[00:42:43] 87 LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:42:43]
[00:42:43] 88 LL | }
[00:42:43] 89    | - first borrow ends here
[00:42:43] 90
[00:42:43] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:42:43] -   --> $DIR/borrowck-closures-two-mut.rs:35:24
[00:42:43] + error[E0499]: cannot borrow `x` as mutable more than once at a time (Ast)
[00:42:43] +   --> $DIR/borrowck-closures-two-mut.rs:42:24
[00:42:43] 93    |
[00:42:43] - LL |     let c1 = to_fn_mut(|| set(&mut x));
[00:42:43] -    |                        --          - previous borrow occurs due to use of `x` in closure
[00:42:43] + LL |     let c1 = to_fn_mut(|| x = 5);
[00:42:43] +    |                        -- - previous borrow occurs due to use of `x` in closure
[00:42:43] 96    |                        |
[00:42:43] 97    |                        first mutable borrow occurs here
[00:42:43] 98 LL |     let c2 = to_fn_mut(|| set(&mut x)); //~ ERROR cannot borrow `x` as mutable more than once
[00:42:43]
[00:42:43] 104    | - first borrow ends here
[00:42:43] 105
[00:42:43] 106 error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:42:43] -   --> $DIR/borrowck-closures-two-mut.rs:42:24
[00:42:43] +   --> $DIR/borrowck-closures-two-mut.rs:49:24
[00:42:43] 108    |
[00:42:43] 109 LL |     let c1 = to_fn_mut(|| x = 5);
[00:42:43] 110    |                        -- - previous borrow occurs due to use of `x` in closure
[00:42:43]
[00:42:43] 111    |                        |
[00:42:43] 112    |                        first mutable borrow occurs here
[00:42:43] - LL |     let c2 = to_fn_mut(|| set(&mut x)); //~ ERROR cannot borrow `x` as mutable more than once
[00:42:43] -    |                        ^^          - borrow occurs due to use of `x` in closure
[00:42:43] + LL |     let c2 = to_fn_mut(|| { let _y = to_fn_mut(|| set(&mut x)); }); // (nested closure)
[00:42:43] +    |                        ^^                                  - borrow occurs due to use of `x` in closure
[00:42:43] 115    |                        |
[00:42:43] 116    |                        second mutable borrow occurs here
[00:42:43] - LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:42:43] + ...
[00:42:43] 118 LL | }
[00:42:43] 119    | - first borrow ends here
[00:42:43] 120
[00:42:43]
[00:42:43] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:42:43] + error[E0499]: cannot borrow `x` as mutable more than once at a time (Ast)
[00:42:43] 122   --> $DIR/borrowck-closures-two-mut.rs:49:24
[00:42:43] 123    |
[00:42:43] 124 LL |     let c1 = to_fn_mut(|| x = 5);
[00:42:43]
[00:42:43] 134    | - first borrow ends here
[00:42:43] 135
[00:42:43] 136 error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:42:43] +   --> $DIR/borrowck-closures-two-mut.rs:61:24
[00:42:43] +    |
[00:42:43] + LL |     let c1 = to_fn_mut(|| set(&mut *x.f));
[00:42:43] +    |                        --           - previous borrow occurs due to use of `x` in closure
[00:42:43] +    |                        |
[00:42:43] +    |                        first mutable borrow occurs here
[00:42:43] + LL |     let c2 = to_fn_mut(|| set(&mut *x.f));
[00:42:43] +    |                        ^^           - borrow occurs due to use of `x` in closure
[00:42:43] +    |                        |
[00:42:43] +    |                        second mutable borrow occurs here
[00:42:43] + ...
[00:42:43] + LL | }
[00:42:43] +    | - first borrow ends here
[00:42:43] +
[00:42:43] + error[E0499]: cannot borrow `x` as mutable more than once at a time (Ast)
[00:42:43] 137   --> $DIR/borrowck-closures-two-mut.rs:61:24
[00:42:43] 138    |
[00:42:43] 139 LL |     let c1 = to_fn_mut(|| set(&mut *x.f));
[00:42:43]
[00:42:43]
[00:42:43] The actual stderr differed from the expected stderr.
[00:42:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut.stderr
[00:42:43] To update references, run this command from build directory:
[00:42:43] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'borrowck/borrowck-closures-two-mut.rs'
[00:42:43]
[00:42:43] error: 1 errors occurred comparing output.
[00:42:43] status: exit code: 101
[00:42:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:42:43] {"message":"cannot borrow `x` as mutable more than once at a time (Mir)","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n