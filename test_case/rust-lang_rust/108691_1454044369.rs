plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:0fdabd83e1d3faaa8e9cfd7c00031e3a92997344)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 8800/14556
........................................................................................ 8888/14556
........F............................................................................... 8976/14556
........................................................................................ 9064/14556
..........................................FF.......FF.F.........F..F.................... 9152/14556
..................ii.................................................................... 9328/14556
........................................................................................ 9416/14556
........................................................................i............... 9504/14556
.........................i.............................................................. 9592/14556
---
diff of stderr:

10                (),
11            ]
12    = note: number of external vids: 2
-    = note: where T: '_#1r
+    = note: where ClosureOutlivesSubjectTy { inner: T }: '_#1r
15 note: no external requirements
16   --> $DIR/propagate-from-trait-match.rs:28:1



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-from-trait-match/propagate-from-trait-match.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-from-trait-match.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/closure-requirements/propagate-from-trait-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-from-trait-match" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-from-trait-match/auxiliary" "-Zverbose"
stdout: none
--- stderr -------------------------------
note: external requirements
  --> fake-test-src-base/nll/closure-requirements/propagate-from-trait-match.rs:32:36
   |
LL |     establish_relationships(value, |value| {
   |
   |
   = note: defining type: supply::<'_#1r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((T,)),
               (),
           ]
   = note: number of external vids: 2
   = note: where ClosureOutlivesSubjectTy { inner: T }: '_#1r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/closure-requirements/propagate-from-trait-match.rs:28:1
   |
LL | / fn supply<'a, T>(value: T)
LL | | where
LL | |     T: Trait<'a>,
   |
   |
   = note: defining type: supply::<'_#1r, T>
error[E0309]: the parameter type `T` may not live long enough
error[E0309]: the parameter type `T` may not live long enough
  --> fake-test-src-base/nll/closure-requirements/propagate-from-trait-match.rs:43:9
LL |         require(value);
LL |         require(value);
   |         ^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL |     T: Trait<'a> + 'a,

error: aborting due to previous error

For more information about this error, try `rustc --explain E0309`.
---
diff of stderr:

10                (),
11            ]
12    = note: number of external vids: 3
-    = note: where <T as std::iter::Iterator>::Item: '_#2r
+    = note: where ClosureOutlivesSubjectTy { inner: <T as std::iter::Iterator>::Item }: '_#2r
15 note: no external requirements
16   --> $DIR/projection-no-regions-closure.rs:21:1

43                (),
43                (),
44            ]
45    = note: number of external vids: 3
-    = note: where <T as std::iter::Iterator>::Item: '_#2r
+    = note: where ClosureOutlivesSubjectTy { inner: <T as std::iter::Iterator>::Item }: '_#2r
48 note: no external requirements
49   --> $DIR/projection-no-regions-closure.rs:30:1

67                (),
67                (),
68            ]
69    = note: number of external vids: 4
-    = note: where <T as std::iter::Iterator>::Item: '_#3r
+    = note: where ClosureOutlivesSubjectTy { inner: <T as std::iter::Iterator>::Item }: '_#3r
72 note: no external requirements
73   --> $DIR/projection-no-regions-closure.rs:38:1

100                (),
100                (),
101            ]
102    = note: number of external vids: 4
-    = note: where <T as std::iter::Iterator>::Item: '_#3r
+    = note: where ClosureOutlivesSubjectTy { inner: <T as std::iter::Iterator>::Item }: '_#3r
105 note: no external requirements
106   --> $DIR/projection-no-regions-closure.rs:47:1



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-no-regions-closure/projection-no-regions-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/projection-no-regions-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/ty-outlives/projection-no-regions-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-no-regions-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-no-regions-closure/auxiliary" "-Zverbose"
stdout: none
--- stderr -------------------------------
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-no-regions-closure.rs:25:23
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
   = note: where ClosureOutlivesSubjectTy { inner: <T as std::iter::Iterator>::Item }: '_#2r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-no-regions-closure.rs:21:1
   |
LL | / fn no_region<'a, T>(x: Box<T>) -> Box<dyn Anything + 'a>
LL | | where
LL | |     T: Iterator,
   |
   |
   = note: defining type: no_region::<'_#1r, T>

error[E0309]: the associated type `<T as Iterator>::Item` may not live long enough
  --> fake-test-src-base/nll/ty-outlives/projection-no-regions-closure.rs:25:31
   |
LL |     with_signature(x, |mut y| Box::new(y.next()))
   |
   |
   = help: consider adding an explicit lifetime bound `<T as Iterator>::Item: 'a`...
   = note: ...so that the type `<T as Iterator>::Item` will meet its required lifetime bounds
