plain
---- [ui] ui/derives/issue-36617.rs stdout ----
diff of stderr:

6    |
7    = note: import resolution is stuck, try simplifying macro imports
- error: aborting due to previous error
- error: aborting due to previous error
+ error: `derive` attribute cannot be used at crate level
+    |
+    |
+ LL | #![derive(Copy)]
+    |
+    |
+ help: Perhaps you meant to use an outer attribute
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
+ LL - #![derive(Copy)]
+ LL + #[derive(Copy)]
+ 
+ error: aborting due to 2 previous errors
10 
11 
---
To only update this specific test, also pass `--test-args derives/issue-36617.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/issue-36617.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-36617" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-36617/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot determine resolution for the attribute macro `derive`
  --> /checkout/src/test/ui/derives/issue-36617.rs:1:4
   |
LL | #![derive(Copy)] //~ ERROR cannot determine resolution for the attribute macro `derive`
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: `derive` attribute cannot be used at crate level
  --> /checkout/src/test/ui/derives/issue-36617.rs:1:1
   |
   |
LL | #![derive(Copy)] //~ ERROR cannot determine resolution for the attribute macro `derive`
   |
help: Perhaps you meant to use an outer attribute
   |
   |
LL - #![derive(Copy)] //~ ERROR cannot determine resolution for the attribute macro `derive`
LL + #[derive(Copy)] //~ ERROR cannot determine resolution for the attribute macro `derive`

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs stdout ----
diff of stderr:

114    |
115 LL | #![macro_export]
+    |
+    |
+ help: Perhaps you meant to use an outer attribute
+    |
+ LL - #![macro_export]
+ LL + #[macro_export]
117 
118 error: `rustc_main` attribute cannot be used at crate level
119   --> $DIR/issue-43106-gating-of-builtin-attrs-error.rs:14:1


120    |
121 LL | #![rustc_main]
122    | ^^^^^^^^^^^^^^
+    |
+ help: Perhaps you meant to use an outer attribute
+    |
+ LL - #![rustc_main]
+ LL + #[rustc_main]
123 
124 error: `start` attribute cannot be used at crate level
125   --> $DIR/issue-43106-gating-of-builtin-attrs-error.rs:16:1


126    |
127 LL | #![start]
+    |
+    |
+ help: Perhaps you meant to use an outer attribute
+    |
+ LL - #![start]
+ LL + #[start]
129 
129 
130 error: `repr` attribute cannot be used at crate level

132    |
132    |
133 LL | #![repr()]
+    |
+    |
+ help: Perhaps you meant to use an outer attribute
+    |
+ LL - #![repr()]
+ LL + #[repr()]
135 
135 
136 error: `path` attribute cannot be used at crate level

138    |
138    |
139 LL | #![path = "3800"]
+    |
+    |
+ help: Perhaps you meant to use an outer attribute
+    |
+ LL - #![path = "3800"]
+ LL + #[path = "3800"]
141 
141 
142 error: `automatically_derived` attribute cannot be used at crate level

144    |
144    |
145 LL | #![automatically_derived]
+    |
+    |
+ help: Perhaps you meant to use an outer attribute
+    |
+ LL - #![automatically_derived]
+ LL + #[automatically_derived]
147 
148 error[E0518]: attribute should be applied to function or closure
149   --> $DIR/issue-43106-gating-of-builtin-attrs-error.rs:36:17

---
To only update this specific test, also pass `--test-args feature-gates/issue-43106-gating-of-builtin-attrs-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: the `#[rustc_main]` attribute is used internally to specify test entry point function
   |
   |
LL | #![rustc_main] //~ ERROR: the `#[rustc_main]` attribute is used internally to specify
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:119:1
   |
LL | #[start]

error: `start` attribute can only be used on functions
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:122:17
   |
   |
LL |     mod inner { #![start] }

error: `start` attribute can only be used on functions
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:127:5
   |
   |
LL |     #[start] struct S;

error: `start` attribute can only be used on functions
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:130:5
   |
   |
LL |     #[start] type T = S;

error: `start` attribute can only be used on functions
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:133:5
   |
   |
LL |     #[start] impl S { }

error[E0518]: attribute should be applied to function or closure
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:31:1
   |
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:59:1
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

error: attribute should be applied to a free function, impl method or static
   |
   |
LL |   #[export_name = "2200"]
   |   ^^^^^^^^^^^^^^^^^^^^^^^
LL |   //~^ ERROR attribute should be applied to a free function, impl method or static
LL | / mod export_name {
LL | |     //~^ NOTE not a free function, impl method or static
LL | |
LL | |     mod inner { #![export_name="2200"] }
LL | |     }
LL | | }
LL | | }
   | |_- not a free function, impl method or static
error: attribute should be applied to an `extern crate` item
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:25:1
   |
   |
LL | #![no_link]


error: attribute should be applied to a free function, impl method or static
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
   |
help: Perhaps you meant to use an outer attribute
   |
   |
LL - #![macro_export]
LL + #[macro_export]

error: `rustc_main` attribute cannot be used at crate level
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:14:1
   |
   |
