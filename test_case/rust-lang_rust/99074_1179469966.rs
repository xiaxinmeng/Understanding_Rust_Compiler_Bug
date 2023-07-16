plain

---- [ui] src/test/ui/allocator/not-an-allocator.rs stdout ----
diff of stderr:

6 LL | static A: usize = 0;
7    |           ^^^^^ the trait `GlobalAlloc` is not implemented for `usize`
8    |
-    = help: the trait `GlobalAlloc` is implemented for `System`
+    = help: the following other types implement trait `GlobalAlloc`:
+              std::sys::unix::alloc::System
+              std::sys::unix::alloc::System
10    = note: this error originates in the attribute macro `global_allocator` (in Nightly builds, run with -Z macro-backtrace for more info)
12 error[E0277]: the trait bound `usize: GlobalAlloc` is not satisfied


17 LL | static A: usize = 0;
18    |           ^^^^^ the trait `GlobalAlloc` is not implemented for `usize`
19    |
-    = help: the trait `GlobalAlloc` is implemented for `System`
+    = help: the following other types implement trait `GlobalAlloc`:
+              std::sys::unix::alloc::System
+              std::sys::unix::alloc::System
21    = note: this error originates in the attribute macro `global_allocator` (in Nightly builds, run with -Z macro-backtrace for more info)
23 error[E0277]: the trait bound `usize: GlobalAlloc` is not satisfied


28 LL | static A: usize = 0;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
29    |           ^^^^^ the trait `GlobalAlloc` is not implemented for `usize`
30    |
-    = help: the trait `GlobalAlloc` is implemented for `System`
+    = help: the following other types implement trait `GlobalAlloc`:
+              std::sys::unix::alloc::System
+              std::sys::unix::alloc::System
32    = note: this error originates in the attribute macro `global_allocator` (in Nightly builds, run with -Z macro-backtrace for more info)
34 error[E0277]: the trait bound `usize: GlobalAlloc` is not satisfied


39 LL | static A: usize = 0;
40    |           ^^^^^ the trait `GlobalAlloc` is not implemented for `usize`
41    |
-    = help: the trait `GlobalAlloc` is implemented for `System`
+    = help: the following other types implement trait `GlobalAlloc`:
+              std::sys::unix::alloc::System
+              std::sys::unix::alloc::System
43    = note: this error originates in the attribute macro `global_allocator` (in Nightly builds, run with -Z macro-backtrace for more info)
45 error: aborting due to 4 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/not-an-allocator/not-an-allocator.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args allocator/not-an-allocator.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/not-an-allocator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/not-an-allocator" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/not-an-allocator/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `usize: GlobalAlloc` is not satisfied
   |
   |
LL | #[global_allocator]
   | ------------------- in this procedural macro expansion
LL | static A: usize = 0;
   |           ^^^^^ the trait `GlobalAlloc` is not implemented for `usize`
   = help: the following other types implement trait `GlobalAlloc`:
             System
             std::sys::unix::alloc::System
             std::sys::unix::alloc::System
   = note: this error originates in the attribute macro `global_allocator` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0277]: the trait bound `usize: GlobalAlloc` is not satisfied
  --> /checkout/src/test/ui/allocator/not-an-allocator.rs:2:11
   |
   |
LL | #[global_allocator]
   | ------------------- in this procedural macro expansion
LL | static A: usize = 0;
   |           ^^^^^ the trait `GlobalAlloc` is not implemented for `usize`
   = help: the following other types implement trait `GlobalAlloc`:
             System
             std::sys::unix::alloc::System
             std::sys::unix::alloc::System
   = note: this error originates in the attribute macro `global_allocator` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0277]: the trait bound `usize: GlobalAlloc` is not satisfied
  --> /checkout/src/test/ui/allocator/not-an-allocator.rs:2:11
   |
   |
LL | #[global_allocator]
   | ------------------- in this procedural macro expansion
LL | static A: usize = 0;
   |           ^^^^^ the trait `GlobalAlloc` is not implemented for `usize`
   = help: the following other types implement trait `GlobalAlloc`:
             System
             std::sys::unix::alloc::System
             std::sys::unix::alloc::System
   = note: this error originates in the attribute macro `global_allocator` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0277]: the trait bound `usize: GlobalAlloc` is not satisfied
  --> /checkout/src/test/ui/allocator/not-an-allocator.rs:2:11
   |
   |
LL | #[global_allocator]
   | ------------------- in this procedural macro expansion
LL | static A: usize = 0;
   |           ^^^^^ the trait `GlobalAlloc` is not implemented for `usize`
   = help: the following other types implement trait `GlobalAlloc`:
             System
             std::sys::unix::alloc::System
             std::sys::unix::alloc::System
   = note: this error originates in the attribute macro `global_allocator` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