note: external requirements
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-no-regions-closure.rs:34:23
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
   = note: where ClosureOutlivesSubjectTy { inner: <T as std::iter::Iterator>::Item }: '_#2r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-no-regions-closure.rs:30:1
   |
LL | / fn correct_region<'a, T>(x: Box<T>) -> Box<dyn Anything + 'a>
LL | | where
LL | |     T: 'a + Iterator,
   |
   |
   = note: defining type: correct_region::<'_#1r, T>
note: external requirements
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-no-regions-closure.rs:42:23
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
   = note: where ClosureOutlivesSubjectTy { inner: <T as std::iter::Iterator>::Item }: '_#3r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-no-regions-closure.rs:38:1
   |
LL | / fn wrong_region<'a, 'b, T>(x: Box<T>) -> Box<dyn Anything + 'a>
LL | | where
LL | |     T: 'b + Iterator,
   |
   |
   = note: defining type: wrong_region::<'_#1r, '_#2r, T>

error[E0309]: the associated type `<T as Iterator>::Item` may not live long enough
  --> fake-test-src-base/nll/ty-outlives/projection-no-regions-closure.rs:42:31
   |
LL |     with_signature(x, |mut y| Box::new(y.next()))
   |
   |
   = help: consider adding an explicit lifetime bound `<T as Iterator>::Item: 'a`...
   = note: ...so that the type `<T as Iterator>::Item` will meet its required lifetime bounds
note: external requirements
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-no-regions-closure.rs:52:23
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
   = note: where ClosureOutlivesSubjectTy { inner: <T as std::iter::Iterator>::Item }: '_#3r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-no-regions-closure.rs:47:1
   |
LL | / fn outlives_region<'a, 'b, T>(x: Box<T>) -> Box<dyn Anything + 'a>
LL | | where
LL | |     T: 'b + Iterator,
LL | |     'b: 'a,
   |
   |
   = note: defining type: outlives_region::<'_#1r, '_#2r, T>
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0309`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/nll/ty-outlives/projection-one-region-closure.rs stdout ----
diff of stderr:

11            ]
12    = note: late-bound region is '_#3r
13    = note: number of external vids: 4
-    = note: where T: '_#2r
+    = note: where ClosureOutlivesSubjectTy { inner: T }: '_#2r
15    = note: where '_#1r: '_#2r
17 note: no external requirements

60                (),
61            ]
61            ]
62    = note: number of external vids: 4
-    = note: where T: '_#3r
+    = note: where ClosureOutlivesSubjectTy { inner: T }: '_#3r
64    = note: where '_#2r: '_#3r
66 note: no external requirements

110                (),
111            ]
111            ]
112    = note: number of external vids: 4
-    = note: where <T as Anything<ReEarlyBound(1, 'b)>>::AssocType: '_#3r
+    = note: where ClosureOutlivesSubjectTy { inner: <T as Anything<ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrAnon(0, None) })>>::AssocType }: '_#3r
115 note: no external requirements
116   --> $DIR/projection-one-region-closure.rs:62:1

135                (),
135                (),
136            ]
137    = note: number of external vids: 4
-    = note: where T: '_#3r
+    = note: where ClosureOutlivesSubjectTy { inner: T }: '_#3r
139    = note: where '_#2r: '_#3r
141 note: no external requirements


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/projection-one-region-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/projection-one-region-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/ty-outlives/projection-one-region-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/auxiliary" "-Zverbose"
stdout: none
--- stderr -------------------------------
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-closure.rs:45:29
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: no_relationships_late::<'_#1r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#2r ()>, T)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#3r
   = note: number of external vids: 4
   = note: where ClosureOutlivesSubjectTy { inner: T }: '_#2r
   = note: where '_#1r: '_#2r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-closure.rs:41:1
   |
LL | / fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
   |
   |
   = note: defining type: no_relationships_late::<'_#1r, T>
error[E0309]: the parameter type `T` may not live long enough
error[E0309]: the parameter type `T` may not live long enough
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-closure.rs:45:39
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |                                       ^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL |     T: Anything<'b> + 'a,

error: lifetime may not live long enough
error: lifetime may not live long enough
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-closure.rs:45:39
   |
LL | fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
   |                          --  -- lifetime `'b` defined here
   |                          |
   |                          lifetime `'a` defined here
...
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
note: external requirements
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-closure.rs:56:29
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: no_relationships_early::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
               (),
           ]
   = note: number of external vids: 4
   = note: where ClosureOutlivesSubjectTy { inner: T }: '_#3r
   = note: where '_#2r: '_#3r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-closure.rs:51:1
   |
