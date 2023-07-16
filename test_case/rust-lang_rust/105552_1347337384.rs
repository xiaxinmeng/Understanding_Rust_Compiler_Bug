plain
........................................................................................ 1760/13979
...........................................................................i...........i 1848/13979
i....................................................................................... 1936/13979
........................................................................................ 2024/13979
..............FF....F.......................................i........................... 2112/13979
........................................................................................ 2288/13979
........................................................................................ 2376/13979
........................................................................................ 2464/13979
........................................................................................ 2552/13979
---
......................................................................................i. 4048/13979
....F....i...........i.................................................................. 4136/13979
.................................................................................iii.... 4224/13979
........................................................................................ 4312/13979
....F.........F.F.........F.......................i..................................... 4400/13979
........................................................................................ 4576/13979
........................................................................................ 4664/13979
........................................................................................ 4752/13979
...............i...........................................................F............ 4840/13979
...............i...........................................................F............ 4840/13979
......................................................................F................. 4928/13979
........................................................................................ 5016/13979
...............................F........................F.F............................. 5104/13979
........................................................................................ 5192/13979
.........F...........F.................................................................. 5280/13979
....................................................................................i... 5456/13979
....................................................................i................... 5544/13979
....................................................................i................... 5544/13979
....F...................................................................F............... 5632/13979
.........................................................F.............................. 5720/13979
.....................................................................................F.. 5896/13979
.........................................F.............................................. 5984/13979
...........................................................................F............ 6072/13979
........................................................................................ 6160/13979
---
........................................................................................ 8008/13979
.....................F...................................ii............................. 8096/13979
........................................................................................ 8184/13979
........................................................................................ 8272/13979
........................F..................................F........F.............ii.... 8360/13979
........................................................................................ 8536/13979
........................................................................................ 8624/13979
........................................................................................ 8712/13979
........................................................................................ 8712/13979
.................................FF......F.............................................. 8800/13979
...........................ii........................................................... 8976/13979
......................................iiii.............................................. 9064/13979
................................................................................i....... 9152/13979
.................................i...................................................... 9240/13979
---
........................................................................................ 10120/13979
........................................................................................ 10208/13979
.................................................................ii...............i...ii 10296/13979
i....................................................................................... 10384/13979
....................................................................................F... 10472/13979
.........................F.F...........................................................F 10560/13979
..........................................F............................................. 10648/13979
..............................F................................FF....................... 10736/13979
........................................................................................ 10912/13979
........................................................................................ 11000/13979
...................................iiiii...i....i.i..................................... 11088/13979
........................................................................................ 11176/13979
---
..........................................................................i.......i..... 12320/13979
...i.....i.....................i........................................................ 12408/13979
........................................................................................ 12496/13979
........................................................................................ 12584/13979
..F................................................................F.................... 12672/13979
..............................F......................................................... 12760/13979
........................................................................................ 12936/13979
........................................................................................ 13024/13979
..............................................................i......................... 13112/13979
........................................................................................ 13200/13979
---
---- [ui] src/test/ui/argument-suggestions/two-mismatch-notes.rs stdout ----
diff of stderr:

11    |         ^
12    = note: expected fn pointer `fn(i32)`
13                  found fn item `fn(u32) {f}`
+    = note: different `fn` items always have unique types, even if their signatures are the same
14 note: expected `i32`, found `isize`
16    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/argument-suggestions/two-mismatch-notes/two-mismatch-notes.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args argument-suggestions/two-mismatch-notes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/argument-suggestions/two-mismatch-notes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/argument-suggestions/two-mismatch-notes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/argument-suggestions/two-mismatch-notes/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/argument-suggestions/two-mismatch-notes.rs:10:5
   |
   |
LL |     foo(f, w); //~ ERROR arguments to this function are incorrect
   |
note: expected `i32`, found `u32`
  --> /checkout/src/test/ui/argument-suggestions/two-mismatch-notes.rs:10:9
   |
   |
LL |     foo(f, w); //~ ERROR arguments to this function are incorrect
   = note: expected fn pointer `fn(i32)`
   = note: expected fn pointer `fn(i32)`
                 found fn item `fn(u32) {f}`
   = note: different `fn` items always have unique types, even if their signatures are the same
note: expected `i32`, found `isize`
   |
   |
LL |     foo(f, w); //~ ERROR arguments to this function are incorrect
   |            ^
   = note: expected struct `Wrapper<i32>`
              found struct `Wrapper<isize>`
  --> /checkout/src/test/ui/argument-suggestions/two-mismatch-notes.rs:4:4
   |
   |
LL | fn foo(_: fn(i32), _: Wrapper<i32>) {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/associated-types/defaults-specialization.rs stdout ----
diff of stderr:

24    |                  ^^^^^^^^
25    = note: expected fn pointer `fn() -> <A<T> as Tr>::Ty`
26               found fn pointer `fn() -> u8`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn() -> <A<T> as Tr>::Ty`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn() -> <A<T> as Tr>::Ty as fn() -> u8)`
27 
28 error[E0053]: method `make` has an incompatible type for trait

44    |                  ^^^^^^^^
44    |                  ^^^^^^^^
45    = note: expected fn pointer `fn() -> <B<T> as Tr>::Ty`
46               found fn pointer `fn() -> bool`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn() -> <B<T> as Tr>::Ty`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn() -> <B<T> as Tr>::Ty as fn() -> bool)`
48 error[E0308]: mismatched types
49   --> $DIR/defaults-specialization.rs:10:9



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-specialization/defaults-specialization.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/defaults-specialization.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-specialization.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-specialization" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-specialization/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(associated_type_defaults, specialization)]
   |                                      ^^^^^^^^^^^^^^
   |
   |
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = note: `#[warn(incomplete_features)]` on by default

error[E0053]: method `make` has an incompatible type for trait
   |
LL |     fn make() -> u8 { 0 }
   |                  ^^
   |                  |
   |                  |
   |                  expected associated type, found `u8`
   |                  help: change the output type to match the trait: `<A<T> as Tr>::Ty`
note: type in trait
  --> /checkout/src/test/ui/associated-types/defaults-specialization.rs:9:18
   |
LL |     fn make() -> Self::Ty {
LL |     fn make() -> Self::Ty {
   |                  ^^^^^^^^
   = note: expected fn pointer `fn() -> <A<T> as Tr>::Ty`
              found fn pointer `fn() -> u8`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn() -> <A<T> as Tr>::Ty`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn() -> <A<T> as Tr>::Ty as fn() -> u8)`

error[E0053]: method `make` has an incompatible type for trait
   |
LL |     default type Ty = bool;
   |     ----------------------- expected this associated type
LL |
LL |
LL |     fn make() -> bool { true }
   |                  ^^^^
   |                  |
   |                  expected associated type, found `bool`
   |                  help: change the output type to match the trait: `<B<T> as Tr>::Ty`
note: type in trait
  --> /checkout/src/test/ui/associated-types/defaults-specialization.rs:9:18
   |
LL |     fn make() -> Self::Ty {
LL |     fn make() -> Self::Ty {
   |                  ^^^^^^^^
   = note: expected fn pointer `fn() -> <B<T> as Tr>::Ty`
              found fn pointer `fn() -> bool`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn() -> <B<T> as Tr>::Ty`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn() -> <B<T> as Tr>::Ty as fn() -> bool)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/defaults-specialization.rs:10:9
   |
LL |     type Ty = u8;
---
   |
LL |     fn make() -> Self::Ty { 0u8 }
   |                  --------   ^^^ expected associated type, found `u8`
   |                  |
   |                  expected `<A2<T> as Tr>::Ty` because of return type
   |
   = note: expected associated type `<A2<T> as Tr>::Ty`
                         found type `u8`
   = help: consider constraining the associated type `<A2<T> as Tr>::Ty` to `u8` or calling a method that returns `<A2<T> as Tr>::Ty`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/defaults-specialization.rs:44:29
   |
   |
LL |     default type Ty = bool;
   |     ----------------------- expected this associated type
LL |
LL |     fn make() -> Self::Ty { true }
   |                  --------   ^^^^ expected associated type, found `bool`
   |                  |
   |                  expected `<B2<T> as Tr>::Ty` because of return type
   |
   = note: expected associated type `<B2<T> as Tr>::Ty`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/defaults-specialization.rs:87:32
   |
   |
LL |     let _: <B<()> as Tr>::Ty = 0u8;   //~ error: mismatched types
   |            -----------------   ^^^ expected associated type, found `u8`
   |            expected due to this
   |
   |
   = note: expected associated type `<B<()> as Tr>::Ty`
                         found type `u8`
help: a method is available that returns `<B<()> as Tr>::Ty`
   |
LL |     fn make() -> Self::Ty {
   |     ^^^^^^^^^^^^^^^^^^^^^ consider calling `Tr::make`


error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/defaults-specialization.rs:88:32
   |
LL |     let _: <B<()> as Tr>::Ty = true;  //~ error: mismatched types
   |            -----------------   ^^^^ expected associated type, found `bool`
   |            expected due to this
   |
   |
   = note: expected associated type `<B<()> as Tr>::Ty`
                         found type `bool`
help: a method is available that returns `<B<()> as Tr>::Ty`
   |
LL |     fn make() -> Self::Ty {
   |     ^^^^^^^^^^^^^^^^^^^^^ consider calling `Tr::make`


error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/defaults-specialization.rs:89:33
   |
LL |     let _: <B2<()> as Tr>::Ty = 0u8;  //~ error: mismatched types
   |            ------------------   ^^^ expected associated type, found `u8`
   |            expected due to this
   |
   |
   = note: expected associated type `<B2<()> as Tr>::Ty`
                         found type `u8`
help: a method is available that returns `<B2<()> as Tr>::Ty`
   |
LL |     fn make() -> Self::Ty {
   |     ^^^^^^^^^^^^^^^^^^^^^ consider calling `Tr::make`


error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/defaults-specialization.rs:90:33
   |
LL |     let _: <B2<()> as Tr>::Ty = true; //~ error: mismatched types
   |            ------------------   ^^^^ expected associated type, found `bool`
   |            expected due to this
   |
   |
   = note: expected associated type `<B2<()> as Tr>::Ty`
                         found type `bool`
help: a method is available that returns `<B2<()> as Tr>::Ty`
   |
LL |     fn make() -> Self::Ty {
   |     ^^^^^^^^^^^^^^^^^^^^^ consider calling `Tr::make`

---
---- [ui] src/test/ui/async-await/in-trait/async-example-desugared-boxed-in-trait.rs stdout ----
diff of stderr:

11    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
12    = note: expected fn pointer `fn(&i32) -> Pin<Box<dyn Future<Output = i32>>>`
13               found fn pointer `fn(&i32) -> impl Future<Output = i32>`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(&i32) -> Pin<Box<dyn Future<Output = i32>>>`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&i32) -> Pin<Box<dyn Future<Output = i32>>> as fn(&i32) -> impl Future<Output = i32>)`
15 error: aborting due to previous error
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared-boxed-in-trait/async-example-desugared-boxed-in-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/in-trait/async-example-desugared-boxed-in-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/in-trait/async-example-desugared-boxed-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared-boxed-in-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared-boxed-in-trait/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error[E0053]: method `foo` has an incompatible type for trait
   |
LL |     async fn foo(&self) -> i32 {
   |                            ^^^ expected struct `Pin`, found opaque type
   |
   |
note: type in trait
  --> /checkout/src/test/ui/async-await/in-trait/async-example-desugared-boxed-in-trait.rs:11:22
   |
LL |     fn foo(&self) -> Pin<Box<dyn Future<Output = i32> + '_>>;
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected fn pointer `fn(&i32) -> Pin<Box<dyn Future<Output = i32>>>`
              found fn pointer `fn(&i32) -> impl Future<Output = i32>`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&i32) -> Pin<Box<dyn Future<Output = i32>>>`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&i32) -> Pin<Box<dyn Future<Output = i32>>> as fn(&i32) -> impl Future<Output = i32>)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/regions-bound-missing-bound-in-impl.rs stdout ----
diff of stderr:

24    |
25    = note: expected fn pointer `fn(&'a isize, Inv<'c>, Inv<'c>, Inv<'_>)`
26               found fn pointer `fn(&'a isize, Inv<'_>, Inv<'c>, Inv<'_>)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(&'a isize, Inv<'c>, Inv<'c>, Inv<'d>)`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&'a isize, Inv<'c>, Inv<'c>, Inv<'d>) as fn(&'a isize, Inv<'_>, Inv<'c>, Inv<'d>))`
27 note: the lifetime `'c` as defined here...
29    |

