plain
.................................................................................................... 9400/11720
.................................................................................................... 9500/11720
..............................................................i......i.............................. 9600/11720
.................................................................................................... 9700/11720
........iiiiiii..iiiiii.i........................................................................... 9800/11720
.................................................................................................... 10000/11720
.................................................................................................... 10100/11720
.................................................................................................... 10200/11720
.................................................................................................... 10300/11720
---

---- [ui] ui/anonymous-higher-ranked-lifetime.rs stdout ----
diff of stderr:

114 LL |     h2(|_: (), _: (), _: (), _: ()| {});
115    |     ^^ ---------------------------- found signature of `fn((), (), (), ()) -> _`
116    |     |
-    |     expected signature of `for<'r, 't0> fn(&'r (), Box<(dyn for<'s> Fn(&'s ()) + 'static)>, &'t0 (), for<'s, 't1> fn(&'s (), &'t1 ())) -> _`
+    |     expected signature of `for<'r, 't0> fn(&'r (), Box<(dyn for<'s> Fn(&'s ()) + 'static)>, &'t0 (), for<'s, 't0> fn(&'s (), &'t0 ())) -> _`
118 ...
119 LL | fn h2<F>(_: F) where F: for<'t0> Fn(&(), Box<dyn Fn(&())>, &'t0 (), fn(&(), &())) {}
120    |                         --------------------------------------------------------- required by this bound in `h2`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anonymous-higher-ranked-lifetime/anonymous-higher-ranked-lifetime.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anonymous-higher-ranked-lifetime/anonymous-higher-ranked-lifetime.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args anonymous-higher-ranked-lifetime.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/anonymous-higher-ranked-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anonymous-higher-ranked-lifetime" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anonymous-higher-ranked-lifetime/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/anonymous-higher-ranked-lifetime.rs:2:5
   |
LL |     f1(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature of `fn((), ()) -> _`
   |     |
   |     expected signature of `for<'r, 's> fn(&'r (), &'s ()) -> _`
...
LL | fn f1<F>(_: F) where F: Fn(&(), &()) {}
   |                         ------------ required by this bound in `f1`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/anonymous-higher-ranked-lifetime.rs:3:5
   |
   |
LL |     f2(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature of `fn((), ()) -> _`
   |     |
   |     expected signature of `for<'a, 'r> fn(&'a (), &'r ()) -> _`
...
LL | fn f2<F>(_: F) where F: for<'a> Fn(&'a (), &()) {}
   |                         ----------------------- required by this bound in `f2`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/anonymous-higher-ranked-lifetime.rs:4:5
   |
   |
LL |     f3(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature of `fn((), ()) -> _`
   |     |
   |     expected signature of `for<'r> fn(&(), &'r ()) -> _`
...
LL | fn f3<'a, F>(_: F) where F: Fn(&'a (), &()) {}
   |                             --------------- required by this bound in `f3`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/anonymous-higher-ranked-lifetime.rs:5:5
   |
   |
LL |     f4(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature of `fn((), ()) -> _`
   |     |
   |     expected signature of `for<'s, 'r> fn(&'s (), &'r ()) -> _`
...
LL | fn f4<F>(_: F) where F: for<'r> Fn(&(), &'r ()) {}
   |                         ----------------------- required by this bound in `f4`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/anonymous-higher-ranked-lifetime.rs:6:5
   |
   |
LL |     f5(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature of `fn((), ()) -> _`
   |     |
   |     expected signature of `for<'r> fn(&'r (), &'r ()) -> _`
...
LL | fn f5<F>(_: F) where F: for<'r> Fn(&'r (), &'r ()) {}
   |                         -------------------------- required by this bound in `f5`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/anonymous-higher-ranked-lifetime.rs:7:5
   |
   |
LL |     g1(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature of `fn((), ()) -> _`
   |     |
   |     expected signature of `for<'r> fn(&'r (), Box<(dyn for<'s> Fn(&'s ()) + 'static)>) -> _`
...
LL | fn g1<F>(_: F) where F: Fn(&(), Box<dyn Fn(&())>) {}
   |                         ------------------------- required by this bound in `g1`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/anonymous-higher-ranked-lifetime.rs:8:5
   |
   |
LL |     g2(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature of `fn((), ()) -> _`
   |     |
   |     expected signature of `for<'r> fn(&'r (), for<'s> fn(&'s ())) -> _`
...
LL | fn g2<F>(_: F) where F: Fn(&(), fn(&())) {}
   |                         ---------------- required by this bound in `g2`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/anonymous-higher-ranked-lifetime.rs:9:5
   |
   |
LL |     g3(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature of `fn((), ()) -> _`
   |     |
   |     expected signature of `for<'s> fn(&'s (), Box<(dyn for<'r> Fn(&'r ()) + 'static)>) -> _`
...
LL | fn g3<F>(_: F) where F: for<'s> Fn(&'s (), Box<dyn Fn(&())>) {}
   |                         ------------------------------------ required by this bound in `g3`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/anonymous-higher-ranked-lifetime.rs:10:5
   |
   |
LL |     g4(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature of `fn((), ()) -> _`
   |     |
   |     expected signature of `for<'s> fn(&'s (), for<'r> fn(&'r ())) -> _`
...
LL | fn g4<F>(_: F) where F: Fn(&(), for<'r> fn(&'r ())) {}
   |                         --------------------------- required by this bound in `g4`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/anonymous-higher-ranked-lifetime.rs:11:5
   |
   |
LL |     h1(|_: (), _: (), _: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ ---------------------------- found signature of `fn((), (), (), ()) -> _`
   |     |
   |     expected signature of `for<'r, 's> fn(&'r (), Box<(dyn for<'t0> Fn(&'t0 ()) + 'static)>, &'s (), for<'t0, 't1> fn(&'t0 (), &'t1 ())) -> _`
...
LL | fn h1<F>(_: F) where F: Fn(&(), Box<dyn Fn(&())>, &(), fn(&(), &())) {}
   |                         -------------------------------------------- required by this bound in `h1`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/anonymous-higher-ranked-lifetime.rs:12:5
   |
   |
LL |     h2(|_: (), _: (), _: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ ---------------------------- found signature of `fn((), (), (), ()) -> _`
   |     |
   |     expected signature of `for<'r, 't0> fn(&'r (), Box<(dyn for<'s> Fn(&'s ()) + 'static)>, &'t0 (), for<'s, 't0> fn(&'s (), &'t0 ())) -> _`
...
LL | fn h2<F>(_: F) where F: for<'t0> Fn(&(), Box<dyn Fn(&())>, &'t0 (), fn(&(), &())) {}
   |                         --------------------------------------------------------- required by this bound in `h2`
error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0631`.


------------------------------------------


---- [ui] ui/recursion/issue-83150.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/issue-83150.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:05
