plain

30 error[E0277]: the size for values of type `dyn Fn()` cannot be known at compilation time
31   --> $DIR/issue-84973-blacklist.rs:22:13
32    |
- LL | fn f_sized<T: Sized>(t: T) {}
-    |            - required by this bound in `f_sized`
- ...
36 LL |     f_sized(*ref_cl);
38    |

39    = help: the trait `Sized` is not implemented for `dyn Fn()`
39    = help: the trait `Sized` is not implemented for `dyn Fn()`
+ note: type parameters introduce an implicit `Sized` obligation
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
+    |
+ LL | fn f_sized<T: Sized>(t: T) {}
+    |            ^ required by this bound in `f_sized`
40 
41 error[E0277]: `Rc<{integer}>` cannot be sent between threads safely


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-84973-blacklist/issue-84973-blacklist.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-84973-blacklist/issue-84973-blacklist.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-84973-blacklist.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-84973-blacklist.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-84973-blacklist" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-84973-blacklist/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `String: Copy` is not satisfied
  --> /checkout/src/test/ui/suggestions/issue-84973-blacklist.rs:15:12
   |
LL | fn f_copy<T: Copy>(t: T) {}
   |              ---- required by this bound in `f_copy`
...
LL |     f_copy("".to_string()); //~ ERROR: the trait bound `String: Copy` is not satisfied [E0277]
   |            ^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `String`

error[E0277]: the trait bound `S: Clone` is not satisfied
   |
   |
LL | fn f_clone<T: Clone>(t: T) {}
   |               ----- required by this bound in `f_clone`
...
LL |     f_clone(S); //~ ERROR: the trait bound `S: Clone` is not satisfied [E0277]
   |             ^ the trait `Clone` is not implemented for `S`

error[E0277]: `[static generator@/checkout/src/test/ui/suggestions/issue-84973-blacklist.rs:17:13: 17:33]` cannot be unpinned
   |
   |
LL | fn f_unpin<T: Unpin>(t: T) {}
   |               ----- required by this bound in `f_unpin`
...
LL |     f_unpin(static || { yield; });
   |     ^^^^^^^ the trait `Unpin` is not implemented for `[static generator@/checkout/src/test/ui/suggestions/issue-84973-blacklist.rs:17:13: 17:33]`
   |
   = note: consider using `Box::pin`
error[E0277]: the size for values of type `dyn Fn()` cannot be known at compilation time
  --> /checkout/src/test/ui/suggestions/issue-84973-blacklist.rs:22:13
   |
   |
LL |     f_sized(*ref_cl);
   |
   = help: the trait `Sized` is not implemented for `dyn Fn()`
note: type parameters introduce an implicit `Sized` obligation
  --> /checkout/src/test/ui/suggestions/issue-84973-blacklist.rs:9:12
  --> /checkout/src/test/ui/suggestions/issue-84973-blacklist.rs:9:12
   |
LL | fn f_sized<T: Sized>(t: T) {}
   |            ^ required by this bound in `f_sized`

error[E0277]: `Rc<{integer}>` cannot be sent between threads safely
   |
   |
LL | fn f_send<T: Send>(t: T) {}
   |              ---- required by this bound in `f_send`
...
LL |     f_send(rc); //~ ERROR: `Rc<{integer}>` cannot be sent between threads safely [E0277]
   |            ^^ `Rc<{integer}>` cannot be sent between threads safely
   |
   = help: the trait `Send` is not implemented for `Rc<{integer}>`
error[E0277]: the size for values of type `dyn Fn()` cannot be known at compilation time
  --> /checkout/src/test/ui/suggestions/issue-84973-blacklist.rs:22:5
   |
   |
LL |     f_sized(*ref_cl);
   |
   = help: the trait `Sized` is not implemented for `dyn Fn()`
   = note: all function arguments must have a statically known size
   = help: unsized fn params are gated as an unstable feature
---
test result: FAILED. 11841 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 117.65s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:54