43    |
43    |
44    = note: expected fn pointer `fn(&'a isize, Inv<'c>, Inv<'c>, Inv<'_>)`
45               found fn pointer `fn(&'a isize, Inv<'_>, Inv<'c>, Inv<'_>)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(&'a isize, Inv<'c>, Inv<'c>, Inv<'d>)`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&'a isize, Inv<'c>, Inv<'c>, Inv<'d>) as fn(&'a isize, Inv<'_>, Inv<'c>, Inv<'d>))`
46 note: the lifetime `'c` as defined here...
48    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/regions-bound-missing-bound-in-impl/regions-bound-missing-bound-in-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/regions-bound-missing-bound-in-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/regions-bound-missing-bound-in-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/regions-bound-missing-bound-in-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/regions-bound-missing-bound-in-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0195]: lifetime parameters or bounds on method `no_bound` do not match the trait declaration
   |
   |
LL |     fn no_bound<'b>(self, b: Inv<'b>);
   |                ---- lifetimes in impl do not match this method in trait
...
LL |     fn no_bound<'b:'a>(self, b: Inv<'b>) {
   |                ^^^^^^^ lifetimes do not match method in trait

error[E0195]: lifetime parameters or bounds on method `has_bound` do not match the trait declaration
   |
   |
LL |     fn has_bound<'b:'a>(self, b: Inv<'b>);
   |                 ------- lifetimes in impl do not match this method in trait
...
LL |     fn has_bound<'b>(self, b: Inv<'b>) {
   |                 ^^^^ lifetimes do not match method in trait
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/borrowck/regions-bound-missing-bound-in-impl.rs:27:5
   |
   |
LL |     fn wrong_bound1<'b,'c,'d:'a+'c>(self, b: Inv<'b>, c: Inv<'c>, d: Inv<'d>) {
   |
   |
   = note: expected fn pointer `fn(&'a isize, Inv<'c>, Inv<'c>, Inv<'_>)`
              found fn pointer `fn(&'a isize, Inv<'_>, Inv<'c>, Inv<'_>)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&'a isize, Inv<'c>, Inv<'c>, Inv<'d>)`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&'a isize, Inv<'c>, Inv<'c>, Inv<'d>) as fn(&'a isize, Inv<'_>, Inv<'c>, Inv<'d>))`
note: the lifetime `'c` as defined here...
   |
   |
LL |     fn wrong_bound1<'b,'c,'d:'a+'c>(self, b: Inv<'b>, c: Inv<'c>, d: Inv<'d>) {
note: ...does not necessarily outlive the lifetime `'c` as defined here
  --> /checkout/src/test/ui/borrowck/regions-bound-missing-bound-in-impl.rs:27:24
   |
   |
LL |     fn wrong_bound1<'b,'c,'d:'a+'c>(self, b: Inv<'b>, c: Inv<'c>, d: Inv<'d>) {

error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/borrowck/regions-bound-missing-bound-in-impl.rs:27:5
   |
   |
LL |     fn wrong_bound1<'b,'c,'d:'a+'c>(self, b: Inv<'b>, c: Inv<'c>, d: Inv<'d>) {
   |
   |
   = note: expected fn pointer `fn(&'a isize, Inv<'c>, Inv<'c>, Inv<'_>)`
              found fn pointer `fn(&'a isize, Inv<'_>, Inv<'c>, Inv<'_>)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&'a isize, Inv<'c>, Inv<'c>, Inv<'d>)`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&'a isize, Inv<'c>, Inv<'c>, Inv<'d>) as fn(&'a isize, Inv<'_>, Inv<'c>, Inv<'d>))`
note: the lifetime `'c` as defined here...
   |
   |
LL |     fn wrong_bound1<'b,'c,'d:'a+'c>(self, b: Inv<'b>, c: Inv<'c>, d: Inv<'d>) {
note: ...does not necessarily outlive the lifetime `'c` as defined here
  --> /checkout/src/test/ui/borrowck/regions-bound-missing-bound-in-impl.rs:27:24
   |
   |
LL |     fn wrong_bound1<'b,'c,'d:'a+'c>(self, b: Inv<'b>, c: Inv<'c>, d: Inv<'d>) {


error[E0195]: lifetime parameters or bounds on method `wrong_bound2` do not match the trait declaration
   |
   |
LL |     fn wrong_bound2<'b,'c,'d:'a+'b>(self, b: Inv<'b>, c: Inv<'c>, d: Inv<'d>);
   |                    ---------------- lifetimes in impl do not match this method in trait
