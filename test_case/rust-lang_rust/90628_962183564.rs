plain
.............................................................i.ii................................... 12300/12374
..........................................................................
failures:

---- [ui] ui/privacy/crate-private-reexport.rs stdout ----

11    |             ^^^^
12 
12 
13 error[E0365]: `S1` is only public to inside of the crate, and cannot be re-exported outside
-   --> $DIR/crate-private-reexport.rs:9:9
15    |
15    |
- LL | pub use ::S1;
-    |         ^^^^ re-export of crate public `S1`
+ LL |     pub use ::S1;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |             ^^^^ re-export of crate public `S1`
18    |
19    = note: consider declaring type or module `S1` with `pub`


21 error[E0365]: `E1` is only public to inside of the crate, and cannot be re-exported outside
-   --> $DIR/crate-private-reexport.rs:10:9
23    |
23    |
- LL | pub use ::E1;
-    |         ^^^^ re-export of crate public `E1`
+ LL |     pub use ::E1;
+    |             ^^^^ re-export of crate public `E1`
26    |
27    = note: consider declaring type or module `E1` with `pub`


29 error[E0364]: `V` is only public to inside of the crate, and cannot be re-exported outside
-   --> $DIR/crate-private-reexport.rs:11:9
31    |
31    |
- LL | pub use ::E1::V;
-    |         ^^^^^^^
+ LL |     pub use ::E1::V;
34    |
34    |
35 note: consider marking `V` as `pub` in the imported module
-   --> $DIR/crate-private-reexport.rs:11:9
37    |
37    |
- LL | pub use ::E1::V;
-    |         ^^^^^^^
+ LL |     pub use ::E1::V;
40 
40 
41 error[E0364]: `f2` is only public to inside of the crate, and cannot be re-exported outside

51    |             ^^^^
52 
52 
53 error[E0365]: `S2` is only public to inside of the crate, and cannot be re-exported outside
-   --> $DIR/crate-private-reexport.rs:24:9
55    |
55    |
- LL | pub use ::S2;
-    |         ^^^^ re-export of crate public `S2`
+ LL |     pub use ::S2;
+    |             ^^^^ re-export of crate public `S2`
58    |
59    = note: consider declaring type or module `S2` with `pub`


61 error[E0365]: `E2` is only public to inside of the crate, and cannot be re-exported outside
-   --> $DIR/crate-private-reexport.rs:25:9
63    |
63    |
- LL | pub use ::E2;
-    |         ^^^^ re-export of crate public `E2`
+ LL |     pub use ::E2;
+    |             ^^^^ re-export of crate public `E2`
66    |
67    = note: consider declaring type or module `E2` with `pub`


69 error[E0364]: `V` is only public to inside of the crate, and cannot be re-exported outside
-   --> $DIR/crate-private-reexport.rs:26:9
71    |
71    |
- LL | pub use ::E2::V;
-    |         ^^^^^^^
+ LL |     pub use ::E2::V;
74    |
74    |
75 note: consider marking `V` as `pub` in the imported module
-   --> $DIR/crate-private-reexport.rs:26:9
77    |
77    |
- LL | pub use ::E2::V;
-    |         ^^^^^^^
+ LL |     pub use ::E2::V;
80 
80 
81 error[E0364]: `f3` is only public to inside of the crate, and cannot be re-exported outside

