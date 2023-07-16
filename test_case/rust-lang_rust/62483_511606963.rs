
failures:

---- [ui] ui/issues/issue-31173.rs stdout ----
diff of stderr:

14	   |          ^^^^^^^
15	   |
16	   = note: the method `collect` exists but the following trait bounds were not satisfied:
-	           `&mut std::iter::Cloned<std::iter::TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:6:39: 9:6 found_e:_]>> : std::iter::Iterator`
18	           `std::iter::Cloned<std::iter::TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:6:39: 9:6 found_e:_]>> : std::iter::Iterator`
19	
20	error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173/issue-31173.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-31173.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31173.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173/auxiliary" "-A" "unused"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<std::iter::TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6 found_e:_]> as std::iter::Iterator>::Item == &_`
  --> /checkout/src/test/ui/issues/issue-31173.rs:10:10
   |
LL |         .cloned()
   |          ^^^^^^ expected u8, found reference
   |
   = note: expected type `u8`
              found type `&_`

error[E0599]: no method named `collect` found for type `std::iter::Cloned<std::iter::TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6 found_e:_]>>` in the current scope
  --> /checkout/src/test/ui/issues/issue-31173.rs:14:10
   |
LL |         .collect(); //~ ERROR no method named `collect`
   |          ^^^^^^^
   |
   = note: the method `collect` exists but the following trait bounds were not satisfied:
           `std::iter::Cloned<std::iter::TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6 found_e:_]>> : std::iter::Iterator`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0271, E0599.
For more information about an error, try `rustc --explain E0271`.

------------------------------------------


---- [ui] ui/methods/method-call-err-msg.rs stdout ----
diff of stderr:

34	LL |      .take()
35	   |       ^^^^
36	   |
-	   = note: the method `take` exists but the following trait bounds were not satisfied:
-	           `&mut Foo : std::iter::Iterator`
39	   = help: items from traits can only be used if the trait is implemented and in scope
40	   = note: the following traits define an item `take`, perhaps you need to implement one of them:
41	           candidate #1: `std::io::Read`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-err-msg/method-call-err-msg.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args methods/method-call-err-msg.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-call-err-msg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-err-msg" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-err-msg/auxiliary" "-A" "unused"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0061]: this function takes 0 parameters but 1 parameter was supplied
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:12:7
   |
LL |     fn zero(self) -> Foo { self }
   |     -------------------- defined here
...
LL |     x.zero(0)   //~ ERROR this function takes 0 parameters but 1 parameter was supplied
   |       ^^^^ expected 0 parameters

error[E0061]: this function takes 1 parameter but 0 parameters were supplied
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:13:7
   |
LL |     fn one(self, _: isize) -> Foo { self }
   |     ----------------------------- defined here
...
LL |      .one()     //~ ERROR this function takes 1 parameter but 0 parameters were supplied
   |       ^^^ expected 1 parameter

error[E0061]: this function takes 2 parameters but 1 parameter was supplied
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:14:7
   |
LL |     fn two(self, _: isize, _: isize) -> Foo { self }
   |     --------------------------------------- defined here
...
LL |      .two(0);   //~ ERROR this function takes 2 parameters but 1 parameter was supplied
   |       ^^^ expected 2 parameters

error[E0599]: no method named `take` found for type `Foo` in the current scope
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:18:7
   |
LL | pub struct Foo;
   | --------------- method `take` not found for this
...
LL |      .take()    //~ ERROR no method named `take` found for type `Foo` in the current scope
   |       ^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following traits define an item `take`, perhaps you need to implement one of them:
           candidate #1: `std::io::Read`
           candidate #2: `std::iter::Iterator`

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0061, E0599.
For more information about an error, try `rustc --explain E0061`.

------------------------------------------


---- [ui] ui/mismatched_types/issue-36053-2.rs stdout ----
diff of stderr:

5	   |                                                       ^^^^^
6	   |
7	   = note: the method `count` exists but the following trait bounds were not satisfied:
-	           `&mut std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:7:39: 7:53]> : std::iter::Iterator`
9	           `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:7:39: 7:53]> : std::iter::Iterator`
10	
11	error[E0631]: type mismatch in closure arguments


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/issue-36053-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/issue-36053-2.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/issue-36053-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/auxiliary" "-A" "unused"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0599]: no method named `count` found for type `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]>` in the current scope
  --> /checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:55
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                                       ^^^^^
   |
   = note: the method `count` exists but the following trait bounds were not satisfied:
           `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]> : std::iter::Iterator`

error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:32
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                ^^^^^^ -------------- found signature of `for<'r> fn(&'r str) -> _`
   |                                |
   |                                expected signature of `for<'r> fn(&'r &str) -> _`

error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:32
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                ^^^^^^ -------------- found signature of `for<'r> fn(&'r str) -> _`
   |                                |
   |                                expected signature of `fn(&&str) -> _`

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0599`.

------------------------------------------



failures:
    [ui] ui/issues/issue-31173.rs
    [ui] ui/methods/method-call-err-msg.rs
    [ui] ui/mismatched_types/issue-36053-2.rs