...
LL |     fn wrong_bound2(self, b: Inv, c: Inv, d: Inv) {
   |                    ^ lifetimes do not match method in trait
error[E0276]: impl has stricter requirements than trait
  --> /checkout/src/test/ui/borrowck/regions-bound-missing-bound-in-impl.rs:49:26
   |
   |
LL |     fn another_bound<'x: 'a>(self, x: Inv<'x>, y: Inv<'t>);
   |     ------------------------------------------------------- definition of `another_bound` from trait
...
LL |     fn another_bound<'x: 't>(self, x: Inv<'x>, y: Inv<'t>) {
   |                          ^^ impl has extra requirement `'x: 't`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0195, E0276, E0308.
For more information about an error, try `rustc --explain E0195`.
For more information about an error, try `rustc --explain E0195`.
------------------------------------------


---- [ui] src/test/ui/c-variadic/variadic-ffi-1.rs stdout ----
diff of stderr:

46    |
47    = note: expected fn pointer `unsafe extern "C" fn(_, _)`
48                  found fn item `unsafe extern "C" fn(_, _, ...) {foo}`
+    = note: different `fn` items always have unique types, even if their signatures are the same
50 error[E0308]: mismatched types
51   --> $DIR/variadic-ffi-1.rs:26:54

57    |
57    |
58    = note: expected fn pointer `extern "C" fn(_, _, ...)`
59                  found fn item `extern "C" fn(_, _) {bar}`
+    = note: different `fn` items always have unique types, even if their signatures are the same
60 
61 error[E0617]: can't pass `f32` to variadic function


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1/variadic-ffi-1.stderr
To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-1.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1/auxiliary" "--target=i686-pc-windows-msvc" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
error[E0045]: C-variadic function must have a compatible calling convention, like `C` or `cdecl`
   |
   |
LL |     fn printf(_: *const u8, ...);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadic function must have a compatible calling convention
error[E0060]: this function takes at least 2 arguments but 0 arguments were supplied
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:22:9
   |
   |
LL |         foo(); //~ ERROR this function takes at least 2 arguments but 0 arguments were supplied
   |         ^^^-- two arguments of type `isize` and `u8` are missing
note: function defined here
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:15:8
   |
   |
LL |     fn foo(f: isize, x: u8, ...);
help: provide the arguments
   |
   |
LL |         foo(/* isize */, /* u8 */); //~ ERROR this function takes at least 2 arguments but 0 arguments were supplied

error[E0060]: this function takes at least 2 arguments but 1 argument was supplied
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:23:9
   |
   |
LL |         foo(1); //~ ERROR this function takes at least 2 arguments but 1 argument was supplied
   |         ^^^--- an argument of type `u8` is missing
note: function defined here
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:15:8
   |
   |
LL |     fn foo(f: isize, x: u8, ...);
help: provide the argument
   |
   |
LL |         foo(1, /* u8 */); //~ ERROR this function takes at least 2 arguments but 1 argument was supplied

error[E0308]: mismatched types
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:25:56
   |
   |
LL |         let x: unsafe extern "C" fn(f: isize, x: u8) = foo; //~ ERROR mismatched types
   |                -------------------------------------   ^^^ expected non-variadic fn, found variadic function
   |                expected due to this
   |
   |
   = note: expected fn pointer `unsafe extern "C" fn(_, _)`
                 found fn item `unsafe extern "C" fn(_, _, ...) {foo}`
   = note: different `fn` items always have unique types, even if their signatures are the same
error[E0308]: mismatched types
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:26:54
   |
   |
LL |         let y: extern "C" fn(f: isize, x: u8, ...) = bar; //~ ERROR mismatched types
   |                -----------------------------------   ^^^ expected variadic fn, found non-variadic function
   |                expected due to this
   |
   = note: expected fn pointer `extern "C" fn(_, _, ...)`
   = note: expected fn pointer `extern "C" fn(_, _, ...)`
                 found fn item `extern "C" fn(_, _) {bar}`
   = note: different `fn` items always have unique types, even if their signatures are the same

error[E0617]: can't pass `f32` to variadic function
   |
   |
LL |         foo(1, 2, 3f32); //~ ERROR can't pass
   |                   ^^^^ help: cast the value to `c_double`: `3f32 as c_double`

error[E0617]: can't pass `bool` to variadic function
   |
   |
LL |         foo(1, 2, true); //~ ERROR can't pass
   |                   ^^^^ help: cast the value to `c_int`: `true as c_int`
error[E0617]: can't pass `i8` to variadic function
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:30:19
   |
   |
LL |         foo(1, 2, 1i8); //~ ERROR can't pass
   |                   ^^^ help: cast the value to `c_int`: `1i8 as c_int`

error[E0617]: can't pass `u8` to variadic function
   |
   |
LL |         foo(1, 2, 1u8); //~ ERROR can't pass
   |                   ^^^ help: cast the value to `c_uint`: `1u8 as c_uint`
error[E0617]: can't pass `i16` to variadic function
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:32:19
   |
   |
LL |         foo(1, 2, 1i16); //~ ERROR can't pass
   |                   ^^^^ help: cast the value to `c_int`: `1i16 as c_int`

error[E0617]: can't pass `u16` to variadic function
   |
   |
LL |         foo(1, 2, 1u16); //~ ERROR can't pass
   |                   ^^^^ help: cast the value to `c_uint`: `1u16 as c_uint`
error: aborting due to 11 previous errors

Some errors have detailed explanations: E0045, E0060, E0308, E0617.
For more information about an error, try `rustc --explain E0045`.
For more information about an error, try `rustc --explain E0045`.
------------------------------------------


---- [ui] src/test/ui/closure-expected-type/expect-fn-supply-fn.rs stdout ----
diff of stderr:

27    |
28    = note: expected fn pointer `fn(&u32)`
29               found fn pointer `for<'a> fn(&'a u32)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(&u32)`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&u32) as for<'a> fn(&'a u32))`
31 error[E0308]: mismatched types
32   --> $DIR/expect-fn-supply-fn.rs:39:50

36    |
36    |
37    = note: expected fn pointer `for<'a> fn(&'a u32)`
38               found fn pointer `fn(&u32)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `for<'a> fn(&'a u32)`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a> fn(&'a u32) as fn(&u32))`
40 error[E0308]: mismatched types
41   --> $DIR/expect-fn-supply-fn.rs:48:50

45    |
45    |
46    = note: expected fn pointer `for<'a> fn(&'a u32)`
47               found fn pointer `fn(&u32)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `for<'a> fn(&'a u32)`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a> fn(&'a u32) as fn(&u32))`
49 error: aborting due to 5 previous errors
50 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-fn-supply-fn/expect-fn-supply-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closure-expected-type/expect-fn-supply-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-fn-supply-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-fn-supply-fn/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn expect_free_supply_free_from_fn<'x>(x: &'x u32) {
   |                                    -- lifetime `'x` defined here
...
LL |     with_closure_expecting_fn_with_free_region(|x: fn(&'x u32), y| {});
   |                                                 |
   |                                                 |
   |                                                 has type `fn(&'1 u32)`
   |                                                 requires that `'1` must outlive `'x`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:16:49
   |
   |
LL | fn expect_free_supply_free_from_fn<'x>(x: &'x u32) {
   |                                    -- lifetime `'x` defined here
...
LL |     with_closure_expecting_fn_with_free_region(|x: fn(&'x u32), y| {});
   |                                                 ^ requires that `'x` must outlive `'static`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:32:49
   |
   |
LL |     with_closure_expecting_fn_with_free_region(|x: fn(&u32), y| {});
   |                                                 ^ one type is more general than the other
   = note: expected fn pointer `fn(&u32)`
   = note: expected fn pointer `fn(&u32)`
              found fn pointer `for<'a> fn(&'a u32)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&u32)`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&u32) as for<'a> fn(&'a u32))`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:39:50
   |
   |
LL |     with_closure_expecting_fn_with_bound_region(|x: fn(&'x u32), y| {});
   |                                                  ^ one type is more general than the other
   |
   = note: expected fn pointer `for<'a> fn(&'a u32)`
              found fn pointer `fn(&u32)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `for<'a> fn(&'a u32)`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a> fn(&'a u32) as fn(&u32))`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:48:50
   |
   |
LL |     with_closure_expecting_fn_with_bound_region(|x: Foo<'_>, y| {
   |                                                  ^ one type is more general than the other
   |
   = note: expected fn pointer `for<'a> fn(&'a u32)`
              found fn pointer `fn(&u32)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `for<'a> fn(&'a u32)`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a> fn(&'a u32) as fn(&u32))`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/compare-method/issue-90444.rs stdout ----
diff of stderr:

9    |
10    = note: expected fn pointer `fn(for<'a> fn((), (), &'a ())) -> A`
11               found fn pointer `fn(for<'a> fn((), (), &'a mut ())) -> A`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(for<'a> fn((), (), &'a ())) -> A`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(for<'a> fn((), (), &'a ())) -> A as fn(for<'a> fn((), (), &'a mut ())) -> A)`
13 error[E0053]: method `from` has an incompatible type for trait
14   --> $DIR/issue-90444.rs:11:16

21    |
21    |
22    = note: expected fn pointer `fn(fn((), (), u32)) -> B`
23               found fn pointer `fn(fn((), (), u64)) -> B`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(fn((), (), u32)) -> B`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(fn((), (), u32)) -> B as fn(fn((), (), u64)) -> B)`
25 error: aborting due to 2 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/issue-90444/issue-90444.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args compare-method/issue-90444.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/compare-method/issue-90444.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/issue-90444" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/issue-90444/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0053]: method `from` has an incompatible type for trait
   |
   |
LL |     fn from(_: fn((), (), &mut ())) -> Self {
   |                |
   |                types differ in mutability
   |                types differ in mutability
   |                help: change the parameter type to match the trait: `for<'a> fn((), (), &'a ())`
   |
   = note: expected fn pointer `fn(for<'a> fn((), (), &'a ())) -> A`
              found fn pointer `fn(for<'a> fn((), (), &'a mut ())) -> A`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(for<'a> fn((), (), &'a ())) -> A`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(for<'a> fn((), (), &'a ())) -> A as fn(for<'a> fn((), (), &'a mut ())) -> A)`
error[E0053]: method `from` has an incompatible type for trait
  --> /checkout/src/test/ui/compare-method/issue-90444.rs:11:16
   |
   |
LL |     fn from(_: fn((), (), u64)) -> Self {
   |                |
   |                expected `u32`, found `u64`
   |                expected `u32`, found `u64`
   |                help: change the parameter type to match the trait: `fn((), (), u32)`
   |
   = note: expected fn pointer `fn(fn((), (), u32)) -> B`
              found fn pointer `fn(fn((), (), u64)) -> B`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(fn((), (), u32)) -> B`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(fn((), (), u32)) -> B as fn(fn((), (), u64)) -> B)`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/compare-method/bad-self-type.rs stdout ----
diff of stderr:

9    |
10    = note: expected fn pointer `fn(Pin<&mut MyFuture>, &mut Context<'_>) -> Poll<_>`
11               found fn pointer `fn(MyFuture, &mut Context<'_>) -> Poll<_>`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(Pin<&mut MyFuture>, &mut Context<'_>) -> Poll<()>`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(Pin<&mut MyFuture>, &mut Context<'_>) -> Poll<()> as fn(MyFuture, &mut Context<'_>) -> Poll<()>)`
13 error[E0053]: method `foo` has an incompatible type for trait
14   --> $DIR/bad-self-type.rs:22:18

26    |            ^^^^
26    |            ^^^^
27    = note: expected fn pointer `fn(MyFuture)`
28               found fn pointer `fn(Box<MyFuture>)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(MyFuture)`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(MyFuture) as fn(Box<MyFuture>))`
---
---- [ui] src/test/ui/compare-method/reordered-type-param.rs stdout ----
diff of stderr:

16    |                             ^
17    = note: expected fn pointer `fn(&E, F) -> F`
18               found fn pointer `fn(&E, G) -> G`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(&E, F) -> F`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&E, F) -> F as fn(&E, G) -> G)`
19    = note: a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
20    = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/reordered-type-param/reordered-type-param.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/reordered-type-param/reordered-type-param.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args compare-method/reordered-type-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/compare-method/reordered-type-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/reordered-type-param" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/reordered-type-param/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0053]: method `b` has an incompatible type for trait
   |
   |
LL |   fn b<F:Clone,G>(&self, _x: G) -> G { panic!() } //~ ERROR method `b` has an incompatible type
   |        |       |             |
   |        |       |             expected type parameter `F`, found type parameter `G`
   |        |       |             help: change the parameter type to match the trait: `F`
   |        |       found type parameter
   |        |       found type parameter
   |        expected type parameter
   |
note: type in trait
  --> /checkout/src/test/ui/compare-method/reordered-type-param.rs:7:29
   |
LL |   fn b<C:Clone,D>(&self, x: C) -> C;
   |                             ^
   = note: expected fn pointer `fn(&E, F) -> F`
              found fn pointer `fn(&E, G) -> G`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&E, F) -> F`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&E, F) -> F as fn(&E, G) -> G)`
   = note: a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
error: aborting due to previous error

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/error-codes/E0308.rs stdout ----
diff of stderr:

6    |
7    = note: expected fn pointer `extern "rust-intrinsic" fn()`
8               found fn pointer `extern "rust-intrinsic" fn() -> usize`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `extern "rust-intrinsic" fn()`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(extern "rust-intrinsic" fn() as extern "rust-intrinsic" fn() -> usize)`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0308/E0308.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0308.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0308.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0308" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0308/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0308]: intrinsic has wrong type
   |
   |
