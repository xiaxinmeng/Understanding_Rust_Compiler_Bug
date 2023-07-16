plain
diff of stderr:

26   --> $DIR/impl-trait-missing-lifetime-gated.rs:5:31
27    |
28 LL | fn f(_: impl Iterator<Item = &'_ ()>) {}
+    |                               ^^ expected named lifetime parameter
30    |
31    = help: add `#![feature(anonymous_lifetime_in_impl_trait)]` to the crate attributes to enable
+ help: consider introducing a named lifetime parameter
+ help: consider introducing a named lifetime parameter
+    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL | fn f<'a>(_: impl Iterator<Item = &&'a  ()>) {}
32 
33 error[E0658]: anonymous lifetimes in `impl Trait` are unstable
34   --> $DIR/impl-trait-missing-lifetime-gated.rs:8:31


35    |
36 LL | fn g(x: impl Iterator<Item = &'_ ()>) -> Option<&'_ ()> { x.next() }
+    |                               ^^ expected named lifetime parameter
38    |
39    = help: add `#![feature(anonymous_lifetime_in_impl_trait)]` to the crate attributes to enable
+ help: consider introducing a named lifetime parameter
+ help: consider introducing a named lifetime parameter
+    |
+ LL | fn g<'a>(x: impl Iterator<Item = &&'a  ()>) -> Option<&'_ ()> { x.next() }
40 
41 error: aborting due to 4 previous errors
42 

---
To only update this specific test, also pass `--test-args suggestions/impl-trait-missing-lifetime-gated.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/impl-trait-missing-lifetime-gated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-trait-missing-lifetime-gated" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-trait-missing-lifetime-gated/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
   |
   |
LL | fn g(x: impl Iterator<Item = &'_ ()>) -> Option<&'_ ()> { x.next() }
   |                                                  ^^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL | fn g(x: impl Iterator<Item = &'_ ()>) -> Option<&'static ()> { x.next() }

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/impl-trait-missing-lifetime-gated.rs:18:56
   |
   |
LL | async fn i(x: impl Iterator<Item = &'_ ()>) -> Option<&'_ ()> { x.next() }
   |                                                        ^^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL | async fn i(x: impl Iterator<Item = &'_ ()>) -> Option<&'static ()> { x.next() }

error[E0658]: anonymous lifetimes in `impl Trait` are unstable
  --> /checkout/src/test/ui/suggestions/impl-trait-missing-lifetime-gated.rs:5:31
   |
   |
LL | fn f(_: impl Iterator<Item = &'_ ()>) {}
   |                               ^^ expected named lifetime parameter
   = help: add `#![feature(anonymous_lifetime_in_impl_trait)]` to the crate attributes to enable
help: consider introducing a named lifetime parameter
   |
   |
LL | fn f<'a>(_: impl Iterator<Item = &&'a  ()>) {}

error[E0658]: anonymous lifetimes in `impl Trait` are unstable
  --> /checkout/src/test/ui/suggestions/impl-trait-missing-lifetime-gated.rs:8:31
   |
   |
LL | fn g(x: impl Iterator<Item = &'_ ()>) -> Option<&'_ ()> { x.next() }
   |                               ^^ expected named lifetime parameter
   = help: add `#![feature(anonymous_lifetime_in_impl_trait)]` to the crate attributes to enable
help: consider introducing a named lifetime parameter
   |
   |
LL | fn g<'a>(x: impl Iterator<Item = &&'a  ()>) -> Option<&'_ ()> { x.next() }

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0106, E0658.
