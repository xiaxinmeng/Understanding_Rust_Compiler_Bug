plain

9    |
10 LL |     fn foo() {}
11    |        ^^^
- help: remove these generics
+ help: consider moving this generic argument to the `A` trait, which takes up to 1 argument
13    |
14 LL -     let _ = A::foo::<S>();
- LL +     let _ = A::foo();
+ LL +     let _ = A::<S>::foo();
16    |
- help: consider moving this generic argument to the `A` trait, which takes up to 1 argument
+ help: remove these generics
18    |
19 LL -     let _ = A::foo::<S>();
- LL +     let _ = A::<S>::foo();
+ LL +     let _ = A::foo();
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
22 
22 
23 error[E0107]: this associated function takes 0 generic arguments but 2 generic arguments were supplied
31    |
32 LL |     fn bar() {}
33    |        ^^^
- help: remove these generics
- help: remove these generics
+ help: consider moving these generic arguments to the `B` trait, which takes up to 2 arguments
35    |
36 LL -     let _ = B::bar::<S, S>();
- LL +     let _ = B::bar();
+ LL +     let _ = B::<S, S>::bar();
38    |
- help: consider moving these generic arguments to the `B` trait, which takes up to 2 arguments
+ help: remove these generics
40    |
41 LL -     let _ = B::bar::<S, S>();
- LL +     let _ = B::<S, S>::bar();
+ LL +     let _ = B::bar();
44 
44 
45 error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-89064/issue-89064.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-89064.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-89064.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-89064" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-89064/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |     let _ = A::foo::<S>();
   |                ^^^ expected 0 generic arguments
note: associated function defined here, with 0 generic parameters
  --> /checkout/src/test/ui/suggestions/issue-89064.rs:4:8
   |
LL |     fn foo() {}
LL |     fn foo() {}
   |        ^^^
help: consider moving this generic argument to the `A` trait, which takes up to 1 argument
   |
LL -     let _ = A::foo::<S>();
LL +     let _ = A::<S>::foo();
help: remove these generics
   |
   |
LL -     let _ = A::foo::<S>();
LL +     let _ = A::foo();


error[E0107]: this associated function takes 0 generic arguments but 2 generic arguments were supplied
   |
   |
LL |     let _ = B::bar::<S, S>();
   |                ^^^ expected 0 generic arguments
note: associated function defined here, with 0 generic parameters
  --> /checkout/src/test/ui/suggestions/issue-89064.rs:8:8
   |
LL |     fn bar() {}
LL |     fn bar() {}
   |        ^^^
help: consider moving these generic arguments to the `B` trait, which takes up to 2 arguments
   |
LL -     let _ = B::bar::<S, S>();
LL +     let _ = B::<S, S>::bar();
help: remove these generics
   |
   |
LL -     let _ = B::bar::<S, S>();
LL +     let _ = B::bar();


error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |     let _ = A::<S>::foo::<S>();
   |                     ^^^----- help: remove these generics
   |                     expected 0 generic arguments
   |
note: associated function defined here, with 0 generic parameters
  --> /checkout/src/test/ui/suggestions/issue-89064.rs:4:8
