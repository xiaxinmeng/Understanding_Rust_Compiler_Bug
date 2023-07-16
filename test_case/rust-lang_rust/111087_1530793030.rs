plain
........................................................................................   528/14892
........................................................................................   616/14892
........................................................................................   704/14892
........................................................................................   792/14892
....................................................FF.F................................   880/14892
...............i........................................................................  1056/14892
..................................i.....................................................  1144/14892
........................................................................................  1232/14892
........................................................................................  1320/14892
---
failures:

---- [ui] tests/ui/async-await/issue-70935-complex-spans.rs#drop_tracking stdout ----

error in revision `drop_tracking`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/issue-70935-complex-spans.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.drop_tracking/auxiliary" "--edition=2018" "-Zdrop-tracking"
stdout: none
stderr: none

---- [ui] tests/ui/async-await/issue-70935-complex-spans.rs#drop_tracking_mir stdout ----


error in revision `drop_tracking_mir`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/issue-70935-complex-spans.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking_mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.drop_tracking_mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.drop_tracking_mir/auxiliary" "--edition=2018" "-Zdrop-tracking-mir"
stdout: none
stderr: none

---- [ui] tests/ui/async-await/issue-70935-complex-spans.rs#no_drop_tracking stdout ----

error in revision `no_drop_tracking`: ui test compiled successfully!
error in revision `no_drop_tracking`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/issue-70935-complex-spans.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "no_drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.no_drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.no_drop_tracking/auxiliary" "--edition=2018"
stdout: none
stderr: none

---- [ui] tests/ui/closures/closure-move-sync.rs stdout ----
diff of stderr:


20 note: required by a bound in `spawn`
21   --> $SRC_DIR/std/src/thread/mod.rs:LL:COL
22 
- error[E0277]: `Sender<()>` cannot be shared between threads safely
-   --> $DIR/closure-move-sync.rs:18:19
-    |
- LL |     thread::spawn(|| tx.send(()).unwrap());
-    |     ------------- ^^^^^^^^^^^^^^^^^^^^^^^ `Sender<()>` cannot be shared between threads safely
-    |     required by a bound introduced by this call
-    |
-    |
-    = help: the trait `Sync` is not implemented for `Sender<()>`
-    = note: required for `&Sender<()>` to implement `Send`
- note: required because it's used within this closure
-   --> $DIR/closure-move-sync.rs:18:19
-    |
- LL |     thread::spawn(|| tx.send(()).unwrap());
- note: required by a bound in `spawn`
-   --> $SRC_DIR/std/src/thread/mod.rs:LL:COL
- 
- error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args closures/closure-move-sync.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/closures/closure-move-sync.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
  --> fake-test-src-base/closures/closure-move-sync.rs:6:27
   |
LL |       let t = thread::spawn(|| {
   |  _____________-------------_^
   | |             required by a bound introduced by this call
   | |             required by a bound introduced by this call
LL | |         recv.recv().unwrap();
LL | |         //~^^ ERROR `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
LL | |     });
   | |_____^ `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
   = help: the trait `Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
   = help: the trait `Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
   = note: required for `&std::sync::mpsc::Receiver<()>` to implement `Send`
note: required because it's used within this closure
  --> fake-test-src-base/closures/closure-move-sync.rs:6:27
   |
LL |     let t = thread::spawn(|| {
note: required by a bound in `spawn`
  --> /rustc/FAKE_PREFIX/library/std/src/thread/mod.rs:680:1

error: aborting due to previous error
---

---- [ui] tests/ui/stdlib-unit-tests/not-sync.rs stdout ----
diff of stderr:

65 LL | fn test<T: Sync>() {}
66    |            ^^^^ required by this bound in `test`
67 
- error[E0277]: `Sender<i32>` cannot be shared between threads safely
-   --> $DIR/not-sync.rs:20:12
-    |
- LL |     test::<Sender<i32>>();
-    |            ^^^^^^^^^^^ `Sender<i32>` cannot be shared between threads safely
-    = help: the trait `Sync` is not implemented for `Sender<i32>`
- note: required by a bound in `test`
-   --> $DIR/not-sync.rs:5:12
-    |
-    |
- LL | fn test<T: Sync>() {}
-    |            ^^^^ required by this bound in `test`
- error: aborting due to 6 previous errors
+ error: aborting due to 5 previous errors
82 
83 For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args stdlib-unit-tests/not-sync.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/stdlib-unit-tests/not-sync.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stdlib-unit-tests/not-sync" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stdlib-unit-tests/not-sync/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `Cell<i32>` cannot be shared between threads safely
  --> fake-test-src-base/stdlib-unit-tests/not-sync.rs:8:12
   |
LL |     test::<Cell<i32>>();
   |            ^^^^^^^^^ `Cell<i32>` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `Cell<i32>`
   = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicI32` instead
note: required by a bound in `test`
  --> fake-test-src-base/stdlib-unit-tests/not-sync.rs:5:12
   |
LL | fn test<T: Sync>() {}
   |            ^^^^ required by this bound in `test`

error[E0277]: `RefCell<i32>` cannot be shared between threads safely
  --> fake-test-src-base/stdlib-unit-tests/not-sync.rs:10:12
   |
LL |     test::<RefCell<i32>>();
   |            ^^^^^^^^^^^^ `RefCell<i32>` cannot be shared between threads safely
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
   = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
note: required by a bound in `test`
  --> fake-test-src-base/stdlib-unit-tests/not-sync.rs:5:12
   |
LL | fn test<T: Sync>() {}
   |            ^^^^ required by this bound in `test`

error[E0277]: `Rc<i32>` cannot be shared between threads safely
  --> fake-test-src-base/stdlib-unit-tests/not-sync.rs:13:12
   |
LL |     test::<Rc<i32>>();
   |            ^^^^^^^ `Rc<i32>` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `Rc<i32>`
note: required by a bound in `test`
  --> fake-test-src-base/stdlib-unit-tests/not-sync.rs:5:12
   |
LL | fn test<T: Sync>() {}
   |            ^^^^ required by this bound in `test`

error[E0277]: `std::rc::Weak<i32>` cannot be shared between threads safely
  --> fake-test-src-base/stdlib-unit-tests/not-sync.rs:15:12
   |
LL |     test::<Weak<i32>>();
   |            ^^^^^^^^^ `std::rc::Weak<i32>` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `std::rc::Weak<i32>`
note: required by a bound in `test`
  --> fake-test-src-base/stdlib-unit-tests/not-sync.rs:5:12
   |
LL | fn test<T: Sync>() {}
   |            ^^^^ required by this bound in `test`

error[E0277]: `std::sync::mpsc::Receiver<i32>` cannot be shared between threads safely
  --> fake-test-src-base/stdlib-unit-tests/not-sync.rs:18:12
   |
LL |     test::<Receiver<i32>>();
Build completed unsuccessfully in 0:11:18
   |            ^^^^^^^^^^^^^ `std::sync::mpsc::Receiver<i32>` cannot be shared between threads safely
   = help: the trait `Sync` is not implemented for `std::sync::mpsc::Receiver<i32>`
note: required by a bound in `test`
  --> fake-test-src-base/stdlib-unit-tests/not-sync.rs:5:12
   |
   |
LL | fn test<T: Sync>() {}
   |            ^^^^ required by this bound in `test`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