LL |     fn size_of<T>(); //~ ERROR E0308
   |
   |
   = note: expected fn pointer `extern "rust-intrinsic" fn()`
              found fn pointer `extern "rust-intrinsic" fn() -> usize`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `extern "rust-intrinsic" fn()`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(extern "rust-intrinsic" fn() as extern "rust-intrinsic" fn() -> usize)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/extern/extern-main-fn.rs stdout ----
diff of stderr:

6    |
7    = note: expected fn pointer `fn()`
8               found fn pointer `extern "C" fn()`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn()`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn() as extern "C" fn())`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-main-fn/extern-main-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args extern/extern-main-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/extern-main-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-main-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-main-fn/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0580]: `main` function has wrong type
   |
   |
LL | extern "C" fn main() {} //~ ERROR: `main` function has wrong type [E0580]
   | ^^^^^^^^^^^^^^^^^^^^ expected "Rust" fn, found "C" fn
   = note: expected fn pointer `fn()`
              found fn pointer `extern "C" fn()`
              found fn pointer `extern "C" fn()`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn()`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn() as extern "C" fn())`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0580`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/fn/bad-main.rs stdout ----
diff of stderr:

6    |
7    = note: expected fn pointer `fn()`
8               found fn pointer `fn(isize)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn()`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn() as fn(isize))`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/bad-main/bad-main.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fn/bad-main.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/bad-main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/bad-main" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/bad-main/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0580]: `main` function has wrong type
   |
   |
LL | fn main(x: isize) { } //~ ERROR: `main` function has wrong type [E0580]
   |
   = note: expected fn pointer `fn()`
   = note: expected fn pointer `fn()`
              found fn pointer `fn(isize)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn()`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn() as fn(isize))`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0580`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/fn/fn-compare-mismatch.rs stdout ----
diff of stderr:

19    |
20    = note: expected fn item `fn() {f}`
21               found fn item `fn() {g}`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: if the expected type is due to type inference, consider declaring the type as `fn()` and casting the expected `fn` to a function pointer: `(f as fn())`
23 error: aborting due to 2 previous errors
24 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-compare-mismatch/fn-compare-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fn/fn-compare-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/fn-compare-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-compare-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-compare-mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `fn() {f}`
   |
   |
LL |     let x = f == g;
   |             - ^^ - fn() {g}
   |             fn() {f}
   |
help: use parentheses to call these
   |
   |
LL |     let x = f() == g();
   |              ++     ++
error[E0308]: mismatched types
  --> /checkout/src/test/ui/fn/fn-compare-mismatch.rs:4:18
   |
   |
LL |     let x = f == g;
   |                  ^ expected fn item, found a different fn item
   |
   = note: expected fn item `fn() {f}`
              found fn item `fn() {g}`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: if the expected type is due to type inference, consider declaring the type as `fn()` and casting the expected `fn` to a function pointer: `(f as fn())`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0369.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/fn/fn-item-type.rs stdout ----
diff of stderr:

9    = note: expected fn item `fn(_) -> _ {foo::<u8>}`
10               found fn item `fn(_) -> _ {bar::<u8>}`
11    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: if the expected type is due to type inference, consider declaring the type as `fn(isize) -> isize` and casting the expected `fn` to a function pointer: `(foo::<u8> as fn(isize) -> isize)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
12    = help: change the expected type to be function pointer `fn(isize) -> isize`
13    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `foo::<u8> as fn(isize) -> isize`


28    = note: expected fn item `fn(_) -> _ {foo::<u8>}`
29               found fn item `fn(_) -> _ {foo::<i8>}`
30    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: if the expected type is due to type inference, consider declaring the type as `fn(isize) -> isize` and casting the expected `fn` to a function pointer: `(foo::<u8> as fn(isize) -> isize)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
31    = help: change the expected type to be function pointer `fn(isize) -> isize`
32    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `foo::<u8> as fn(isize) -> isize`


47    = note: expected fn item `fn(_) -> _ {bar::<String>}`
48               found fn item `fn(_) -> _ {bar::<Vec<u8>>}`
49    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: if the expected type is due to type inference, consider declaring the type as `fn(isize) -> isize` and casting the expected `fn` to a function pointer: `(bar::<String> as fn(isize) -> isize)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
50    = help: change the expected type to be function pointer `fn(isize) -> isize`
51    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `bar::<String> as fn(isize) -> isize`


66    = note: expected fn item `fn() {<u8 as Foo>::foo}`
67               found fn item `fn() {<u16 as Foo>::foo}`
68    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: if the expected type is due to type inference, consider declaring the type as `fn()` and casting the expected `fn` to a function pointer: `(<u8 as Foo>::foo as fn())`
+    = note: different `fn` items always have unique types, even if their signatures are the same
69    = help: change the expected type to be function pointer `fn()`
70    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `<u8 as Foo>::foo as fn()`

84    |
84    |
85    = note: expected fn item `fn(_) -> _ {foo::<u8>}`
86            found fn pointer `fn(_) -> _`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be a function pointer reference `&fn(isize) -> isize`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(foo::<u8> as fn(isize) -> isize)`
87    = help: change the expected type to be function pointer `fn(isize) -> isize`
88    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `foo::<u8> as fn(isize) -> isize`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-item-type/fn-item-type.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-item-type/fn-item-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fn/fn-item-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/fn-item-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-item-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-item-type/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/fn/fn-item-type.rs:13:19
   |
   |
