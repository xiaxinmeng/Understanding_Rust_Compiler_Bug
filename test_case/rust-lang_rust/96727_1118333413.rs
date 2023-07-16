plain
---- [ui] src/test/ui/lazy-type-alias-impl-trait/branches3.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lazy-type-alias-impl-trait/branches3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lazy-type-alias-impl-trait/branches3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lazy-type-alias-impl-trait/branches3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0601]: `main` function not found in crate `branches3`
   |
   |
LL | fn bar2() -> impl FnOnce(&'static str) -> usize { if true { |s| s.len() } else { panic!() } }
   |                                                                                              ^ consider adding a `main` function to `/checkout/src/test/ui/lazy-type-alias-impl-trait/branches3.rs`
error[E0282]: type annotations needed
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> /checkout/src/test/ui/lazy-type-alias-impl-trait/branches3.rs:9:30
   |
   |
LL | fn foo() -> Foo { if true { |s| s.len() } else { panic!() } }
   |                              ^ consider giving this closure parameter a type
   = note: type must be known at this point

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/lazy-type-alias-impl-trait/branches3.rs:10:30
  --> /checkout/src/test/ui/lazy-type-alias-impl-trait/branches3.rs:10:30
   |
LL | fn bar() -> Bar { if true { |s| s.len() } else { panic!() } }
   |                              ^ consider giving this closure parameter a type
   = note: type must be known at this point

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/lazy-type-alias-impl-trait/branches3.rs:13:65
  --> /checkout/src/test/ui/lazy-type-alias-impl-trait/branches3.rs:13:65
   |
LL | fn foo2() -> impl for<'a> FnOnce(&'a str) -> usize { if true { |s| s.len() } else { panic!() } }
   |                                                                 ^ consider giving this closure parameter a type
   = note: type must be known at this point

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/lazy-type-alias-impl-trait/branches3.rs:14:62
  --> /checkout/src/test/ui/lazy-type-alias-impl-trait/branches3.rs:14:62
   |
LL | fn bar2() -> impl FnOnce(&'static str) -> usize { if true { |s| s.len() } else { panic!() } }
   |                                                              ^ consider giving this closure parameter a type
   = note: type must be known at this point

error: aborting due to 5 previous errors