LL | / fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
LL | |     'a: 'a,
   |
   |
   = note: defining type: no_relationships_early::<'_#1r, '_#2r, T>
error[E0309]: the parameter type `T` may not live long enough
error[E0309]: the parameter type `T` may not live long enough
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-closure.rs:56:39
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |                                       ^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL |     T: Anything<'b> + 'a,

error: lifetime may not live long enough
error: lifetime may not live long enough
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-closure.rs:56:39
   |
LL | fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
   |                           --  -- lifetime `'b` defined here
   |                           |
   |                           lifetime `'a` defined here
...
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
note: external requirements
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-closure.rs:70:29
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: projection_outlives::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
               (),
           ]
   = note: number of external vids: 4
   = note: where ClosureOutlivesSubjectTy { inner: <T as Anything<ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrAnon(0, None) })>>::AssocType }: '_#3r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-closure.rs:62:1
   |
LL | / fn projection_outlives<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
LL | |     T::AssocType: 'a,
   |
   |
   = note: defining type: projection_outlives::<'_#1r, '_#2r, T>
note: external requirements
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-closure.rs:80:29
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: elements_outlive::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
               (),
           ]
   = note: number of external vids: 4
   = note: where ClosureOutlivesSubjectTy { inner: T }: '_#3r
   = note: where '_#2r: '_#3r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-closure.rs:74:1
   |
LL | / fn elements_outlive<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
LL | |     T: 'a,
LL | |     'b: 'a,
   |
   |
   = note: defining type: elements_outlive::<'_#1r, '_#2r, T>
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0309`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs stdout ----
diff of stderr:

86                (),
87            ]
88    = note: number of external vids: 4
-    = note: where <T as Anything<ReEarlyBound(1, 'b)>>::AssocType: '_#3r
+    = note: where ClosureOutlivesSubjectTy { inner: <T as Anything<ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrAnon(0, None) })>>::AssocType }: '_#3r
91 note: no external requirements
92   --> $DIR/projection-one-region-trait-bound-closure.rs:52:1



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure/projection-one-region-trait-bound-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/projection-one-region-trait-bound-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure/auxiliary" "-Zverbose"
stdout: none
--- stderr -------------------------------
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:37:29
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: no_relationships_late::<'_#1r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#2r ()>, T)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#3r
   = note: number of external vids: 4
   = note: where '_#1r: '_#2r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:33:1
   |
LL | / fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
   |
   |
   = note: defining type: no_relationships_late::<'_#1r, T>
error: lifetime may not live long enough
error: lifetime may not live long enough
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:37:39
   |
LL | fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
   |                          --  -- lifetime `'b` defined here
   |                          |
   |                          lifetime `'a` defined here
...
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
note: external requirements
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:47:29
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: no_relationships_early::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
               (),
           ]
   = note: number of external vids: 4
   = note: where '_#2r: '_#3r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:42:1
   |
LL | / fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
LL | |     'a: 'a,
   |
   |
   = note: defining type: no_relationships_early::<'_#1r, '_#2r, T>
error: lifetime may not live long enough
error: lifetime may not live long enough
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:47:39
   |
LL | fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
   |                           --  -- lifetime `'b` defined here
   |                           |
   |                           lifetime `'a` defined here
...
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
note: external requirements
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:60:29
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: projection_outlives::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
               (),
           ]
   = note: number of external vids: 4
   = note: where ClosureOutlivesSubjectTy { inner: <T as Anything<ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrAnon(0, None) })>>::AssocType }: '_#3r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:52:1
   |
LL | / fn projection_outlives<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
LL | |     T::AssocType: 'a,
   |
   |
---
diff of stderr:

10                (),
11            ]
12    = note: number of external vids: 3
-    = note: where T: '_#2r
+    = note: where ClosureOutlivesSubjectTy { inner: T }: '_#2r
15 note: no external requirements
16   --> $DIR/ty-param-closure-outlives-from-return-type.rs:15:1



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type/ty-param-closure-outlives-from-return-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type/auxiliary" "-Zverbose"
stdout: none
--- stderr -------------------------------
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs:26:23
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
   = note: where ClosureOutlivesSubjectTy { inner: T }: '_#2r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs:15:1
   |
LL | / fn no_region<'a, T>(x: Box<T>) -> Box<dyn Debug + 'a>
LL | | where
LL | |     T: Debug,
   |
   |
   = note: defining type: no_region::<'_#1r, T>
error[E0309]: the parameter type `T` may not live long enough
error[E0309]: the parameter type `T` may not live long enough
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs:26:27
   |
LL |     with_signature(x, |y| y)
   |                           ^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL |     T: Debug + 'a,

