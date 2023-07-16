plain
.................................................................................................... 7400/11225
...............................................i...ii............................................... 7500/11225
...........ii....................................................................................... 7600/11225
................................i................................................................... 7700/11225
....F............i..F............................................................................... 7800/11225
.................................................................................................... 8000/11225
.................................................................i.................................. 8100/11225
.................................................................................................... 8200/11225
.....................................................................i.............................. 8300/11225
---
.................................................................................................... 9000/11225
.................................................................................................... 9100/11225
.................................................................................................... 9200/11225
............i......i................................................................................ 9300/11225
...................................................iiiiii..iiiiii.i................................. 9400/11225
.................................................................................................... 9600/11225
.................................................................................................... 9700/11225
.................................................................................................... 9800/11225
.................................................................................................... 9900/11225
---
diff of stderr:

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = note: candidates:
-            crate `crateresolve1`: \?\$TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/crateresolve1-1.dll
-            crate `crateresolve1`: \?\$TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/crateresolve1-2.dll
-            crate `crateresolve1`: \?\$TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/crateresolve1-3.dll
+            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-3.so
+            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-1.so
+            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-2.so
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/crateresolve1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crate-loading/crateresolve1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/crateresolve1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0464]: multiple matching crates for `crateresolve1`
  --> /checkout/src/test/ui/crate-loading/crateresolve1.rs:6:1
   |
LL | extern crate crateresolve1;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary/libcrateresolve1-3.so
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary/libcrateresolve1-1.so
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary/libcrateresolve1-2.so
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/linkage-attr/issue-10755.rs stdout ----
diff of stderr:

1 error: linker `llllll` not found
2    |
-    = note: The system cannot find the file specified. (os error 2)
+    = note: No such file or directory (os error 2)
5 error: aborting due to previous error
6 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/issue-10755/issue-10755.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args linkage-attr/issue-10755.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/linkage-attr/issue-10755.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/issue-10755" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "linker=llllll" "-C" "linker-flavor=ld" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/issue-10755/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linker `llllll` not found
   |
   = note: No such file or directory (os error 2)
error: aborting due to previous error

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


------------------------------------------


---- [ui] ui/linkage-attr/invalid-link-args.rs stdout ----
diff of stderr:

1 error: linking with `ld` failed: exit code: 1
2    |
-    = note: "ld" "C:/msys64/home/we/rust/build/x86_64-pc-windows-gnu/stage1/lib/rustlib/x86_64-pc-windows-gnu/lib/rsbegin.o" "-L" "C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\stage1\lib\rustlib\x86_64-pc-windows-gnu\lib" "C:/msys64/home/we/rust/build/x86_64-pc-windows-gnu/test/ui/linkage-attr/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.0.rcgu.o" "C:/msys64/home/we/rust/build/x86_64-pc-windows-gnu/test/ui/linkage-attr/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.1.rcgu.o" "C:/msys64/home/we/rust/build/x86_64-pc-windows-gnu/test/ui/linkage-attr/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.2.rcgu.o" "-o" "C:/msys64/home/we/rust/build/x86_64-pc-windows-gnu/test/ui/linkage-attr/invalid-link-args/invalid-link-args.exe" "--gc-sections" "-L" "C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\native\rust-test-helpers" "-L" "C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\test\ui\linkage-attr\invalid-link-args\auxiliary" "-L" "C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\stage1\lib\rustlib\x86_64-pc-windows-gnu\lib" "--start-group" "-L" "C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\stage1\lib\rustlib\x86_64-pc-windows-gnu\lib" "-lstd-b7455dbbc80bbdae" "--end-group" "-Bstatic" "C:/msys64/home/we/rust/build/x86_64-pc-windows-gnu/stage1/lib/rustlib/x86_64-pc-windows-gnu/lib/libcompiler_builtins-bd8decb4ebfd577a.rlib" "-Bdynamic" "-ladvapi32" "-lws2_32" "-luserenv" "-fuse-ld=lld" "aFdEfSeVEEE" "C:/msys64/home/we/rust/build/x86_64-pc-windows-gnu/stage1/lib/rustlib/x86_64-pc-windows-gnu/lib/rsend.o"
-    = note: C:/msys64/mingw64/bin/ld.exe: cannot find -ladvapi32
-            C:/msys64/mingw64/bin/ld.exe: cannot find -lws2_32
-            C:/msys64/mingw64/bin/ld.exe: cannot find -luserenv
-            C:/msys64/mingw64/bin/ld.exe: cannot find aFdEfSeVEEE: No such file or directory
+    = note: "ld" "--eh-frame-hdr" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "$TEST_BUILD_DIR/linkage-attr/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.0.rcgu.o" "$TEST_BUILD_DIR/linkage-attr/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.1.rcgu.o" "$TEST_BUILD_DIR/linkage-attr/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.2.rcgu.o" "-o" "$TEST_BUILD_DIR/linkage-attr/invalid-link-args/invalid-link-args" "--gc-sections" "-pie" "-zrelro" "-znow" "-O1" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "$TEST_BUILD_DIR/linkage-attr/invalid-link-args/auxiliary" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--start-group" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-05207fe44e4b40c3" "--end-group" "-Bstatic" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-3e915a944a8f578e.rlib" "-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,$LIB_DIR/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "aFdEfSeVEEE"
+    = note: ld: unrecognized option '-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib'
+            ld: use the --help option for usage information
9 
10 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/invalid-link-args/invalid-link-args.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args linkage-attr/invalid-link-args.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/linkage-attr/invalid-link-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/invalid-link-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "linker-flavor=ld" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/invalid-link-args/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `ld` failed: exit code: 1
   |
   = note: "ld" "--eh-frame-hdr" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/invalid-link-args/invalid-link-args.invalid_link_args.7rcbfp3g-cgu.2.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/invalid-link-args/invalid-link-args" "--gc-sections" "-pie" "-zrelro" "-znow" "-O1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/invalid-link-args/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-05207fe44e4b40c3" "--end-group" "-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-3e915a944a8f578e.rlib" "-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "aFdEfSeVEEE"
   = note: ld: unrecognized option '-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib'
           ld: use the --help option for usage information