143    |             ^^^^^^^^^^^
144 
144 
145 error[E0364]: `f7` is only public to inside of the crate, and cannot be re-exported outside
-   --> $DIR/crate-private-reexport.rs:54:9
147    |
147    |
- LL | pub use self::m::f7;
-    |         ^^^^^^^^^^^
+ LL |     pub use self::m::f7;
150    |
150    |
151 note: consider marking `f7` as `pub` in the imported module
-   --> $DIR/crate-private-reexport.rs:54:9
153    |
153    |
- LL | pub use self::m::f7;
-    |         ^^^^^^^^^^^
+ LL |     pub use self::m::f7;
156 
156 
157 error[E0364]: `f8` is private, and cannot be re-exported
-   --> $DIR/crate-private-reexport.rs:55:9
159    |
159    |
- LL | pub use self::m::f8;
-    |         ^^^^^^^^^^^
+ LL |     pub use self::m::f8;
162    |
162    |
163 note: consider marking `f8` as `pub` in the imported module
-   --> $DIR/crate-private-reexport.rs:55:9
165    |
165    |
- LL | pub use self::m::f8;
-    |         ^^^^^^^^^^^
+ LL |     pub use self::m::f8;
168 
168 
169 error[E0364]: `f7` is only public to inside of the crate, and cannot be re-exported outside


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/crate-private-reexport/crate-private-reexport.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/crate-private-reexport/crate-private-reexport.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/crate-private-reexport.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/crate-private-reexport.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/crate-private-reexport" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/crate-private-reexport/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0364]: `f1` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:8:13
   |
LL |     pub use ::f1; //~ ERROR `f1` is only public to inside of the crate, and cannot be re-exported outside
   |
   |
note: consider marking `f1` as `pub` in the imported module
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:8:13
   |
LL |     pub use ::f1; //~ ERROR `f1` is only public to inside of the crate, and cannot be re-exported outside


error[E0365]: `S1` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:9:13
   |
LL |     pub use ::S1; //~ ERROR `S1` is only public to inside of the crate, and cannot be re-exported outside
   |             ^^^^ re-export of crate public `S1`
   |
   = note: consider declaring type or module `S1` with `pub`

error[E0365]: `E1` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:10:13
   |
LL |     pub use ::E1; //~ ERROR `E1` is only public to inside of the crate, and cannot be re-exported outside
   |             ^^^^ re-export of crate public `E1`
   |
   = note: consider declaring type or module `E1` with `pub`

error[E0364]: `V` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:11:13
   |
LL |     pub use ::E1::V; //~ ERROR `V` is only public to inside of the crate, and cannot be re-exported outside
   |
   |
note: consider marking `V` as `pub` in the imported module
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:11:13
   |
LL |     pub use ::E1::V; //~ ERROR `V` is only public to inside of the crate, and cannot be re-exported outside


error[E0364]: `f2` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:23:13
   |
LL |     pub use ::f2; //~ ERROR `f2` is only public to inside of the crate, and cannot be re-exported outside
   |
   |
note: consider marking `f2` as `pub` in the imported module
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:23:13
   |
LL |     pub use ::f2; //~ ERROR `f2` is only public to inside of the crate, and cannot be re-exported outside


error[E0365]: `S2` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:24:13
   |
LL |     pub use ::S2; //~ ERROR `S2` is only public to inside of the crate, and cannot be re-exported outside
   |             ^^^^ re-export of crate public `S2`
   |
   = note: consider declaring type or module `S2` with `pub`

error[E0365]: `E2` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:25:13
   |
LL |     pub use ::E2; //~ ERROR `E2` is only public to inside of the crate, and cannot be re-exported outside
   |             ^^^^ re-export of crate public `E2`
   |
   = note: consider declaring type or module `E2` with `pub`

error[E0364]: `V` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:26:13
   |
LL |     pub use ::E2::V; //~ ERROR `V` is only public to inside of the crate, and cannot be re-exported outside
   |
   |
note: consider marking `V` as `pub` in the imported module
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:26:13
   |
LL |     pub use ::E2::V; //~ ERROR `V` is only public to inside of the crate, and cannot be re-exported outside


error[E0364]: `f3` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:39:9
   |
LL | pub use m3::f3; //~ ERROR `f3` is only public to inside of the crate, and cannot be re-exported outside
   |
   |
note: consider marking `f3` as `pub` in the imported module
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:39:9
   |
LL | pub use m3::f3; //~ ERROR `f3` is only public to inside of the crate, and cannot be re-exported outside


error[E0365]: `S3` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:40:9
   |