LL |     eq(foo::<u8>, bar::<u8>);
   |     --            ^^^^^^^^^ expected fn item, found a different fn item
   |     arguments to this function are incorrect
   |
   |
   = note: expected fn item `fn(_) -> _ {foo::<u8>}`
              found fn item `fn(_) -> _ {bar::<u8>}`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: if the expected type is due to type inference, consider declaring the type as `fn(isize) -> isize` and casting the expected `fn` to a function pointer: `(foo::<u8> as fn(isize) -> isize)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer `fn(isize) -> isize`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `foo::<u8> as fn(isize) -> isize`
  --> /checkout/src/test/ui/fn/fn-item-type.rs:7:4
   |
   |
LL | fn eq<T>(x: T, y: T) { }
   |    ^^          ----
error[E0308]: mismatched types
  --> /checkout/src/test/ui/fn/fn-item-type.rs:22:19
   |
   |
LL |     eq(foo::<u8>, foo::<i8>);
   |     --            ^^^^^^^^^ expected `u8`, found `i8`
   |     arguments to this function are incorrect
   |
   |
   = note: expected fn item `fn(_) -> _ {foo::<u8>}`
              found fn item `fn(_) -> _ {foo::<i8>}`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: if the expected type is due to type inference, consider declaring the type as `fn(isize) -> isize` and casting the expected `fn` to a function pointer: `(foo::<u8> as fn(isize) -> isize)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer `fn(isize) -> isize`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `foo::<u8> as fn(isize) -> isize`
  --> /checkout/src/test/ui/fn/fn-item-type.rs:7:4
   |
   |
LL | fn eq<T>(x: T, y: T) { }
   |    ^^          ----
error[E0308]: mismatched types
  --> /checkout/src/test/ui/fn/fn-item-type.rs:29:23
   |
   |
LL |     eq(bar::<String>, bar::<Vec<u8>>);
   |     --                ^^^^^^^^^^^^^^ expected struct `String`, found struct `Vec`
   |     arguments to this function are incorrect
   |
   |
   = note: expected fn item `fn(_) -> _ {bar::<String>}`
              found fn item `fn(_) -> _ {bar::<Vec<u8>>}`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: if the expected type is due to type inference, consider declaring the type as `fn(isize) -> isize` and casting the expected `fn` to a function pointer: `(bar::<String> as fn(isize) -> isize)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer `fn(isize) -> isize`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `bar::<String> as fn(isize) -> isize`
  --> /checkout/src/test/ui/fn/fn-item-type.rs:7:4
   |
   |
LL | fn eq<T>(x: T, y: T) { }
   |    ^^          ----
error[E0308]: mismatched types
  --> /checkout/src/test/ui/fn/fn-item-type.rs:38:26
   |
   |
LL |     eq(<u8 as Foo>::foo, <u16 as Foo>::foo);
   |     --                   ^^^^^^^^^^^^^^^^^ expected `u8`, found `u16`
   |     arguments to this function are incorrect
   |
   |
   = note: expected fn item `fn() {<u8 as Foo>::foo}`
              found fn item `fn() {<u16 as Foo>::foo}`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: if the expected type is due to type inference, consider declaring the type as `fn()` and casting the expected `fn` to a function pointer: `(<u8 as Foo>::foo as fn())`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer `fn()`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `<u8 as Foo>::foo as fn()`
  --> /checkout/src/test/ui/fn/fn-item-type.rs:7:4
   |
   |
LL | fn eq<T>(x: T, y: T) { }
   |    ^^          ----
error[E0308]: mismatched types
  --> /checkout/src/test/ui/fn/fn-item-type.rs:45:19
   |
   |
LL |     eq(foo::<u8>, bar::<u8> as fn(isize) -> isize);
   |     --            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn item, found fn pointer
   |     arguments to this function are incorrect
   |
   |
   = note: expected fn item `fn(_) -> _ {foo::<u8>}`
           found fn pointer `fn(_) -> _`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be a function pointer reference `&fn(isize) -> isize`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(foo::<u8> as fn(isize) -> isize)`
   = help: change the expected type to be function pointer `fn(isize) -> isize`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `foo::<u8> as fn(isize) -> isize`
  --> /checkout/src/test/ui/fn/fn-item-type.rs:7:4
   |
   |
LL | fn eq<T>(x: T, y: T) { }
   |    ^^          ----
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/fn/signature-error-reporting-under-verbose.rs stdout ----
diff of stderr:

8    |
9    = note: expected fn pointer `fn(i32, u32)`
10                  found fn item `fn(i32, i32) {foo}`
+    = note: different `fn` items always have unique types, even if their signatures are the same
12   --> $DIR/signature-error-reporting-under-verbose.rs:5:4
13    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/signature-error-reporting-under-verbose/signature-error-reporting-under-verbose.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fn/signature-error-reporting-under-verbose.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/signature-error-reporting-under-verbose.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/signature-error-reporting-under-verbose" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/signature-error-reporting-under-verbose/auxiliary" "-Zverbose"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/fn/signature-error-reporting-under-verbose.rs:10:15
   |
   |
LL |     needs_ptr(foo);
   |     --------- ^^^ expected `u32`, found `i32`
   |     arguments to this function are incorrect
   |
   = note: expected fn pointer `fn(i32, u32)`
   = note: expected fn pointer `fn(i32, u32)`
                 found fn item `fn(i32, i32) {foo}`
   = note: different `fn` items always have unique types, even if their signatures are the same
  --> /checkout/src/test/ui/fn/signature-error-reporting-under-verbose.rs:5:4
   |
   |
LL | fn needs_ptr(_: fn(i32, u32)) {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/higher-rank-trait-bounds/hrtb-exists-forall-fn.rs stdout ----
diff of stderr:

6    |
7    = note: expected fn pointer `for<'b> fn(&'b u32)`
8               found fn pointer `fn(&u32)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `for<'b> fn(&'b u32)`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'b> fn(&'b u32) as fn(&u32))`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/hrtb-exists-forall-fn/hrtb-exists-forall-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args higher-rank-trait-bounds/hrtb-exists-forall-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/hrtb-exists-forall-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/hrtb-exists-forall-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/hrtb-exists-forall-fn/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/higher-rank-trait-bounds/hrtb-exists-forall-fn.rs:17:34
   |
   |
LL |     let _: for<'b> fn(&'b u32) = foo(); //~ ERROR mismatched types
   |                                  ^^^^^ one type is more general than the other
   |
   = note: expected fn pointer `for<'b> fn(&'b u32)`
              found fn pointer `fn(&u32)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `for<'b> fn(&'b u32)`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'b> fn(&'b u32) as fn(&u32))`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/hr-subtype/placeholder-pattern-fail.rs stdout ----
diff of stderr:

6    |
7    = note: expected fn pointer `for<'a, 'b> fn(Inv<'a>, Inv<'b>)`
8               found fn pointer `for<'a> fn(Inv<'a>, Inv<'a>)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `for<'a, 'b> fn(Inv<'a>, Inv<'b>)`
---
  --> /checkout/src/test/ui/impl-trait/in-trait/specialization-broken.rs:9:22
   |
LL |     fn bar(&self) -> impl Sized;
   |                      ^^^^^^^^^^
   = note: expected fn pointer `fn(&U) -> impl Sized`
              found fn pointer `fn(&U) -> U`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&U) -> impl Sized`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&U) -> impl Sized as fn(&U) -> U)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle.rs stdout ----
diff of stderr:

20    |
21    = note: expected fn pointer `fn(&a::Bar, &(a::Bar, i32)) -> _`
22               found fn pointer `fn(&a::Bar, &(a::Foo, i32)) -> _`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(&a::Bar, &(a::Bar, i32)) -> bool`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&a::Bar, &(a::Bar, i32)) -> bool as fn(&a::Bar, &(a::Foo, i32)) -> bool)`
24 error: unconstrained opaque type
25   --> $DIR/recursive-type-alias-impl-trait-declaration-too-subtle.rs:18:16

43    |
43    |
44    = note: expected fn pointer `fn(&b::Bar, &(b::Foo, i32)) -> _`
45               found fn pointer `fn(&b::Bar, &(b::Bar, i32)) -> _`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(&b::Bar, &(b::Foo, i32)) -> bool`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&b::Bar, &(b::Foo, i32)) -> bool as fn(&b::Bar, &(b::Bar, i32)) -> bool)`
47 error: aborting due to 4 previous errors
48 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle/recursive-type-alias-impl-trait-declaration-too-subtle.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle.rs:4:16
   |
   |
LL |     type Foo = impl PartialEq<(Foo, i32)>;
   |
   |
   = note: `Foo` must be used in combination with a concrete type within the same module

error[E0053]: method `eq` has an incompatible type for trait
   |
   |
LL |     type Foo = impl PartialEq<(Foo, i32)>;
...
...
LL |         fn eq(&self, _other: &(Foo, i32)) -> bool {
   |                              |
   |                              expected struct `Bar`, found opaque type
   |                              expected struct `Bar`, found opaque type
   |                              help: change the parameter type to match the trait: `&(a::Bar, i32)`
   |
   = note: expected fn pointer `fn(&a::Bar, &(a::Bar, i32)) -> _`
              found fn pointer `fn(&a::Bar, &(a::Foo, i32)) -> _`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&a::Bar, &(a::Bar, i32)) -> bool`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&a::Bar, &(a::Bar, i32)) -> bool as fn(&a::Bar, &(a::Foo, i32)) -> bool)`
error: unconstrained opaque type
  --> /checkout/src/test/ui/impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle.rs:18:16
   |
   |
LL |     type Foo = impl PartialEq<(Foo, i32)>;
   |
   |
   = note: `Foo` must be used in combination with a concrete type within the same module

