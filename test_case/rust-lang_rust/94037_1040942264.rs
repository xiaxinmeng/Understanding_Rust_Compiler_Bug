plain
..........................................................................ii................i....i.. 7700/12639
ii.................................................................................................. 7800/12639
.................................................................................................... 7900/12639
.................................................................................................... 8000/12639
..........................................F...............F......................................... 8100/12639
.......................................................................i.i..ii...................... 8300/12639
............................................................................................i....... 8400/12639
................................i..................................................................i 8500/12639
.................................................................................................... 8600/12639
---
diff of stderr:

20   --> $DIR/substs-ppaux.rs:25:17
21    |
22 LL |     fn bar<'a, T>() where T: 'a {}
-    |     --------------------------- fn() {<i8 as Foo<ReStatic, ReStatic, u32>>::bar::<ReStatic, char>} defined here
+    |     --------------------------- fn() {<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>} defined here
24 ...
25 LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
26    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
28    |            expected due to this
29    |
30    = note: expected unit type `()`
30    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<ReStatic, ReStatic, u32>>::bar::<ReStatic, char>}`
+                 found fn item `fn() {<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>}`
32 help: use parentheses to call this function
33    |
34 LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>();

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose/substs-ppaux.verbose.stderr
To only update this specific test, also pass `--test-args associated-types/substs-ppaux.rs`


error in revision `verbose`: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/substs-ppaux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "verbose" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:16:17
   |
LL |     fn bar<'a, T>() where T: 'a {}
   |     --------------------------- fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:25:17
   |
   |
LL |     fn bar<'a, T>() where T: 'a {}
   |     --------------------------- fn() {<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `fn() {<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:33:17
   |
   |
LL |     fn baz() {}
   |     -------- fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::baz} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::baz}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:41:17
   |
   |
LL | fn foo<'z>() where &'z (): Sized {
   | -------------------------------- fn() {foo::<ReStatic>} defined here
...
LL |     let x: () = foo::<'static>;
   |            --   ^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `fn() {foo::<ReStatic>}`
help: use parentheses to call this function
   |
LL |     let x: () = foo::<'static>();

error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:49:5
   |
   |
LL |     <str as Foo<u8>>::bar;
   |
   = help: the trait `Sized` is not implemented for `str`
   = help: the trait `Sized` is not implemented for `str`
note: required because of the requirements on the impl of `Foo<'_#0r, '_#1r, u8>` for `str`
   |
   |
LL | impl<'a,'b,T,S> Foo<'a, 'b, S> for T {}

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0277, E0308.
---
---- [ui] ui/nll/ty-outlives/projection-no-regions-closure.rs stdout ----
diff of stderr:

6    |
7    = note: defining type: no_region::<'_#1r, T>::{closure#0} with closure substs [
8                i32,
-                extern "rust-call" fn((std::boxed::Box<T, std::alloc::Global>,)) -> std::boxed::Box<(dyn Anything + '_#2r), std::alloc::Global>,
+                extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn Anything + '_#2r)>,
10                (),
11            ]
12    = note: number of external vids: 3
42    |
42    |
43    = note: defining type: correct_region::<'_#1r, T>::{closure#0} with closure substs [
44                i32,
-                extern "rust-call" fn((std::boxed::Box<T, std::alloc::Global>,)) -> std::boxed::Box<(dyn Anything + '_#2r), std::alloc::Global>,
+                extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn Anything + '_#2r)>,
46                (),
47            ]
48    = note: number of external vids: 3
69    |
69    |
70    = note: defining type: wrong_region::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
71                i32,
-                extern "rust-call" fn((std::boxed::Box<T, std::alloc::Global>,)) -> std::boxed::Box<(dyn Anything + '_#3r), std::alloc::Global>,
+                extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn Anything + '_#3r)>,
73                (),
74            ]
75    = note: number of external vids: 4
105    |
105    |
106    = note: defining type: outlives_region::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
107                i32,
-                extern "rust-call" fn((std::boxed::Box<T, std::alloc::Global>,)) -> std::boxed::Box<(dyn Anything + '_#3r), std::alloc::Global>,
+                extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn Anything + '_#3r)>,
109                (),
110            ]
111    = note: number of external vids: 4

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-no-regions-closure/projection-no-regions-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/projection-no-regions-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-no-regions-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-no-regions-closure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs:25:23
   |
LL |     with_signature(x, |mut y| Box::new(y.next()))
   |
   |
   = note: defining type: no_region::<'_#1r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn Anything + '_#2r)>,
               (),
           ]
   = note: number of external vids: 3
   = note: where <T as std::iter::Iterator>::Item: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs:21:1
   |
   |
