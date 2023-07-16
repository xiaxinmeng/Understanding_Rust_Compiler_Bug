plain
failures:

---- [ui] ui/async-await/async-await.rs#thirunsafeck stdout ----

error in revision `thirunsafeck`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zthir-unsafeck" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |         wake_and_yield_once().await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |         wake_and_yield_once().await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |         wake_and_yield_once().await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |         wake_and_yield_once().await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |             wake_and_yield_once().await;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |             wake_and_yield_once().await;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
LL |         future.await
   |         ^^^^^^^^^^^^ call to unsafe function
   |
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
LL |         future.await
   |         ^^^^^^^^^^^^ call to unsafe function
   |
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     wake_and_yield_once().await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     wake_and_yield_once().await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     wake_and_yield_once().await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     wake_and_yield_once().await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     wake_and_yield_once().await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     wake_and_yield_once().await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     wake_and_yield_once().await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     wake_and_yield_once().await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |         wake_and_yield_once().await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |         wake_and_yield_once().await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     wake_and_yield_once().await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     wake_and_yield_once().await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     wake_and_yield_once().await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     wake_and_yield_once().await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |         async_fn_with_borrow_named_lifetime(&y).await
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |         async_fn_with_borrow_named_lifetime(&y).await
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to 24 previous errors

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/command/command-pre-exec.rs#thir stdout ----

error in revision `thir`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/command/command-pre-exec.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/command-pre-exec.thir/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zthir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/command-pre-exec.thir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |             "test1" => println!("hello2"),
   |                        ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |             _ => panic!("unknown argument: {}", arg),
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: this error originates in the macro `$crate::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/no-stdio.rs#thir stdout ----

error in revision `thir`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-stdio.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-stdio.thir/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zthir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-stdio.thir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |         panic!("file descriptor {} is not valid: {}", fd, io::Error::last_os_error());
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: this error originates in the macro `$crate::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |         println!("test");
   |         ^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     assert!(status.success(), "{} isn't a success", status);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: this error originates in the macro `$crate::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     assert!(status.success(), "{} isn't a success", status);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: this error originates in the macro `$crate::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/rfcs/rfc-2396-target_feature-11/closures-inherit-target_feature.rs#thir stdout ----

error in revision `thir`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/closures-inherit-target_feature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/closures-inherit-target_feature.thir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/closures-inherit-target_feature.thir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
LL |     println!("Hello from AVX")
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/running-with-no-runtime.rs#thir stdout ----

error in revision `thir`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/running-with-no-runtime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/running-with-no-runtime.thir/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zthir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/running-with-no-runtime.thir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |         println!("{:?}", str::from_utf8(&output.stdout));
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |         println!("{:?}", str::from_utf8(&output.stderr));
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/threads-sendsync/issue-43733.rs#thir stdout ----
diff of stderr:

14    |
15    = note: consult the function's documentation for information on how to avoid undefined behavior
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+    |
+    |
+ LL |     FOO.with(|foo| println!("{}", foo.borrow()));
+    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
+    |
+    = note: consult the function's documentation for information on how to avoid undefined behavior
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+    |
+    |
+ LL |     FOO.with(|foo| println!("{}", foo.borrow()));
+    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
+    |
+    = note: consult the function's documentation for information on how to avoid undefined behavior
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ error: aborting due to 4 previous errors
18 
19 For more information about this error, try `rustc --explain E0133`.
20 
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.thir/issue-43733.thir.stderr
To only update this specific test, also pass `--test-args threads-sendsync/issue-43733.rs`


error in revision `thir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/issue-43733.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.thir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.thir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     __KEY.get(Default::default) //~ ERROR call to unsafe function is unsafe
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL | static FOO: std::thread::LocalKey<Foo> = std::thread::LocalKey::new(__getit);
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     FOO.with(|foo| println!("{}", foo.borrow()));
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     FOO.with(|foo| println!("{}", foo.borrow()));
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/unsafe/unsafe-around-compiler-generated-unsafe.rs#thir stdout ----

error in revision `thir`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-around-compiler-generated-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-around-compiler-generated-unsafe.thir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-around-compiler-generated-unsafe.thir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
test result: FAILED. 11862 passed; 7 failed; 101 ignored; 0 measured; 0 filtered out; finished in 117.99s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:54