error[E0053]: method `eq` has an incompatible type for trait
   |
   |
LL |     type Foo = impl PartialEq<(Foo, i32)>;
...
...
LL |         fn eq(&self, _other: &(Bar, i32)) -> bool {
   |                              |
   |                              expected opaque type, found struct `Bar`
   |                              expected opaque type, found struct `Bar`
   |                              help: change the parameter type to match the trait: `&(b::Foo, i32)`
   |
   = note: expected fn pointer `fn(&b::Bar, &(b::Foo, i32)) -> _`
              found fn pointer `fn(&b::Bar, &(b::Bar, i32)) -> _`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&b::Bar, &(b::Foo, i32)) -> bool`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&b::Bar, &(b::Foo, i32)) -> bool as fn(&b::Bar, &(b::Bar, i32)) -> bool)`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/impl-trait/trait_type.rs stdout ----
diff of stderr:

9    |
10    = note: expected fn pointer `fn(&MyType, &mut Formatter<'_>) -> Result<(), std::fmt::Error>`
11               found fn pointer `fn(&MyType, &str)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(&MyType, &mut Formatter<'_>) -> Result<(), std::fmt::Error>`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&MyType, &mut Formatter<'_>) -> Result<(), std::fmt::Error> as fn(&MyType, &str))`
12 
13 error[E0050]: method `fmt` has 1 parameter but the declaration in trait `std::fmt::Display::fmt` has 2


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/trait_type/trait_type.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/trait_type/trait_type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/trait_type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/trait_type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/trait_type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/trait_type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0053]: method `fmt` has an incompatible type for trait
   |
   |
LL |    fn fmt(&self, x: &str) -> () { }
   |                     |
   |                     types differ in mutability
   |                     help: change the parameter type to match the trait: `&mut Formatter<'_>`
   |
   |
   = note: expected fn pointer `fn(&MyType, &mut Formatter<'_>) -> Result<(), std::fmt::Error>`
              found fn pointer `fn(&MyType, &str)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&MyType, &mut Formatter<'_>) -> Result<(), std::fmt::Error>`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&MyType, &mut Formatter<'_>) -> Result<(), std::fmt::Error> as fn(&MyType, &str))`

error[E0050]: method `fmt` has 1 parameter but the declaration in trait `std::fmt::Display::fmt` has 2
   |
LL |    fn fmt(&self) -> () { }
   |           ^^^^^ expected 2 parameters, found 1
   |
   |
   = note: `fmt` from trait: `fn(&Self, &mut Formatter<'_>) -> Result<(), std::fmt::Error>`

error[E0186]: method `fmt` has a `&self` declaration in the trait, but not in the impl
   |
   |
LL |    fn fmt() -> () { }
   |    ^^^^^^^^^^^^^^ expected `&self` in impl
   |
   = note: `fmt` from trait: `fn(&Self, &mut Formatter<'_>) -> Result<(), std::fmt::Error>`
error[E0046]: not all trait items implemented, missing: `fmt`
  --> /checkout/src/test/ui/impl-trait/trait_type.rs:21:1
   |
   |
LL | impl std::fmt::Display for MyType4 {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `fmt` in implementation
   |
   = help: implement the missing item: `fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0046, E0050, E0053, E0186.
For more information about an error, try `rustc --explain E0046`.
For more information about an error, try `rustc --explain E0046`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-10764.rs stdout ----
diff of stderr:

8    |
9    = note: expected fn pointer `fn()`
10                  found fn item `extern "C" fn() {bar}`
+    = note: different `fn` items always have unique types, even if their signatures are the same
12   --> $DIR/issue-10764.rs:1:4
13    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10764/issue-10764.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-10764.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10764.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10764" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10764/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-10764.rs:4:15
   |
   |
LL | fn main() { f(bar) }
   |             - ^^^ expected "Rust" fn, found "C" fn
   |             arguments to this function are incorrect
   |
   = note: expected fn pointer `fn()`
   = note: expected fn pointer `fn()`
                 found fn item `extern "C" fn() {bar}`
   = note: different `fn` items always have unique types, even if their signatures are the same
  --> /checkout/src/test/ui/issues/issue-10764.rs:1:4
   |
   |
LL | fn f(_: extern "Rust" fn()) {}
   |    ^ ---------------------
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-13033.rs stdout ----
diff of stderr:

14    |                              ^^^^^^^^^^^^
15    = note: expected fn pointer `fn(&mut Baz, &mut dyn Foo)`
16               found fn pointer `fn(&mut Baz, &dyn Foo)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(&mut Baz, &mut dyn Foo)`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&mut Baz, &mut dyn Foo) as fn(&mut Baz, &dyn Foo))`
18 error: aborting due to previous error
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13033/issue-13033.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-13033.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-13033.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13033" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13033/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0053]: method `bar` has an incompatible type for trait
   |
   |
LL |     fn bar(&mut self, other: &dyn Foo) {}
   |                              |
   |                              types differ in mutability
   |                              help: change the parameter type to match the trait: `&mut dyn Foo`
   |
   |
note: type in trait
  --> /checkout/src/test/ui/issues/issue-13033.rs:2:30
   |
LL |     fn bar(&mut self, other: &mut dyn Foo);
   |                              ^^^^^^^^^^^^
   = note: expected fn pointer `fn(&mut Baz, &mut dyn Foo)`
              found fn pointer `fn(&mut Baz, &dyn Foo)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&mut Baz, &mut dyn Foo)`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&mut Baz, &mut dyn Foo) as fn(&mut Baz, &dyn Foo))`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-15094.rs stdout ----
diff of stderr:

6    |
7    = note: expected fn pointer `extern "rust-call" fn(Debuger<_>, ())`
8               found fn pointer `fn(Debuger<_>, ())`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `extern "rust-call" fn(Debuger<T>, ())`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(extern "rust-call" fn(Debuger<T>, ()) as fn(Debuger<T>, ()))`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15094/issue-15094.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-15094.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-15094.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15094" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15094/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0053]: method `call_once` has an incompatible type for trait
   |
