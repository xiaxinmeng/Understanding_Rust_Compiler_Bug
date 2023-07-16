plain
[00:46:07] ....................................................................................................
[00:46:09] ....................................................................................................
[00:46:11] ....................................................................................................
[00:46:14] ...............................i..................................................................i.
[00:46:17] ...............................................................................F....................
[00:46:23] ....................................................................................i...............
[00:46:26] ....................................................................................................
[00:46:29] ....................................................................................................
[00:46:32] ....................................................................................................
---
[00:51:19] ....................................................................................................
[00:51:26] ....................................................................................................
[00:51:36] ....................................................................................................
[00:51:39] ....................................................................................................
[00:51:42] .................F..................................................................................
[00:51:46] .................................F.........................i........................................
[00:51:49] ..........F.........................................................................................
[00:51:56] ....................................................................................................
[00:51:58] .............i............................................................
[00:51:58] failures:
[00:51:58] 
[00:51:58] 
[00:51:58] ---- [ui] ui/impl-trait/no-method-suggested-traits.rs stdout ----
[00:51:58] diff of stderr:
[00:51:58] 
[00:51:58] 5    |          ^^^^^^
[00:51:58] 6    |
[00:51:58] 7    = help: items from traits can only be used if the trait is in scope
[00:51:58] +    = help: did you mean `method2`?
[00:51:58] 8 help: the following traits are implemented but not in scope, perhaps add a `use` for one of them:
[00:51:58] 9    |
[00:51:58] 10 LL | use foo::Bar;
[00:51:58] 23    |                                            ^^^^^^
[00:51:58] 24    |
[00:51:58] 25    = help: items from traits can only be used if the trait is in scope
[00:51:58] 25    = help: items from traits can only be used if the trait is in scope
[00:51:58] +    = help: did you mean `method2`?
[00:51:58] 26 help: the following traits are implemented but not in scope, perhaps add a `use` for one of them:
[00:51:58] 27    |
[00:51:58] 28 LL | use foo::Bar;
[00:51:58] 41    |         ^^^^^^
[00:51:58] 42    |
[00:51:58] 43    = help: items from traits can only be used if the trait is in scope
[00:51:58] 43    = help: items from traits can only be used if the trait is in scope
[00:51:58] +    = help: did you mean `method2`?
[00:51:58] 45    |
[00:51:58] 45    |
[00:51:58] 46 LL | use foo::Bar;
[00:51:58] 53    |                                           ^^^^^^
[00:51:58] 54    |
[00:51:58] 55    = help: items from traits can only be used if the trait is in scope
[00:51:58] 55    = help: items from traits can only be used if the trait is in scope
[00:51:58] +    = help: did you mean `method2`?
[00:51:58] 57    |
[00:51:58] 57    |
[00:51:58] 58 LL | use foo::Bar;
[00:51:58] 65    |          ^^^^^^
[00:51:58] 66    |
[00:51:58] 67    = help: items from traits can only be used if the trait is in scope
[00:51:58] 67    = help: items from traits can only be used if the trait is in scope
[00:51:58] +    = help: did you mean `method3`?
[00:51:58] 69    |
[00:51:58] 69    |
[00:51:58] 70 LL | use no_method_suggested_traits::foo::PubPub;
[00:51:58] 77    |                                            ^^^^^^
[00:51:58] 78    |
[00:51:58] 79    = help: items from traits can only be used if the trait is in scope
[00:51:58] 79    = help: items from traits can only be used if the trait is in scope
[00:51:58] +    = help: did you mean `method3`?
[00:51:58] 81    |
[00:51:58] 81    |
[00:51:58] 82 LL | use no_method_suggested_traits::foo::PubPub;
[00:51:58] 
[00:51:58] The actual stderr differed from the expected stderr.
[00:51:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/no-method-suggested-traits.stderr
[00:51:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/no-method-suggested-traits.stderr
[00:51:58] To update references, rerun the tests and pass the `--bless` flag
[00:51:58] To only update this specific test, also pass `--test-args impl-trait/no-method-suggested-traits.rs`
[00:51:58] error: 1 errors occurred comparing output.
[00:51:58] status: exit code: 1
[00:51:58] status: exit code: 1
[00:51:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/auxiliary" "-A" "unused"
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] ------------------------------------------
[00:51:58] stderr:
[00:51:58] stderr:
[00:51:58] ------------------------------------------
[00:51:58] {"message":"no method named `method` found for type `u32` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n