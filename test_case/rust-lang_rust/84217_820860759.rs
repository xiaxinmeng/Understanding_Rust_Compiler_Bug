plain

---- [ui] ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs stdout ----
diff of stderr:

+ error[E0658]: the `#[rustc_main]` attribute is used internally to specify test entry point function
+    |
+    |
+ LL | #![rustc_main]
+    |
+    = help: add `#![feature(rustc_attrs)]` to the crate attributes to enable
+ 
+ 
1 error: attribute must be of the form `#[inline]` or `#[inline(always|never)]`
3    |

111   --> $DIR/issue-43106-gating-of-builtin-attrs-error.rs:14:1
112    |
112    |
113 LL | #![rustc_main]
+    | ^^^^^^^^^^^^^^
115 
115 
116 error: `start` attribute cannot be used at crate level


215 LL |     #[export_name = "2200"] impl S { }
217 
- error: aborting due to 31 previous errors
+ error: aborting due to 32 previous errors
219 
219 
- For more information about this error, try `rustc --explain E0518`.
+ Some errors have detailed explanations: E0518, E0658.
+ For more information about an error, try `rustc --explain E0518`.
221 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error/issue-43106-gating-of-builtin-attrs-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/issue-43106-gating-of-builtin-attrs-error.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: the `#[rustc_main]` attribute is used internally to specify test entry point function
   |
   |
LL | #![rustc_main]
   |
   = help: add `#![feature(rustc_attrs)]` to the crate attributes to enable


error: attribute must be of the form `#[inline]` or `#[inline(always|never)]`
   |
   |
LL |     #[inline = "2100"] fn f() { }
   |
   = note: `#[deny(ill_formed_attribute_input)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>
   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>

error: `start` attribute can only be used on functions
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:109:1
   |
LL | #[start]

error: `start` attribute can only be used on functions
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:112:17
   |
   |
LL |     mod inner { #![start] }

error: `start` attribute can only be used on functions
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:117:5
   |
   |
LL |     #[start] struct S;

error: `start` attribute can only be used on functions
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:120:5
   |
   |
LL |     #[start] type T = S;

error: `start` attribute can only be used on functions
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:123:5
   |
   |
LL |     #[start] impl S { }

error[E0518]: attribute should be applied to function or closure
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:31:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:31:1
   |
LL |   #[inline]
   |   ^^^^^^^^^
LL |   //~^ ERROR attribute should be applied to function or closure
LL | / mod inline {
LL | |     //~^ NOTE not a function or closure
LL | |
LL | |     mod inner { #![inline] }
...  |
LL | |     //~| NOTE not a function or closure
LL | | }
   | |_- not a function or closure

error: attribute should be applied to an `extern crate` item
   |
   |
LL |   #[no_link]
   |   ^^^^^^^^^^
LL |   //~^ ERROR attribute should be applied to an `extern crate` item
LL | / mod no_link {
LL | |     //~^ NOTE not an `extern crate` item
LL | |
LL | |     mod inner { #![no_link] }
...  |
LL | |     //~| NOTE not an `extern crate` item
LL | | }
   | |_- not an `extern crate` item
error: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:85:1
   |
   |
LL |   #[export_name = "2200"]
   |   ^^^^^^^^^^^^^^^^^^^^^^^
LL |   //~^ ERROR attribute should be applied to a function or static
LL | / mod export_name {
LL | |     //~^ NOTE not a function or static
LL | |
LL | |     mod inner { #![export_name="2200"] }
...  |
LL | |     //~| NOTE not a function or static
LL | | }
   | |_- not a function or static

error: attribute should be applied to an `extern crate` item
   |
   |
LL | #![no_link]

error: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:27:1
   |
   |
LL | #![export_name = "2200"]

error[E0518]: attribute should be applied to function or closure
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:29:1
   |
   |
LL | #![inline]
   | ^^^^^^^^^^

error: `macro_export` attribute cannot be used at crate level
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:12:1
   |
LL | #![macro_export]

error: `rustc_main` attribute cannot be used at crate level
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:14:1
   |
   |
LL | #![rustc_main]


error: `start` attribute cannot be used at crate level
   |
   |
LL | #![start]


error: `repr` attribute cannot be used at crate level
   |
   |
LL | #![repr()]


error: `path` attribute cannot be used at crate level
   |
   |
LL | #![path = "3800"]


error: `automatically_derived` attribute cannot be used at crate level
   |
   |
LL | #![automatically_derived]

error[E0518]: attribute should be applied to function or closure
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:36:17
   |
   |
LL |     mod inner { #![inline] }
   |     ------------^^^^^^^^^^-- not a function or closure
error[E0518]: attribute should be applied to function or closure
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:46:5
   |
LL |     #[inline] struct S;
LL |     #[inline] struct S;
   |     ^^^^^^^^^ --------- not a function or closure

error[E0518]: attribute should be applied to function or closure
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:50:5
   |
LL |     #[inline] type T = S;

error[E0518]: attribute should be applied to function or closure
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:54:5
   |
   |
LL |     #[inline] impl S { }


error: attribute should be applied to an `extern crate` item
   |
   |
LL |     mod inner { #![no_link] }
   |     ------------^^^^^^^^^^^-- not an `extern crate` item

error: attribute should be applied to an `extern crate` item
   |
   |
LL |     #[no_link] fn f() { }
   |     ^^^^^^^^^^ ---------- not an `extern crate` item

error: attribute should be applied to an `extern crate` item
   |
   |
LL |     #[no_link] struct S;
   |     ^^^^^^^^^^ --------- not an `extern crate` item

error: attribute should be applied to an `extern crate` item
   |
   |
LL |     #[no_link]type T = S;
   |     ^^^^^^^^^^----------- not an `extern crate` item

error: attribute should be applied to an `extern crate` item
   |
   |
LL |     #[no_link] impl S { }
   |     ^^^^^^^^^^ ---------- not an `extern crate` item
error: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:90:17
   |
   |
LL |     mod inner { #![export_name="2200"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^^^-- not a function or static
error: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:96:5
   |
   |
LL |     #[export_name = "2200"] struct S;

error: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:100:5
   |
   |
LL |     #[export_name = "2200"] type T = S;

error: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:104:5
   |
   |
LL |     #[export_name = "2200"] impl S { }

error: aborting due to 32 previous errors

Some errors have detailed explanations: E0518, E0658.
---
test result: FAILED. 11656 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 123.77s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:54