LL |     fn call_once(self, _args: ()) {
LL |     fn call_once(self, _args: ()) {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected "rust-call" fn, found "Rust" fn
   |
   = note: expected fn pointer `extern "rust-call" fn(Debuger<_>, ())`
              found fn pointer `fn(Debuger<_>, ())`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `extern "rust-call" fn(Debuger<T>, ())`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(extern "rust-call" fn(Debuger<T>, ()) as fn(Debuger<T>, ()))`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-20225.rs stdout ----
diff of stderr:

11    |
12    = note: expected fn pointer `extern "rust-call" fn(&Foo, (&'a T,))`
13               found fn pointer `extern "rust-call" fn(&Foo, (T,))`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `extern "rust-call" fn(&Foo, (&'a T,))`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(extern "rust-call" fn(&Foo, (&'a T,)) as extern "rust-call" fn(&Foo, (T,)))`
15 error[E0053]: method `call_mut` has an incompatible type for trait
16   --> $DIR/issue-20225.rs:11:51

25    |
25    |
26    = note: expected fn pointer `extern "rust-call" fn(&mut Foo, (&'a T,))`
27               found fn pointer `extern "rust-call" fn(&mut Foo, (T,))`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `extern "rust-call" fn(&mut Foo, (&'a T,))`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(extern "rust-call" fn(&mut Foo, (&'a T,)) as extern "rust-call" fn(&mut Foo, (T,)))`
29 error[E0053]: method `call_once` has an incompatible type for trait
30   --> $DIR/issue-20225.rs:18:47

40    |
40    |
41    = note: expected fn pointer `extern "rust-call" fn(Foo, (&'a T,))`
42               found fn pointer `extern "rust-call" fn(Foo, (T,))`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `extern "rust-call" fn(Foo, (&'a T,))`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(extern "rust-call" fn(Foo, (&'a T,)) as extern "rust-call" fn(Foo, (T,)))`
44 error: aborting due to 3 previous errors
45 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20225/issue-20225.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-20225.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20225.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20225" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20225/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0053]: method `call` has an incompatible type for trait
   |
   |
LL | impl<'a, T> Fn<(&'a T,)> for Foo {
   |          - this type parameter
LL |   extern "rust-call" fn call(&self, (_,): (T,)) {}
   |                                           |
   |                                           |
   |                                           expected `&T`, found type parameter `T`
   |                                           help: change the parameter type to match the trait: `(&'a T,)`
   |
   = note: expected fn pointer `extern "rust-call" fn(&Foo, (&'a T,))`
              found fn pointer `extern "rust-call" fn(&Foo, (T,))`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `extern "rust-call" fn(&Foo, (&'a T,))`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(extern "rust-call" fn(&Foo, (&'a T,)) as extern "rust-call" fn(&Foo, (T,)))`
error[E0053]: method `call_mut` has an incompatible type for trait
  --> /checkout/src/test/ui/issues/issue-20225.rs:11:51
   |
   |
LL | impl<'a, T> FnMut<(&'a T,)> for Foo {
   |          - this type parameter
LL |   extern "rust-call" fn call_mut(&mut self, (_,): (T,)) {}
   |                                                   |
   |                                                   |
   |                                                   expected `&T`, found type parameter `T`
   |                                                   help: change the parameter type to match the trait: `(&'a T,)`
   |
   = note: expected fn pointer `extern "rust-call" fn(&mut Foo, (&'a T,))`
              found fn pointer `extern "rust-call" fn(&mut Foo, (T,))`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `extern "rust-call" fn(&mut Foo, (&'a T,))`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(extern "rust-call" fn(&mut Foo, (&'a T,)) as extern "rust-call" fn(&mut Foo, (T,)))`
error[E0053]: method `call_once` has an incompatible type for trait
  --> /checkout/src/test/ui/issues/issue-20225.rs:18:47
   |
   |
LL | impl<'a, T> FnOnce<(&'a T,)> for Foo {
...
...
LL |   extern "rust-call" fn call_once(self, (_,): (T,)) {}
   |                                               |
   |                                               |
   |                                               expected `&T`, found type parameter `T`
   |                                               help: change the parameter type to match the trait: `(&'a T,)`
   |
   = note: expected fn pointer `extern "rust-call" fn(Foo, (&'a T,))`
              found fn pointer `extern "rust-call" fn(Foo, (T,))`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `extern "rust-call" fn(Foo, (&'a T,))`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(extern "rust-call" fn(Foo, (&'a T,)) as extern "rust-call" fn(Foo, (T,)))`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-21332.rs stdout ----
diff of stderr:

9    |
10    = note: expected fn pointer `fn(&mut S) -> Option<i32>`
11               found fn pointer `fn(&mut S) -> Result<i32, i32>`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(&mut S) -> Option<i32>`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&mut S) -> Option<i32> as fn(&mut S) -> Result<i32, i32>)`
13 error: aborting due to previous error
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21332/issue-21332.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-21332.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21332.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21332" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21332/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0053]: method `next` has an incompatible type for trait
   |
   |
LL |     fn next(&mut self) -> Result<i32, i32> { Ok(7) }
   |                           |
   |                           expected enum `Option`, found enum `Result`
   |                           help: change the output type to match the trait: `Option<i32>`
   |
   |
   = note: expected fn pointer `fn(&mut S) -> Option<i32>`
              found fn pointer `fn(&mut S) -> Result<i32, i32>`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&mut S) -> Option<i32>`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&mut S) -> Option<i32> as fn(&mut S) -> Result<i32, i32>)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-24322.rs stdout ----
diff of stderr:

8    |
9    = note: expected reference `&for<'a> fn(&'a B) -> u32`
10               found reference `&for<'a> fn(&'a B) -> u32 {B::func}`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(B::func as for<'a> fn(&'a B) -> u32)`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24322/issue-24322.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-24322.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24322.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24322" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24322/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-24322.rs:8:29
   |
   |
LL |     let x: &fn(&B) -> u32 = &B::func; //~ ERROR mismatched types
   |            --------------   ^^^^^^^^ expected fn pointer, found fn item
   |            expected due to this
   |
   |
   = note: expected reference `&for<'a> fn(&'a B) -> u32`
              found reference `&for<'a> fn(&'a B) -> u32 {B::func}`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(B::func as for<'a> fn(&'a B) -> u32)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
   |
LL |     foo > 12;
   |           ^^ expected fn item, found integer
   |
   = note: expected fn item `fn() -> i32 {foo}`


error[E0369]: binary operation `>` cannot be applied to type `fn(i64) -> i64 {bar}`
   |
LL |     bar > 13;
LL |     bar > 13;
   |     --- ^ -- {integer}
   |     |
   |     fn(i64) -> i64 {bar}
help: use parentheses to call this function
   |
   |
LL |     bar(/* i64 */) > 13;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-59488.rs:18:11
   |
   |
LL |     bar > 13;
   |           ^^ expected fn item, found integer
   |
   = note: expected fn item `fn(i64) -> i64 {bar}`


error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
   |
LL |     foo > foo;
LL |     foo > foo;
   |     --- ^ --- fn() -> i32 {foo}
   |     fn() -> i32 {foo}
   |
help: use parentheses to call these
   |
   |
LL |     foo() > foo();
   |        ++      ++

error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
   |
   |
LL |     foo > bar;
   |     --- ^ --- fn(i64) -> i64 {bar}
   |     fn() -> i32 {foo}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-59488.rs:25:11
  --> /checkout/src/test/ui/issues/issue-59488.rs:25:11
   |
LL |     foo > bar;
   |           ^^^ expected fn item, found a different fn item
   |
   = note: expected fn item `fn() -> i32 {foo}`
              found fn item `fn(i64) -> i64 {bar}`
   = note: different `fn` items always have unique types, even if their signatures are the same

error[E0369]: binary operation `==` cannot be applied to type `fn(usize) -> Foo {Foo::Bar}`
   |
   |
LL |     assert_eq!(Foo::Bar, i);
   |     |
   |     |
   |     fn(usize) -> Foo {Foo::Bar}
   |     fn(usize) -> Foo {Foo::Bar}
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `Debug`
   |
   |
LL |     assert_eq!(Foo::Bar, i);
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for fn item `fn(usize) -> Foo {Foo::Bar}`
   = help: the following other types implement trait `Debug`:
             extern "C" fn() -> Ret
             extern "C" fn(A, B) -> Ret
             extern "C" fn(A, B, ...) -> Ret
             extern "C" fn(A, B, C) -> Ret
             extern "C" fn(A, B, C, ...) -> Ret
             extern "C" fn(A, B, C, D) -> Ret
             extern "C" fn(A, B, C, D, ...) -> Ret
             extern "C" fn(A, B, C, D, E) -> Ret
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `Debug`
   |
   |
LL |     assert_eq!(Foo::Bar, i);
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for fn item `fn(usize) -> Foo {Foo::Bar}`
   = help: the following other types implement trait `Debug`:
             extern "C" fn() -> Ret
             extern "C" fn(A, B) -> Ret
             extern "C" fn(A, B, ...) -> Ret
             extern "C" fn(A, B, C) -> Ret
             extern "C" fn(A, B, C, ...) -> Ret
             extern "C" fn(A, B, C, D) -> Ret
             extern "C" fn(A, B, C, D, ...) -> Ret
             extern "C" fn(A, B, C, D, E) -> Ret
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 10 previous errors

---
---- [ui] src/test/ui/issues/issue-9575.rs stdout ----
diff of stderr:

6    |
7    = note: expected fn pointer `fn(isize, *const *const u8) -> _`
8               found fn pointer `fn(isize, *const *const u8, *const u8) -> _`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(isize, *const *const u8) -> isize`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(isize, *const *const u8) -> isize as fn(isize, *const *const u8, *const u8) -> isize)`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-9575/issue-9575.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-9575.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-9575.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-9575" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-9575/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0308]: `#[start]` function has wrong type
   |
   |
LL | fn start(argc: isize, argv: *const *const u8, crate_map: *const u8) -> isize {
   |
   |
   = note: expected fn pointer `fn(isize, *const *const u8) -> _`
              found fn pointer `fn(isize, *const *const u8, *const u8) -> _`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(isize, *const *const u8) -> isize`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(isize, *const *const u8) -> isize as fn(isize, *const *const u8, *const u8) -> isize)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs#leak stdout ----
diff of stderr:

14    |
15    = note: expected fn pointer `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
16               found fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8 as for<'a> fn(&'a u8, &'a u8) -> &'a u8)`
18 error: aborting due to previous error
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq1.leak/old-lub-glb-hr-noteq1.leak.stderr
To only update this specific test, also pass `--test-args lub-glb/old-lub-glb-hr-noteq1.rs`


error in revision `leak`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "leak" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq1.leak" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq1.leak/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs:14:14
   |
   |
LL |       let z = match 22 {
LL | |         0 => x,
LL | |         0 => x,
   | |              - this is found to be of type `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
LL | |         _ => y,
   | |              ^ one type is more general than the other
LL | |         //[leak]~^ ERROR `match` arms have incompatible types
LL | |         //[noleak]~^^ ERROR mismatched types
LL | |     };
   |
   |
   = note: expected fn pointer `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
              found fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8 as for<'a> fn(&'a u8, &'a u8) -> &'a u8)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs#noleak stdout ----
diff of stderr:

6    |
7    = note: expected fn pointer `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
8               found fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8 as for<'a> fn(&'a u8, &'a u8) -> &'a u8)`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq1.noleak/old-lub-glb-hr-noteq1.noleak.stderr
To only update this specific test, also pass `--test-args lub-glb/old-lub-glb-hr-noteq1.rs`


error in revision `noleak`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noleak" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq1.noleak" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq1.noleak/auxiliary" "-Zno-leak-check"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs:14:14
   |
LL |         _ => y,
LL |         _ => y,
   |              ^ one type is more general than the other
   |
   = note: expected fn pointer `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
              found fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8 as for<'a> fn(&'a u8, &'a u8) -> &'a u8)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs#leak stdout ----
diff of stderr:

13    |
14    = note: expected fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
15               found fn pointer `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a> fn(&'a u8, &'a u8) -> &'a u8 as for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8)`
17 error: aborting due to previous error
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq2.leak/old-lub-glb-hr-noteq2.leak.stderr
To only update this specific test, also pass `--test-args lub-glb/old-lub-glb-hr-noteq2.rs`


error in revision `leak`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "leak" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq2.leak" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq2.leak/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs:25:14
   |
   |
LL |       let z = match 22 {
LL | |         0 => y,
LL | |         0 => y,
   | |              - this is found to be of type `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
LL | |         _ => x,
   | |              ^ one type is more general than the other
LL | |         //[leak]~^ ERROR `match` arms have incompatible types
LL | |     };
   |
   |
   = note: expected fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
              found fn pointer `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a> fn(&'a u8, &'a u8) -> &'a u8 as for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/main-wrong-type.rs stdout ----
diff of stderr:

6    |
7    = note: expected fn pointer `fn()`
8               found fn pointer `fn(S)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn()`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn() as fn(S))`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/main-wrong-type/main-wrong-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args main-wrong-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/main-wrong-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/main-wrong-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/main-wrong-type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0580]: `main` function has wrong type
   |
   |
LL | fn main(foo: S) {
   |
   = note: expected fn pointer `fn()`
              found fn pointer `fn(S)`
              found fn pointer `fn(S)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn()`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn() as fn(S))`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0580`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/mismatched_types/E0053.rs stdout ----
diff of stderr:

14    |               ^^^
15    = note: expected fn pointer `fn(u16)`
16               found fn pointer `fn(i16)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(u16)`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(u16) as fn(i16))`
18 error[E0053]: method `bar` has an incompatible type for trait
19   --> $DIR/E0053.rs:11:12

31    |            ^^^^^
31    |            ^^^^^
32    = note: expected fn pointer `fn(&Bar)`
33               found fn pointer `fn(&mut Bar)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(&Bar)`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&Bar) as fn(&mut Bar))`
35 error: aborting due to 2 previous errors
36 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/E0053/E0053.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/E0053.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/E0053.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/E0053" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/E0053/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0053]: method `foo` has an incompatible type for trait
   |
   |
LL |     fn foo(x: i16) { }
   |               |
   |               expected `u16`, found `i16`
   |               help: change the parameter type to match the trait: `u16`
   |
   |
note: type in trait
  --> /checkout/src/test/ui/mismatched_types/E0053.rs:2:15
   |
LL |     fn foo(x: u16);
   = note: expected fn pointer `fn(u16)`
              found fn pointer `fn(i16)`
              found fn pointer `fn(i16)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(u16)`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(u16) as fn(i16))`
error[E0053]: method `bar` has an incompatible type for trait
  --> /checkout/src/test/ui/mismatched_types/E0053.rs:11:12
   |
LL |     fn bar(&mut self) { }
LL |     fn bar(&mut self) { }
   |            ^^^^^^^^^
   |            |
   |            types differ in mutability
   |            help: change the self-receiver type to match the trait: `self: &Bar`
note: type in trait
  --> /checkout/src/test/ui/mismatched_types/E0053.rs:3:12
   |
LL |     fn bar(&self);
LL |     fn bar(&self);
   |            ^^^^^
   = note: expected fn pointer `fn(&Bar)`
              found fn pointer `fn(&mut Bar)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&Bar)`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&Bar) as fn(&mut Bar))`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/mismatched_types/normalize-fn-sig.rs stdout ----
diff of stderr:

8    |
9    = note: expected fn pointer `fn(&'static i32, i32)`
10                  found fn item `fn(i32, &'static i32) {foo::<()>}`
+    = note: different `fn` items always have unique types, even if their signatures are the same
12   --> $DIR/normalize-fn-sig.rs:11:4
13    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/normalize-fn-sig/normalize-fn-sig.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/normalize-fn-sig.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/normalize-fn-sig.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/normalize-fn-sig" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/normalize-fn-sig/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/mismatched_types/normalize-fn-sig.rs:14:22
   |
   |
LL |     needs_i32_ref_fn(foo::<()>);
   |     ---------------- ^^^^^^^^^ expected `&i32`, found `i32`
   |     arguments to this function are incorrect
   |
   |
   = note: expected fn pointer `fn(&'static i32, i32)`
                 found fn item `fn(i32, &'static i32) {foo::<()>}`
   = note: different `fn` items always have unique types, even if their signatures are the same
  --> /checkout/src/test/ui/mismatched_types/normalize-fn-sig.rs:11:4
   |
   |
LL | fn needs_i32_ref_fn(_: fn(&'static i32, i32)) {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/mismatched_types/trait-impl-fn-incompatibility.rs stdout ----
diff of stderr:

14    |               ^^^
15    = note: expected fn pointer `fn(u16)`
16               found fn pointer `fn(i16)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(u16)`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(u16) as fn(i16))`
18 error[E0053]: method `bar` has an incompatible type for trait
19   --> $DIR/trait-impl-fn-incompatibility.rs:10:28

31    |                            ^^^^^^^^
31    |                            ^^^^^^^^
32    = note: expected fn pointer `fn(&mut Bar, &mut Bar)`
33               found fn pointer `fn(&mut Bar, &Bar)`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `fn(&mut Bar, &mut Bar)`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&mut Bar, &mut Bar) as fn(&mut Bar, &Bar))`
35 error: aborting due to 2 previous errors
36 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/trait-impl-fn-incompatibility/trait-impl-fn-incompatibility.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/trait-impl-fn-incompatibility.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/trait-impl-fn-incompatibility.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/trait-impl-fn-incompatibility" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/trait-impl-fn-incompatibility/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0053]: method `foo` has an incompatible type for trait
   |
   |
LL |     fn foo(x: i16) { } //~ ERROR incompatible type
   |               |
   |               expected `u16`, found `i16`
   |               help: change the parameter type to match the trait: `u16`
   |
   |
note: type in trait
  --> /checkout/src/test/ui/mismatched_types/trait-impl-fn-incompatibility.rs:2:15
   |
LL |     fn foo(x: u16);
   = note: expected fn pointer `fn(u16)`
              found fn pointer `fn(i16)`
              found fn pointer `fn(i16)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(u16)`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(u16) as fn(i16))`
error[E0053]: method `bar` has an incompatible type for trait
  --> /checkout/src/test/ui/mismatched_types/trait-impl-fn-incompatibility.rs:10:28
   |
   |
LL |     fn bar(&mut self, bar: &Bar) { } //~ ERROR incompatible type
   |                            |
   |                            types differ in mutability
   |                            help: change the parameter type to match the trait: `&mut Bar`
   |
   |
note: type in trait
  --> /checkout/src/test/ui/mismatched_types/trait-impl-fn-incompatibility.rs:3:28
   |
LL |     fn bar(&mut self, bar: &mut Bar);
   |                            ^^^^^^^^
   = note: expected fn pointer `fn(&mut Bar, &mut Bar)`
              found fn pointer `fn(&mut Bar, &Bar)`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `fn(&mut Bar, &mut Bar)`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(fn(&mut Bar, &mut Bar) as fn(&mut Bar, &Bar))`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/relate_tys/fn-subtype.rs stdout ----
diff of stderr:

6    |
7    = note: expected fn pointer `for<'a> fn(&'a ())`
8               found fn pointer `fn(&())`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `for<'a> fn(&'a ())`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a> fn(&'a ()) as fn(&()))`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/fn-subtype/fn-subtype.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/relate_tys/fn-subtype.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/relate_tys/fn-subtype.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/fn-subtype" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/fn-subtype/auxiliary" "-Zno-leak-check"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/nll/relate_tys/fn-subtype.rs:7:33
   |
   |
LL |     let y: for<'a> fn(&'a ()) = x; //~ ERROR mismatched types [E0308]
   |                                 ^ one type is more general than the other
   |
   = note: expected fn pointer `for<'a> fn(&'a ())`
              found fn pointer `fn(&())`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer reference `for<'a> fn(&'a ())`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a> fn(&'a ()) as fn(&()))`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/relate_tys/hr-fn-aaa-as-aba.rs stdout ----
diff of stderr:

6    |
7    = note: expected fn pointer `for<'a, 'b> fn(&'a u32, &'b u32) -> &'a u32`
8               found fn pointer `for<'a> fn(&'a u32, &'a u32) -> &'a u32`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `for<'a, 'b> fn(&'a u32, &'b u32) -> &'a u32`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a, 'b> fn(&'a u32, &'b u32) -> &'a u32 as for<'a> fn(&'a u32, &'a u32) -> &'a u32)`
10 error[E0308]: mismatched types
11   --> $DIR/hr-fn-aaa-as-aba.rs:20:12

15    |
15    |
16    = note: expected fn pointer `for<'a, 'b> fn(&'a u32, &'b u32) -> &'a u32`
17               found fn pointer `for<'a> fn(&'a u32, &'a u32) -> &'a u32`
+    = note: different `fn` items always have unique types, even if their signatures are the same
+    = help: change the expected type to be function pointer reference `for<'a, 'b> fn(&'a u32, &'b u32) -> &'a u32`
+    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `&(for<'a, 'b> fn(&'a u32, &'b u32) -> &'a u32 as for<'a> fn(&'a u32, &'a u32) -> &'a u32)`
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/hr-fn-aaa-as-aba/hr-fn-aaa-as-aba.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/relate_tys/hr-fn-aaa-as-aba.rs`
