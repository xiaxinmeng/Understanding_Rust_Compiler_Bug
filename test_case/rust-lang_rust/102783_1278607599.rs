plain
........................................................................................ 11792/13670
........................................................................................ 11880/13670
........................................................................................ 11968/13670
...........................................................................i.......i.... 12056/13670
....i......i....................i..........................F...F........................ 12144/13670
........................................................................................ 12320/13670
........................................................................................ 12408/13670
........................................................................................ 12496/13670
........................................................................................ 12584/13670
---
1 error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
-   --> $DIR/issue-43733.rs:21:5
+   --> $DIR/issue-43733.rs:22:5
3    |
4 LL |     __KEY.get(Default::default)

7    = note: consult the function's documentation for information on how to avoid undefined behavior
8 
9 error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
9 error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
-   --> $DIR/issue-43733.rs:26:42
+   --> $DIR/issue-43733.rs:27:42
11    |
12 LL | static FOO: std::thread::LocalKey<Foo> = std::thread::LocalKey::new(__getit);


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir/issue-43733.mir.stderr
To only update this specific test, also pass `--test-args threads-sendsync/issue-43733.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/issue-43733.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |     __KEY.get(Default::default)
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/threads-sendsync/issue-43733.rs:27:42
   |
LL | static FOO: std::thread::LocalKey<Foo> = std::thread::LocalKey::new(__getit);
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to 2 previous errors
---

---- [ui] src/test/ui/threads-sendsync/issue-43733.rs#thir stdout ----
diff of stderr:

1 error[E0133]: call to unsafe function `$LOCALKEYINNER::<T>::get` is unsafe and requires unsafe function or block
-   --> $DIR/issue-43733.rs:21:5
3    |
3    |
4 LL |     __KEY.get(Default::default)

7    = note: consult the function's documentation for information on how to avoid undefined behavior
8 
8 
9 error[E0133]: call to unsafe function `LocalKey::<T>::new` is unsafe and requires unsafe function or block
-   --> $DIR/issue-43733.rs:26:42
11    |
11    |
12 LL | static FOO: std::thread::LocalKey<Foo> = std::thread::LocalKey::new(__getit);


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.thir/issue-43733.thir.stderr
To only update this specific test, also pass `--test-args threads-sendsync/issue-43733.rs`


error in revision `thir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/issue-43733.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.thir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.thir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `__FastLocalKeyInner::<T>::get` is unsafe and requires unsafe function or block
   |
   |
LL |     __KEY.get(Default::default)
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior


error[E0133]: call to unsafe function `LocalKey::<T>::new` is unsafe and requires unsafe function or block
   |
   |
LL | static FOO: std::thread::LocalKey<Foo> = std::thread::LocalKey::new(__getit);
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to 2 previous errors
