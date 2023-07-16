plain

---- [ui] src/test/ui/threads-sendsync/issue-43733.rs#mir stdout ----
diff of stderr:

- error[E0133]: call to unsafe function `std::thread::__FastLocalKeyInner::<T>::get` is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `std::thread::__OsLocalKeyInner::<T>::get` is unsafe and requires unsafe function or block
3    |
3    |
4 LL |     __KEY.get(Default::default)

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir/issue-43733.mir.stderr
To only update this specific test, also pass `--test-args threads-sendsync/issue-43733.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/issue-43733.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `std::thread::__OsLocalKeyInner::<T>::get` is unsafe and requires unsafe function or block
   |
   |
LL |     __KEY.get(Default::default)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function `std::thread::LocalKey::<T>::new` is unsafe and requires unsafe function or block
   |
   |
LL | static FOO: std::thread::LocalKey<Foo> = std::thread::LocalKey::new(__getit);
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/threads-sendsync/issue-43733.rs#thir stdout ----
diff of stderr:

- error[E0133]: call to unsafe function `__FastLocalKeyInner::<T>::get` is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `__OsLocalKeyInner::<T>::get` is unsafe and requires unsafe function or block
3    |
3    |
4 LL |     __KEY.get(Default::default)

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.thir/issue-43733.thir.stderr
To only update this specific test, also pass `--test-args threads-sendsync/issue-43733.rs`


error in revision `thir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/issue-43733.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--cfg" "thir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.thir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.thir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `__OsLocalKeyInner::<T>::get` is unsafe and requires unsafe function or block
   |
   |
LL |     __KEY.get(Default::default)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function `LocalKey::<T>::new` is unsafe and requires unsafe function or block
   |
   |
LL | static FOO: std::thread::LocalKey<Foo> = std::thread::LocalKey::new(__getit);
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