LL | / fn no_region<'a, T>(x: Box<T>) -> Box<dyn Anything + 'a>
LL | | where
LL | |     T: Iterator,
LL | | {
LL | |     with_signature(x, |mut y| Box::new(y.next()))
LL | |     //~^ ERROR the associated type `<T as Iterator>::Item` may not live long enough
LL | | }
   |
   |
   = note: defining type: no_region::<'_#1r, T>

error[E0309]: the associated type `<T as Iterator>::Item` may not live long enough
   |
   |
LL |     with_signature(x, |mut y| Box::new(y.next()))
   |
   |
   = help: consider adding an explicit lifetime bound `<T as Iterator>::Item: 'a`...
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs:34:23
   |
   |
LL |     with_signature(x, |mut y| Box::new(y.next()))
   |
   |
   = note: defining type: correct_region::<'_#1r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn Anything + '_#2r)>,
               (),
           ]
   = note: number of external vids: 3
   = note: where <T as std::iter::Iterator>::Item: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs:30:1
   |
   |
LL | / fn correct_region<'a, T>(x: Box<T>) -> Box<dyn Anything + 'a>
LL | | where
LL | |     T: 'a + Iterator,
LL | | {
LL | |     with_signature(x, |mut y| Box::new(y.next()))
LL | | }
   |
   |
   = note: defining type: correct_region::<'_#1r, T>
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs:42:23
   |
   |
LL |     with_signature(x, |mut y| Box::new(y.next()))
   |
   |
   = note: defining type: wrong_region::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn Anything + '_#3r)>,
               (),
           ]
   = note: number of external vids: 4
   = note: where <T as std::iter::Iterator>::Item: '_#3r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs:38:1
   |
   |
LL | / fn wrong_region<'a, 'b, T>(x: Box<T>) -> Box<dyn Anything + 'a>
LL | | where
LL | |     T: 'b + Iterator,
LL | | {
LL | |     with_signature(x, |mut y| Box::new(y.next()))
LL | |     //~^ ERROR the associated type `<T as Iterator>::Item` may not live long enough
LL | | }
   |
   |
   = note: defining type: wrong_region::<'_#1r, '_#2r, T>

error[E0309]: the associated type `<T as Iterator>::Item` may not live long enough
   |
   |
LL |     with_signature(x, |mut y| Box::new(y.next()))
   |
   |
   = help: consider adding an explicit lifetime bound `<T as Iterator>::Item: 'a`...
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs:52:23
   |
   |
LL |     with_signature(x, |mut y| Box::new(y.next()))
   |
   |
   = note: defining type: outlives_region::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn Anything + '_#3r)>,
               (),
           ]
   = note: number of external vids: 4
   = note: where <T as std::iter::Iterator>::Item: '_#3r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs:47:1
   |
   |
LL | / fn outlives_region<'a, 'b, T>(x: Box<T>) -> Box<dyn Anything + 'a>
LL | | where
LL | |     T: 'b + Iterator,
LL | |     'b: 'a,
LL | | {
LL | |     with_signature(x, |mut y| Box::new(y.next()))
LL | | }
   |
   |
   = note: defining type: outlives_region::<'_#1r, '_#2r, T>
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0309`.


------------------------------------------


---- [ui] ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs stdout ----
diff of stderr:

6    |
7    = note: defining type: no_region::<'_#1r, T>::{closure#0} with closure substs [
8                i32,
-                extern "rust-call" fn((std::boxed::Box<T, std::alloc::Global>,)) -> std::boxed::Box<(dyn std::fmt::Debug + '_#2r), std::alloc::Global>,
+                extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn std::fmt::Debug + '_#2r)>,
10                (),
11            ]
12    = note: number of external vids: 3

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type/ty-param-closure-outlives-from-return-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs:26:23
   |
LL |     with_signature(x, |y| y)
   |
   |
   = note: defining type: no_region::<'_#1r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn std::fmt::Debug + '_#2r)>,
               (),
           ]
   = note: number of external vids: 3
   = note: where T: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs:15:1
   |
   |
LL | / fn no_region<'a, T>(x: Box<T>) -> Box<dyn Debug + 'a>
LL | | where
LL | |     T: Debug,
LL | | {
...  |
LL | |     //~^ ERROR the parameter type `T` may not live long enough
LL | | }
   |
   |
   = note: defining type: no_region::<'_#1r, T>
error[E0309]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs:26:23
   |
   |
LL |     with_signature(x, |y| y)
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'a`...
error[E0309]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs:41:5
   |
LL |     x
LL |     x
   |     ^
   |
   = help: consider adding an explicit lifetime bound `T: 'a`...
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0309`.