error: aborting due to previous error


---
diff of stderr:

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = note: the lang item is first defined in crate `some_panic_impl` (which `panic_handler_twice` depends on)
-    = note: first definition in `some_panic_impl` loaded from \?\$TEST_BUILD_DIR/panic-handler/panic-handler-twice/auxiliary/libsome_panic_impl.rlib
+    = note: first definition in `some_panic_impl` loaded from $TEST_BUILD_DIR/panic-handler/panic-handler-twice/auxiliary/libsome_panic_impl.rlib
9    = note: second definition in the local crate (`panic_handler_twice`)
11 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-twice/panic-handler-twice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panic-handler/panic-handler-twice.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-handler/panic-handler-twice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-twice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-twice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0152]: found duplicate lang item `panic_impl`
   |
   |
LL | fn panic(info: &PanicInfo) -> ! {
   |
   |
   = note: the lang item is first defined in crate `some_panic_impl` (which `panic_handler_twice` depends on)
   = note: first definition in `some_panic_impl` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-twice/auxiliary/libsome_panic_impl.rlib
   = note: second definition in the local crate (`panic_handler_twice`)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0152`.


------------------------------------------


---- [ui] ui/panic-runtime/unwind-tables-panic-required.rs stdout ----
diff of stderr:

1 error: panic=unwind requires unwind tables, they cannot be disabled with `-C force-unwind-tables=no`.
2 
- error: target requires unwind tables, they cannot be disabled with `-C force-unwind-tables=no`.
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
6 
7 
7 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/unwind-tables-panic-required/unwind-tables-panic-required.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panic-runtime/unwind-tables-panic-required.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/unwind-tables-panic-required.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/unwind-tables-panic-required" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=unwind" "-C" "force-unwind-tables=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/unwind-tables-panic-required/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: panic=unwind requires unwind tables, they cannot be disabled with `-C force-unwind-tables=no`.
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/threads-sendsync/issue-43733-2.rs stdout ----


- error[E0277]: `UnsafeCell<Option<()>>` cannot be shared between threads safely
+ error[E0277]: `Cell<std::thread::local::fast::DtorState>` cannot be shared between threads safely
3    |
3    |
4 LL | static __KEY: Key<()> = Key::new();

-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<Option<()>>` cannot be shared between threads safely
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Cell<std::thread::local::fast::DtorState>` cannot be shared between threads safely
6    |
-    = help: within `Key<()>`, the trait `Sync` is not implemented for `UnsafeCell<Option<()>>`
-    = note: required because it appears within the type `Key<()>`
+    = help: within `__FastLocalKeyInner<()>`, the trait `Sync` is not implemented for `Cell<std::thread::local::fast::DtorState>`
+    = note: required because it appears within the type `__FastLocalKeyInner<()>`
9    = note: shared static variables must have a type that implements `Sync`
10 
- error[E0277]: `Cell<()>` cannot be shared between threads safely
+ error[E0277]: `UnsafeCell<Option<()>>` cannot be shared between threads safely
13    |
13    |
14 LL | static __KEY: Key<()> = Key::new();

-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Cell<()>` cannot be shared between threads safely
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<Option<()>>` cannot be shared between threads safely
16    |
-    = help: within `Key<()>`, the trait `Sync` is not implemented for `Cell<()>`
-    = note: required because it appears within the type `Key<()>`
+    = help: within `__FastLocalKeyInner<()>`, the trait `Sync` is not implemented for `UnsafeCell<Option<()>>`
+    = note: required because it appears within the type `std::thread::local::lazy::LazyKeyInner<()>`
+    = note: required because it appears within the type `__FastLocalKeyInner<()>`
19    = note: shared static variables must have a type that implements `Sync`
21 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733-2/issue-43733-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args threads-sendsync/issue-43733-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/issue-43733-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `Cell<std::thread::local::fast::DtorState>` cannot be shared between threads safely
  --> /checkout/src/test/ui/threads-sendsync/issue-43733-2.rs:24:1
   |
LL | static __KEY: Key<()> = Key::new();
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Cell<std::thread::local::fast::DtorState>` cannot be shared between threads safely
   |
   = help: within `__FastLocalKeyInner<()>`, the trait `Sync` is not implemented for `Cell<std::thread::local::fast::DtorState>`
   = note: required because it appears within the type `__FastLocalKeyInner<()>`
   = note: shared static variables must have a type that implements `Sync`

error[E0277]: `UnsafeCell<Option<()>>` cannot be shared between threads safely
  --> /checkout/src/test/ui/threads-sendsync/issue-43733-2.rs:24:1
   |
LL | static __KEY: Key<()> = Key::new();
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<Option<()>>` cannot be shared between threads safely
   |
   = help: within `__FastLocalKeyInner<()>`, the trait `Sync` is not implemented for `UnsafeCell<Option<()>>`
   = note: required because it appears within the type `std::thread::local::lazy::LazyKeyInner<()>`
   = note: required because it appears within the type `__FastLocalKeyInner<()>`
   = note: shared static variables must have a type that implements `Sync`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 11134 passed; 6 failed; 85 ignored; 0 measured; 0 filtered out; finished in 151.12s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:28
