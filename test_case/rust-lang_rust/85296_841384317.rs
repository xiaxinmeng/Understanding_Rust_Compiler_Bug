plain
.................................................................................................... 9500/11864
.................................................................................................... 9600/11864
...................................................................i......i......................... 9700/11864
.................................................................................................... 9800/11864
............iiiiiii..iiiiii.i....................................................................... 9900/11864
.................................................................................................... 10100/11864
.................................................................................................... 10200/11864
.................................................................................................... 10300/11864
.................................................................................................... 10400/11864
---
failures:

---- [ui] ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0557]: feature has been removed
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:38:18
   |
LL | #![feature(test, plugin_registrar)]
   |
   |
   = note: an __rustc_plugin_registrar symbol must now be defined instead
warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:52:9
   |
   |
LL | #![warn(x5400)] //~ WARN unknown lint: `x5400`
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:39:28
   |
   |
LL | #![warn(unused_attributes, unknown_lints)]
   |                            ^^^^^^^^^^^^^

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:53:10
   |
LL | #![allow(x5300)] //~ WARN unknown lint: `x5300`

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:54:11
   |
   |
LL | #![forbid(x5200)] //~ WARN unknown lint: `x5200`

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:55:9
   |
   |
LL | #![deny(x5100)] //~ WARN unknown lint: `x5100`

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:110:8
   |
   |
LL | #[warn(x5400)]

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:113:25
   |
   |
