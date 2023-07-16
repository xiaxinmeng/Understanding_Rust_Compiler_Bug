plain
[00:47:09] ....................................................................................................
[00:47:11] ....................................................................................................
[00:47:14] ....................................................................................................
[00:47:17] .............................................................i......................................
[00:47:21] ......................................................F.............................................
[00:47:29] ...............................................................................................i....
[00:47:32] ....................................................................................................
[00:47:35] ....................................................................................................
[00:47:38] ....................................................................................................
---
[00:47:41] 13 error[E0531]: cannot find unit struct/variant or constant `Self` in this scope
[00:47:41] -   --> $DIR/self_type_keyword-2.rs:18:9
[00:47:41] +   --> $DIR/self_type_keyword-2.rs:19:9
[00:47:41] 15    |
[00:47:41] 16 LL |         Self => (),
[00:47:41] 
[00:47:41] 18 
[00:47:41] 19 error[E0531]: cannot find unit struct/variant or constant `Self` in this scope
[00:47:41] -   --> $DIR/self_type_keyword-2.rs:20:18
[00:47:41] -   --> $DIR/self_type_keyword-2.rs:20:18
[00:47:41] +   --> $DIR/self_type_keyword-2.rs:22:18
[00:47:41] 21    |
[00:47:41] 22 LL |         Foo { x: Self } => (),
[00:47:41] 
[00:47:41] 
[00:47:41] 31    = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:47:41] 32 
[00:47:41] 33 error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:47:41] +   --> $DIR/self_type_keyword-2.rs:19:9
[00:47:41] 35    |
[00:47:41] 35    |
[00:47:41] 36 LL |         Self => (),
[00:47:41] 
[00:47:41] 
[00:47:41] 39    = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:47:41] 40 
[00:47:41] 41 error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:47:41] +   --> $DIR/self_type_keyword-2.rs:22:18
[00:47:41] 43    |
[00:47:41] 43    |
[00:47:41] 44 LL |         Foo { x: Self } => (),
[00:47:41] 
[00:47:41] 
[00:47:41] The actual stderr differed from the expected stderr.
[00:47:41] The actual stderr differed from the expected stderr.
[00:47:41] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword-2/self_type_keyword-2.stderr
[00:47:41] To update references, rerun the tests and pass the `--bless` flag
[00:47:41] To only update this specific test, also pass `--test-args self/self_type_keyword-2.rs`
[00:47:41] error: 1 errors occurred comparing output.
[00:47:41] status: exit code: 1
[00:47:41] status: exit code: 1
[00:47:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/self_type_keyword-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword-2/auxiliary" "-A" "unused"
[00:47:41] ------------------------------------------
[00:47:41] 
[00:47:41] ------------------------------------------
[00:47:41] stderr:
[00:47:41] stderr:
[00:47:41] ------------------------------------------
[00:47:41] {"message":"unresolved import `self::Self`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n