error[E0309]: the parameter type `T` may not live long enough
error[E0309]: the parameter type `T` may not live long enough
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs:41:5
LL |     x
LL |     x
   |     ^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL |     T: 'b + Debug + 'a,

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0309`.
For more information about this error, try `rustc --explain E0309`.
------------------------------------------


---- [ui] tests/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs stdout ----
diff of stderr:

11            ]
12    = note: late-bound region is '_#2r
13    = note: number of external vids: 3
-    = note: where T: '_#1r
+    = note: where ClosureOutlivesSubjectTy { inner: T }: '_#1r
16 note: no external requirements
17   --> $DIR/ty-param-closure-outlives-from-where-clause.rs:26:1

44                (),
44                (),
45            ]
46    = note: number of external vids: 3
-    = note: where T: '_#2r
+    = note: where ClosureOutlivesSubjectTy { inner: T }: '_#2r
49 note: no external requirements
50   --> $DIR/ty-param-closure-outlives-from-where-clause.rs:38:1

69            ]
69            ]
70    = note: late-bound region is '_#3r
71    = note: number of external vids: 4
-    = note: where T: '_#2r
+    = note: where ClosureOutlivesSubjectTy { inner: T }: '_#2r
74 note: no external requirements
75   --> $DIR/ty-param-closure-outlives-from-where-clause.rs:59:1

104                (),
104                (),
105            ]
106    = note: number of external vids: 4
-    = note: where T: '_#3r
+    = note: where ClosureOutlivesSubjectTy { inner: T }: '_#3r
109 note: no external requirements
110   --> $DIR/ty-param-closure-outlives-from-where-clause.rs:71:1



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause/ty-param-closure-outlives-from-where-clause.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause/auxiliary" "-Zverbose"
stdout: none
--- stderr -------------------------------
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:27:26
   |
LL |     with_signature(a, b, |x, y| {
   |
   |
   = note: defining type: no_region::<T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#1r ()>, T)),
               (),
   = note: late-bound region is '_#2r
   = note: late-bound region is '_#2r
   = note: number of external vids: 3
   = note: where ClosureOutlivesSubjectTy { inner: T }: '_#1r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:26:1
   |
LL | fn no_region<'a, T>(a: Cell<&'a ()>, b: T) {
   |
   = note: defining type: no_region::<T>

error[E0309]: the parameter type `T` may not live long enough
error[E0309]: the parameter type `T` may not live long enough
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:32:9
   |
LL |         require(&x, &y)
   |         ^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn no_region<'a, T: 'a>(a: Cell<&'a ()>, b: T) {

note: external requirements
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:42:26
   |
LL |     with_signature(a, b, |x, y| {
   |
   |
   = note: defining type: correct_region::<'_#1r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#2r ()>, T)),
               (),
           ]
   = note: number of external vids: 3
   = note: where ClosureOutlivesSubjectTy { inner: T }: '_#2r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:38:1
   |
LL | / fn correct_region<'a, T>(a: Cell<&'a ()>, b: T)
LL | | where
LL | |     T: 'a,
   |
   |
   = note: defining type: correct_region::<'_#1r, T>
note: external requirements
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:63:26
   |
LL |     with_signature(a, b, |x, y| {
   |
   |
   = note: defining type: wrong_region::<'_#1r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#2r ()>, T)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#3r
   = note: number of external vids: 4
   = note: where ClosureOutlivesSubjectTy { inner: T }: '_#2r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:59:1
   |
LL | / fn wrong_region<'a, 'b, T>(a: Cell<&'a ()>, b: T)
LL | | where
LL | |     T: 'b,
   |
   |
   = note: defining type: wrong_region::<'_#1r, T>
error[E0309]: the parameter type `T` may not live long enough
error[E0309]: the parameter type `T` may not live long enough
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:65:9
   |
LL |         require(&x, &y)
   |         ^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL |     T: 'b + 'a,

note: external requirements
note: external requirements
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:76:26
   |
LL |     with_signature(a, b, |x, y| {
   |
   |
   = note: defining type: outlives_region::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
               (),
           ]
   = note: number of external vids: 4
   = note: where ClosureOutlivesSubjectTy { inner: T }: '_#3r
note: no external requirements
note: no external requirements
  --> fake-test-src-base/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:71:1
   |
LL | / fn outlives_region<'a, 'b, T>(a: Cell<&'a ()>, b: T)
LL | | where
LL | |     T: 'b,
LL | |     'b: 'a,
   |
   |
   = note: defining type: outlives_region::<'_#1r, '_#2r, T>
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0309`.
------------------------------------------