LL |     mod inner { #![warn(x5400)] }

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:116:12
   |
   |
LL |     #[warn(x5400)] fn f() { }

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:119:12
   |
   |
LL |     #[warn(x5400)] struct S;

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:122:12
   |
   |
LL |     #[warn(x5400)] type T = S;

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:125:12
   |
   |
LL |     #[warn(x5400)] impl S { }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:129:9
   |
   |
LL | #[allow(x5300)]

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:132:26
   |
   |
LL |     mod inner { #![allow(x5300)] }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:135:13
   |
   |
LL |     #[allow(x5300)] fn f() { }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:138:13
   |
   |
LL |     #[allow(x5300)] struct S;

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:141:13
   |
   |
LL |     #[allow(x5300)] type T = S;

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:144:13
   |
   |
LL |     #[allow(x5300)] impl S { }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:148:10
   |
   |
LL | #[forbid(x5200)]

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:151:27
   |
   |
LL |     mod inner { #![forbid(x5200)] }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:154:14
   |
   |
LL |     #[forbid(x5200)] fn f() { }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:157:14
   |
   |
LL |     #[forbid(x5200)] struct S;

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:160:14
   |
   |
LL |     #[forbid(x5200)] type T = S;

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:163:14
   |
   |
LL |     #[forbid(x5200)] impl S { }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:167:8
   |
   |
LL | #[deny(x5100)]
   |        ^^^^^

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:170:25
   |
LL |     mod inner { #![deny(x5100)] }

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:173:12
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:173:12
   |
LL |     #[deny(x5100)] fn f() { }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:176:12
   |
   |
LL |     #[deny(x5100)] struct S;

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:179:12
   |
   |
LL |     #[deny(x5100)] type T = S;

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:182:12
   |
   |
LL |     #[deny(x5100)] impl S { }


warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
   |
   |
LL |     mod inner { #![macro_escape] }
   |
   |
   = help: try an outer attribute: `#[macro_use]`

warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
   |
LL | #[macro_escape]
   | ^^^^^^^^^^^^^^^


error: cannot find attribute `plugin_registrar` in this scope
   |
   |
LL | #![plugin_registrar]

error: cannot determine resolution for the attribute macro `test`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:225:3
   |
   |
LL | #[test]
   |   ^^^^
   |
   = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the attribute macro `test`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:226:27
   |
   |
LL | mod test { mod inner { #![test] }
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the attribute macro `bench`
   |
   |
LL | #[bench]
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the attribute macro `bench`
   |
   |
LL |     mod inner { #![bench] }
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the attribute macro `bench`
   |
   |
LL |     #[bench]
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the attribute macro `bench`
   |
   |
LL |     #[bench]
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the attribute macro `bench`
   |
   |
LL |     #[bench]
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

warning: use of deprecated attribute `crate_id`: no longer used.
   |
   |
LL | #![crate_id = "10"]
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated attribute `no_start`: no longer used.
   |
   |
LL | #![no_start]

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:303:1
   |
   |
LL |   #[no_mangle]
...
...
LL | / mod no_mangle {
LL | |     //~^ NOTE not a function or static
LL | |     mod inner { #![no_mangle] }
LL | |     //~^ WARN attribute should be applied to a function or static [unused_attributes]
...  |
LL | |     //~| NOTE not a function or static
LL | | }
   | |_- not a function or static
note: the lint level is defined here
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:39:9
   |
LL | #![warn(unused_attributes, unknown_lints)]
LL | #![warn(unused_attributes, unknown_lints)]
   |         ^^^^^^^^^^^^^^^^^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:470:1
   |
LL |   #[cold]
...
...
LL | / mod cold {
LL | |     //~^ NOTE not a function
LL | |
LL | |     mod inner { #![cold] }
...  |
LL | |     //~| NOTE not a function
LL | | }
   | |_- not a function
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:499:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:499:1
   |
LL |   #[link_name = "1900"]
   |   ^^^^^^^^^^^^^^^^^^^^^
...
LL | / mod link_name {
LL | |     //~^ NOTE not a foreign function or static
LL | |
LL | |     #[link_name = "1900"]
...  |
LL | |     //~| NOTE not a foreign function or static
LL | | }
   | |_- not a foreign function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:538:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:538:1
   |
LL |   #[link_section = "1800"]
   |   ^^^^^^^^^^^^^^^^^^^^^^^^
...
LL | / mod link_section {
LL | |     //~^ NOTE not a function or static
LL | |
LL | |     mod inner { #![link_section="1800"] }
...  |
LL | |     //~| NOTE not a function or static
LL | | }
   | |_- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:68:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:68:1
   |
LL | #![cold] //~ WARN attribute should be applied to a function
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:72:1
   |
LL | #![link_name = "1900"]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:75:1
   |
LL | #![link_section = "1800"]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:308:17
   |
LL |     mod inner { #![no_mangle] }
   |     ------------^^^^^^^^^^^^^-- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:315:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:315:5
   |
LL |     #[no_mangle] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:320:5
   |
LL |     #[no_mangle] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:325:5
   |
LL |     #[no_mangle] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:476:17
   |
LL |     mod inner { #![cold] }
   |     ------------^^^^^^^^-- not a function
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:483:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:483:5
   |
LL |     #[cold] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:488:5
   |
LL |     #[cold] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:493:5
   |
LL |     #[cold] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
---
LL |     extern "C" { }
   |     -------------- not a foreign function or static
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
help: try `#[link(name = "1900")]` instead
   |
LL |     #[link_name = "1900"]
   |     ^^^^^^^^^^^^^^^^^^^^^


warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:512:17
   |
LL |     mod inner { #![link_name="1900"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^-- not a foreign function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:517:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:517:5
   |
LL |     #[link_name = "1900"] fn f() { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:522:5
   |
LL |     #[link_name = "1900"] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:527:5
   |
LL |     #[link_name = "1900"] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:532:5
   |
LL |     #[link_name = "1900"] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:544:17
   |
LL |     mod inner { #![link_section="1800"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^^^^-- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:551:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:551:5
   |
LL |     #[link_section = "1800"] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:556:5
   |
LL |     #[link_section = "1800"] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:561:5
   |
LL |     #[link_section = "1800"] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

error: aborting due to 9 previous errors; 57 warnings emitted
---
test result: FAILED. 11767 passed; 1 failed; 96 ignored; 0 measured; 0 filtered out; finished in 121.85s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:46
