plain
1 error[E0277]: the trait bound `String: Copy` is not satisfied
-   --> $DIR/const-fn-in-vec.rs:3:32
+   --> $DIR/const-fn-in-vec.rs:4:32
3    |
4 LL |     let strings: [String; 5] = [String::new(); 5];
5    |                                ^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `String`
6    |
6    |
+    = note: the `Copy` trait is required because the repeated element will be copied
7    = help: moving the function call to a new `const` item will resolve the error
9 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-in-vec/const-fn-in-vec.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-fn-in-vec.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-fn-in-vec.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-in-vec" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-in-vec/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `String: Copy` is not satisfied
  --> /checkout/src/test/ui/consts/const-fn-in-vec.rs:4:32
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL |     let strings: [String; 5] = [String::new(); 5];
   |                                ^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `String`
   |
   = note: the `Copy` trait is required because the repeated element will be copied
   = help: moving the function call to a new `const` item will resolve the error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const.rs stdout ----
diff of stderr:

7    = help: the following implementations were found:
8              <Option<T> as Copy>
9    = note: the `Copy` trait is required because the repeated element will be copied
+    = help: moving the function call to a new `const` item will resolve the error
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const/fn-call-in-non-const.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Option<Bar>: Copy` is not satisfied
   |
   |
LL |     let _: [Option<Bar>; 2] = [no_copy(); 2];
   |                               ^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `Option<Bar>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <Option<T> as Copy>
   = note: the `Copy` trait is required because the repeated element will be copied
   = help: moving the function call to a new `const` item will resolve the error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-const_in_array_repeat_expressions.rs stdout ----
diff of stderr:

7    = help: the following implementations were found:
8              <Option<T> as Copy>
9    = note: the `Copy` trait is required because the repeated element will be copied
-    = note: this array initializer can be evaluated at compile-time, see issue #49147 <https://github.com/rust-lang/rust/issues/49147> for more information
12 
12 
13 error[E0277]: the trait bound `Option<String>: Copy` is not satisfied


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_in_array_repeat_expressions/feature-gate-const_in_array_repeat_expressions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_in_array_repeat_expressions/feature-gate-const_in_array_repeat_expressions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-const_in_array_repeat_expressions.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-const_in_array_repeat_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_in_array_repeat_expressions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_in_array_repeat_expressions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Option<String>: Copy` is not satisfied
   |
   |
LL |     let arr: [Option<String>; 2] = [None::<String>; 2];
   |                                    ^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `Option<String>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <Option<T> as Copy>
   = note: the `Copy` trait is required because the repeated element will be copied

error[E0277]: the trait bound `Option<String>: Copy` is not satisfied
   |
   |
LL |     let arr: [Option<String>; 2] = [Some("foo".to_string()); 2];
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `Option<String>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <Option<T> as Copy>
   = note: the `Copy` trait is required because the repeated element will be copied
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 11221 passed; 3 failed; 88 ignored; 0 measured; 0 filtered out; finished in 113.89s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:40
