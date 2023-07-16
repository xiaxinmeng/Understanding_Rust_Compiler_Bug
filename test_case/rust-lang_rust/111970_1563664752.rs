plain

---- [ui] tests/ui/error-codes/E0401.rs stdout ----
diff of stderr:

53            - impl<A, F> Fn<A> for &F
54              where A: Tuple, F: Fn<A>, F: ?Sized;
55            - impl<Args, F, A> Fn<Args> for Box<F, A>
-              where Args: Tuple, F: Fn<Args>, A: Allocator, F: ?Sized;
+              where Args: Tuple, F: Fn<Args>, A: alloc::falloc::Allocator, F: ?Sized;
57 note: required by a bound in `bfnr`
58   --> $DIR/E0401.rs:4:30


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0401/E0401.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0401/E0401.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0401.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/error-codes/E0401.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0401" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0401/auxiliary"
stdout: none
error[E0401]: can't use generic parameters from outer function
  --> fake-test-src-base/error-codes/E0401.rs:4:39
   |
LL | fn foo<T>(x: T) {
LL | fn foo<T>(x: T) {
   |        - type parameter from outer function
LL |     fn bfnr<U, V: Baz<U>, W: Fn()>(y: T) { //~ ERROR E0401
   |             -                         ^ use of generic parameter from outer function
   |             |
   |             help: try using a local generic parameter instead: `T,`
error[E0401]: can't use generic parameters from outer function
  --> fake-test-src-base/error-codes/E0401.rs:9:16
   |
LL | fn foo<T>(x: T) {
LL | fn foo<T>(x: T) {
   |        - type parameter from outer function
...
LL |     fn baz<U,
   |            - help: try using a local generic parameter instead: `T,`
...
LL |            (y: T) { //~ ERROR E0401
   |                ^ use of generic parameter from outer function
error[E0401]: can't use generic parameters from outer function
  --> fake-test-src-base/error-codes/E0401.rs:24:25
   |
   |
LL | impl<T> Iterator for A<T> {
   | ---- `Self` type implicitly declared here, by this `impl`
...
LL |         fn helper(sel: &Self) -> u8 { //~ ERROR E0401
   |                         |
   |                         use of generic parameter from outer function
   |                         use a type here instead


error[E0282]: type annotations needed
  --> fake-test-src-base/error-codes/E0401.rs:11:5
   |
LL |     bfnr(x);
   |     ^^^^ cannot infer type of the type parameter `U` declared on the function `bfnr`
help: consider specifying the generic arguments
   |
   |
LL |     bfnr::<U, V, W>(x);

error[E0283]: type annotations needed
  --> fake-test-src-base/error-codes/E0401.rs:11:5
   |
   |
LL |     bfnr(x);
   |     ^^^^ cannot infer type of the type parameter `W` declared on the function `bfnr`
   |
   = note: multiple `impl`s satisfying `_: Fn<()>` found in the following crates: `alloc`, `core`:
           - impl<A, F> Fn<A> for &F
             where A: Tuple, F: Fn<A>, F: ?Sized;
           - impl<Args, F, A> Fn<Args> for Box<F, A>
             where Args: Tuple, F: Fn<Args>, A: alloc::falloc::Allocator, F: ?Sized;
note: required by a bound in `bfnr`
  --> fake-test-src-base/error-codes/E0401.rs:4:30
   |
LL |     fn bfnr<U, V: Baz<U>, W: Fn()>(y: T) { //~ ERROR E0401
   |                              ^^^^ required by this bound in `bfnr`
help: consider specifying the generic arguments
   |
LL |     bfnr::<U, V, W>(x);

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0282, E0283, E0401.
---
---- [ui] tests/ui/error-codes/e0119/conflict-with-std.rs stdout ----
diff of stderr:

6    |
7    = note: conflicting implementation in crate `alloc`:
8            - impl<T, A> AsRef<T> for Box<T, A>
-              where A: Allocator, T: ?Sized;
+              where A: alloc::falloc::Allocator, T: ?Sized;
10 
11 error[E0119]: conflicting implementations of trait `From<S>` for type `S`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/e0119/conflict-with-std/conflict-with-std.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/e0119/conflict-with-std/conflict-with-std.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/e0119/conflict-with-std.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/error-codes/e0119/conflict-with-std.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/e0119/conflict-with-std" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/e0119/conflict-with-std/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0119]: conflicting implementations of trait `AsRef<Q>` for type `Box<Q>`
  --> fake-test-src-base/error-codes/e0119/conflict-with-std.rs:5:1
   |
LL | impl AsRef<Q> for Box<Q> { //~ ERROR conflicting implementations
   |
   = note: conflicting implementation in crate `alloc`:
   = note: conflicting implementation in crate `alloc`:
           - impl<T, A> AsRef<T> for Box<T, A>
             where A: alloc::falloc::Allocator, T: ?Sized;

error[E0119]: conflicting implementations of trait `From<S>` for type `S`
  --> fake-test-src-base/error-codes/e0119/conflict-with-std.rs:12:1
   |
LL | impl From<S> for S { //~ ERROR conflicting implementations
   |
   = note: conflicting implementation in crate `core`:
   = note: conflicting implementation in crate `core`:
           - impl<T> From<T> for T;

error[E0119]: conflicting implementations of trait `TryFrom<X>` for type `X`
  --> fake-test-src-base/error-codes/e0119/conflict-with-std.rs:19:1
   |
LL | impl TryFrom<X> for X { //~ ERROR conflicting implementations
   |
   = note: conflicting implementation in crate `core`:
   = note: conflicting implementation in crate `core`:
           - impl<T, U> TryFrom<U> for T
             where U: Into<T>;
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0119`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/hygiene/panic-location.rs stdout ----
diff of run.stderr:

- thread 'main' panicked at 'capacity overflow', library/alloc/src/raw_vec.rs:524:5
+ thread 'main' panicked at 'capacity overflow', library/alloc/src/falloc.rs:78:5
3 


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/panic-location.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location" && RUST_BACKTRACE="0" RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a"
stdout: none
thread 'main' panicked at 'capacity overflow', library/alloc/src/falloc.rs:78:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
------------------------------------------

