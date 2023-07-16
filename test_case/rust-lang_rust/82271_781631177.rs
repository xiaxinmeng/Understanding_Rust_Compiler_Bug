plain
.................................................................................................... 7300/11462
.................................................................................................... 7400/11462
.................................................................................................... 7500/11462
..........................................................................................i....ii... 7600/11462
..........F.FF..F.........................................ii........................................ 7700/11462
.............................................................i...................................... 7900/11462
......................................................i............................................. 8000/11462
.................................................................................................... 8100/11462
.................................................................................................... 8200/11462
---
.................................................................................................... 9200/11462
.................................................................................................... 9300/11462
.................................................................................................... 9400/11462
....................i......i........................................................................ 9500/11462
...........................................................iiiiiii..iiiiii.i........................ 9600/11462
.................................................................................................... 9800/11462
.................................................................................................... 9900/11462
.................................................................................................... 10000/11462
.................................................................................................... 10100/11462
---

---- [ui] ui/not-panic/not-panic-safe-2.rs stdout ----
diff of stderr:

25    = note: required because it appears within the type `RefCell<i32>`
26    = note: required because of the requirements on the impl of `UnwindSafe` for `Rc<RefCell<i32>>`
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0277]: the type `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |
+    |
+ LL | fn assert<T: UnwindSafe + ?Sized>() {}
+    |              ---------- required by this bound in `assert`
+ ...
+ LL |     assert::<Rc<RefCell<i32>>>();
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |
+    = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<Option<&'static Location<'static>>>`
+    = note: required because it appears within the type `Cell<Option<&'static Location<'static>>>`
+    = note: required because it appears within the type `RefCell<i32>`
+    = note: required because of the requirements on the impl of `UnwindSafe` for `Rc<RefCell<i32>>`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ error: aborting due to 3 previous errors
29 
30 For more information about this error, try `rustc --explain E0277`.
30 For more information about this error, try `rustc --explain E0277`.
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-2/not-panic-safe-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args not-panic/not-panic-safe-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/not-panic/not-panic-safe-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<Rc<RefCell<i32>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `Rc<RefCell<i32>>`

error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<Rc<RefCell<i32>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<isize>`
   = note: required because it appears within the type `Cell<isize>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `Rc<RefCell<i32>>`

error[E0277]: the type `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<Rc<RefCell<i32>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<Option<&'static Location<'static>>>`
   = note: required because it appears within the type `Cell<Option<&'static Location<'static>>>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `Rc<RefCell<i32>>`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/not-panic/not-panic-safe-4.rs stdout ----
diff of stderr:

25    = note: required because it appears within the type `RefCell<i32>`
26    = note: required because of the requirements on the impl of `UnwindSafe` for `&RefCell<i32>`
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0277]: the type `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |
+    |
+ LL | fn assert<T: UnwindSafe + ?Sized>() {}
+    |              ---------- required by this bound in `assert`
+ ...
+ LL |     assert::<&RefCell<i32>>();
+    |     ^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |
+    = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<Option<&'static Location<'static>>>`
+    = note: required because it appears within the type `Cell<Option<&'static Location<'static>>>`
+    = note: required because it appears within the type `RefCell<i32>`
+    = note: required because of the requirements on the impl of `UnwindSafe` for `&RefCell<i32>`
+ error: aborting due to 3 previous errors
29 
30 For more information about this error, try `rustc --explain E0277`.
31 
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-4/not-panic-safe-4.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args not-panic/not-panic-safe-4.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/not-panic/not-panic-safe-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<&RefCell<i32>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `&RefCell<i32>`

error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<&RefCell<i32>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<isize>`
   = note: required because it appears within the type `Cell<isize>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `&RefCell<i32>`

error[E0277]: the type `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<&RefCell<i32>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<Option<&'static Location<'static>>>`
   = note: required because it appears within the type `Cell<Option<&'static Location<'static>>>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `&RefCell<i32>`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/not-panic/not-panic-safe-3.rs stdout ----
diff of stderr:

25    = note: required because it appears within the type `RefCell<i32>`
26    = note: required because of the requirements on the impl of `UnwindSafe` for `Arc<RefCell<i32>>`
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0277]: the type `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |
+    |
+ LL | fn assert<T: UnwindSafe + ?Sized>() {}
+    |              ---------- required by this bound in `assert`
+ ...
+ LL |     assert::<Arc<RefCell<i32>>>();
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |
+    = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<Option<&'static Location<'static>>>`
+    = note: required because it appears within the type `Cell<Option<&'static Location<'static>>>`
+    = note: required because it appears within the type `RefCell<i32>`
+    = note: required because of the requirements on the impl of `UnwindSafe` for `Arc<RefCell<i32>>`
+ error: aborting due to 3 previous errors
29 
30 For more information about this error, try `rustc --explain E0277`.
31 
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-3/not-panic-safe-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args not-panic/not-panic-safe-3.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/not-panic/not-panic-safe-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<Arc<RefCell<i32>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `Arc<RefCell<i32>>`

error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<Arc<RefCell<i32>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<isize>`
   = note: required because it appears within the type `Cell<isize>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `Arc<RefCell<i32>>`

error[E0277]: the type `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<Arc<RefCell<i32>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<Option<&'static Location<'static>>>`
   = note: required because it appears within the type `Cell<Option<&'static Location<'static>>>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `Arc<RefCell<i32>>`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/not-panic/not-panic-safe-6.rs stdout ----
diff of stderr:

25    = note: required because it appears within the type `RefCell<i32>`
26    = note: required because of the requirements on the impl of `UnwindSafe` for `*mut RefCell<i32>`
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0277]: the type `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |
+    |
+ LL | fn assert<T: UnwindSafe + ?Sized>() {}
+    |              ---------- required by this bound in `assert`
+ ...
+ LL |     assert::<*mut RefCell<i32>>();
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |
+    = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<Option<&'static Location<'static>>>`
+    = note: required because it appears within the type `Cell<Option<&'static Location<'static>>>`
+    = note: required because it appears within the type `RefCell<i32>`
+    = note: required because of the requirements on the impl of `UnwindSafe` for `*mut RefCell<i32>`
+ error: aborting due to 3 previous errors
29 
30 For more information about this error, try `rustc --explain E0277`.
31 
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-6/not-panic-safe-6.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args not-panic/not-panic-safe-6.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/not-panic/not-panic-safe-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-6" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-6/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<*mut RefCell<i32>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `*mut RefCell<i32>`

error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<*mut RefCell<i32>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<isize>`
   = note: required because it appears within the type `Cell<isize>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `*mut RefCell<i32>`

error[E0277]: the type `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<*mut RefCell<i32>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<Option<&'static Location<'static>>>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<Option<&'static Location<'static>>>`
   = note: required because it appears within the type `Cell<Option<&'static Location<'static>>>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `*mut RefCell<i32>`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 11365 passed; 4 failed; 93 ignored; 0 measured; 0 filtered out; finished in 134.54s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:15