LL | #![rustc_main] //~ ERROR: the `#[rustc_main]` attribute is used internally to specify
   |
help: Perhaps you meant to use an outer attribute
   |
   |
LL - #![rustc_main] //~ ERROR: the `#[rustc_main]` attribute is used internally to specify
LL + #[rustc_main] //~ ERROR: the `#[rustc_main]` attribute is used internally to specify

error: `start` attribute cannot be used at crate level
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:16:1
   |
   |
LL | #![start]
   |
help: Perhaps you meant to use an outer attribute
   |
   |
LL - #![start]
LL + #[start]


error: `repr` attribute cannot be used at crate level
   |
   |
LL | #![repr()]
   |
help: Perhaps you meant to use an outer attribute
   |
   |
LL - #![repr()]
LL + #[repr()]


error: `path` attribute cannot be used at crate level
   |
   |
LL | #![path = "3800"]
   |
help: Perhaps you meant to use an outer attribute
   |
   |
LL - #![path = "3800"]
LL + #[path = "3800"]


error: `automatically_derived` attribute cannot be used at crate level
   |
   |
LL | #![automatically_derived]
   |
help: Perhaps you meant to use an outer attribute
   |
   |
LL - #![automatically_derived]
LL + #[automatically_derived]

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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:64:17
   |
   |
LL |     mod inner { #![no_link] }
   |     ------------^^^^^^^^^^^-- not an `extern crate` item
error: attribute should be applied to an `extern crate` item
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:68:5
   |
   |
LL |     #[no_link] fn f() { }
   |     ^^^^^^^^^^ ---------- not an `extern crate` item
error: attribute should be applied to an `extern crate` item
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:72:5
   |
   |
LL |     #[no_link] struct S;
   |     ^^^^^^^^^^ --------- not an `extern crate` item
error: attribute should be applied to an `extern crate` item
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:76:5
   |
   |
LL |     #[no_link]type T = S;
   |     ^^^^^^^^^^----------- not an `extern crate` item
error: attribute should be applied to an `extern crate` item
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs-error.rs:80:5
   |
   |
LL |     #[no_link] impl S { }
   |     ^^^^^^^^^^ ---------- not an `extern crate` item

error: attribute should be applied to a free function, impl method or static
   |
   |
LL |     mod inner { #![export_name="2200"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^^^-- not a free function, impl method or static

error: attribute should be applied to a free function, impl method or static
   |
   |
LL |     #[export_name = "2200"] struct S;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ --------- not a free function, impl method or static

error: attribute should be applied to a free function, impl method or static
   |
   |
LL |     #[export_name = "2200"] type T = S;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ ----------- not a free function, impl method or static

error: attribute should be applied to a free function, impl method or static
   |
   |
LL |     #[export_name = "2200"] impl S { }
   |     ^^^^^^^^^^^^^^^^^^^^^^^ ---------- not a free function, impl method or static

error: attribute should be applied to a free function, impl method or static
   |
   |
LL |         #[export_name = "2200"] fn foo();
   |         ^^^^^^^^^^^^^^^^^^^^^^^ --------- not a free function, impl method or static

error: attribute should be applied to a free function, impl method or static
   |
   |
LL |         #[export_name = "2200"] fn bar() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^ ----------- not a free function, impl method or static
error: aborting due to 34 previous errors

Some errors have detailed explanations: E0518, E0658.
For more information about an error, try `rustc --explain E0518`.
---
---- [ui] ui/span/issue-43927-non-ADT-derive.rs stdout ----
diff of stderr:

6    |
7    = note: import resolution is stuck, try simplifying macro imports
- error: aborting due to previous error
- error: aborting due to previous error
+ error: `derive` attribute cannot be used at crate level
+    |
+    |
+ LL | #![derive(Debug, PartialEq, Eq)] // should be an outer attribute!
+    |
+    |
+ help: Perhaps you meant to use an outer attribute
+    |
+ LL - #![derive(Debug, PartialEq, Eq)] // should be an outer attribute!
+ LL + #[derive(Debug, PartialEq, Eq)] // should be an outer attribute!
+ 
+ error: aborting due to 2 previous errors
10 
11 
---
To only update this specific test, also pass `--test-args span/issue-43927-non-ADT-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-43927-non-ADT-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-43927-non-ADT-derive" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-43927-non-ADT-derive/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot determine resolution for the attribute macro `derive`
   |
   |
LL | #![derive(Debug, PartialEq, Eq)] // should be an outer attribute!
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: `derive` attribute cannot be used at crate level
  --> /checkout/src/test/ui/span/issue-43927-non-ADT-derive.rs:1:1
   |
   |
LL | #![derive(Debug, PartialEq, Eq)] // should be an outer attribute!
   |
help: Perhaps you meant to use an outer attribute
   |
   |
LL - #![derive(Debug, PartialEq, Eq)] // should be an outer attribute!
LL + #[derive(Debug, PartialEq, Eq)] // should be an outer attribute!

error: aborting due to 2 previous errors


---
test result: FAILED. 12148 passed; 3 failed; 116 ignored; 0 measured; 0 filtered out; finished in 127.68s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:13