LL | pub use m3::S3; //~ ERROR `S3` is only public to inside of the crate, and cannot be re-exported outside
   |         ^^^^^^ re-export of crate public `S3`
   |
   = note: consider declaring type or module `S3` with `pub`

error[E0365]: `E3` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:41:9
   |
LL | pub use m3::E3; //~ ERROR `E3` is only public to inside of the crate, and cannot be re-exported outside
   |         ^^^^^^ re-export of crate public `E3`
   |
   = note: consider declaring type or module `E3` with `pub`

error[E0364]: `V` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:42:9
   |
LL | pub use m3::E3::V; //~ ERROR `V` is only public to inside of the crate, and cannot be re-exported outside
   |
   |
note: consider marking `V` as `pub` in the imported module
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:42:9
   |
LL | pub use m3::E3::V; //~ ERROR `V` is only public to inside of the crate, and cannot be re-exported outside


error[E0364]: `f4` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:45:9
   |
LL | pub use ::f4 as f5; //~ ERROR `f4` is only public to inside of the crate, and cannot be re-exported outside
   |
   |
note: consider marking `f4` as `pub` in the imported module
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:45:9
   |
LL | pub use ::f4 as f5; //~ ERROR `f4` is only public to inside of the crate, and cannot be re-exported outside


error[E0364]: `f6` is private, and cannot be re-exported
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:53:13
   |
LL |     pub use self::m::f6; //~ ERROR `f6` is private, and cannot be re-exported
   |
   |
note: consider marking `f6` as `pub` in the imported module
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:53:13
   |
LL |     pub use self::m::f6; //~ ERROR `f6` is private, and cannot be re-exported


error[E0364]: `f7` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:54:13
   |
LL |     pub use self::m::f7; //~ ERROR `f7` is only public to inside of the crate, and cannot be re-exported outside
   |
   |
note: consider marking `f7` as `pub` in the imported module
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:54:13
   |
LL |     pub use self::m::f7; //~ ERROR `f7` is only public to inside of the crate, and cannot be re-exported outside


error[E0364]: `f8` is private, and cannot be re-exported
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:55:13
   |
LL |     pub use self::m::f8; //~ ERROR `f8` is private, and cannot be re-exported
   |
   |
note: consider marking `f8` as `pub` in the imported module
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:55:13
   |
LL |     pub use self::m::f8; //~ ERROR `f8` is private, and cannot be re-exported


error[E0364]: `f7` is only public to inside of the crate, and cannot be re-exported outside
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:58:9
   |
LL | pub use m10::m::f7; //~ ERROR `f7` is only public to inside of the crate, and cannot be re-exported outside
   |
   |
note: consider marking `f7` as `pub` in the imported module
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:58:9
   |
LL | pub use m10::m::f7; //~ ERROR `f7` is only public to inside of the crate, and cannot be re-exported outside

error[E0603]: function `f6` is private
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:57:17
   |
   |
LL | pub use m10::m::f6; //~ ERROR function `f6` is private
   |                 ^^ private function
note: the function `f6` is defined here
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:49:9
   |
LL |         pub(super) fn f6() {}
LL |         pub(super) fn f6() {}
   |         ^^^^^^^^^^^^^^^^^^

error[E0603]: function `f8` is private
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:59:17
   |
LL | pub use m10::m::f8; //~ ERROR function `f8` is private
   |                 ^^ private function
note: the function `f8` is defined here
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:51:9
   |
   |
LL |         pub(in crate::m10) fn f8() {}

error[E0603]: function `f9` is private
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:64:14
   |
   |
LL | pub use m11::f9; //~ ERROR function `f9` is private
   |              ^^ private function
note: the function `f9` is defined here
  --> /checkout/src/test/ui/privacy/crate-private-reexport.rs:62:5
   |
   |
LL |     pub(self) fn f9() {}

error: aborting due to 20 previous errors

Some errors have detailed explanations: E0364, E0365, E0603.
---
test result: FAILED. 12263 passed; 1 failed; 110 ignored; 0 measured; 0 filtered out; finished in 136.99s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:04
