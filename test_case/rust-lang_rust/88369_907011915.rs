plain
.................................................................................................... 10400/12054
.................................................................................................... 10500/12054
.................................................................................................... 10600/12054
.................................................................................................... 10700/12054
.................F...F...................ii..............................i.......................... 10800/12054
.................................................................................................... 11000/12054
.................................................................................................... 11100/12054
.................................................................................................... 11200/12054
..............................................................................................i..... 11300/12054
---
failures:

---- [ui] ui/symbol-names/const-generics-str-demangling.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-str-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-str-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "--crate-name=c" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-str-demangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0557]: feature has been removed
  --> /checkout/src/test/ui/symbol-names/const-generics-str-demangling.rs:3:12
   |
LL | #![feature(const_generics, rustc_attrs)]
   |            ^^^^^^^^^^^^^^ feature has been removed
   |
   = note: removed in favor of `#![feature(const_param_types]` and `#![feature(generic_const_exprs)]`

error: `&'static str` is forbidden as the type of a const generic parameter
   |
   |
LL | pub struct Str<const S: &'static str>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(const_param_types)]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0557`.


------------------------------------------


---- [ui] ui/symbol-names/const-generics-structural-demangling.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-structural-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "--crate-name=c" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-structural-demangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0557]: feature has been removed
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:9:12
   |
LL | #![feature(const_generics, decl_macro, rustc_attrs)]
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
   = note: removed in favor of `#![feature(const_param_types]` and `#![feature(generic_const_exprs)]`

error: `&'static u8` is forbidden as the type of a const generic parameter
   |
   |
LL | pub struct RefByte<const RB: &'static u8>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(const_param_types)]`

error: `&'static [u8; 0]` is forbidden as the type of a const generic parameter
   |
   |
LL | pub struct RefZst<const RMZ: &'static [u8; 0]>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(const_param_types)]`

error: `[u8; 3]` is forbidden as the type of a const generic parameter
   |
   |
LL | pub struct Array3Bytes<const A3B: [u8; 3]>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(const_param_types)]`

error: `(u8, bool)` is forbidden as the type of a const generic parameter
   |
   |
LL | pub struct TupleByteBool<const TBB: (u8, bool)>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(const_param_types)]`

error: `Option<usize>` is forbidden as the type of a const generic parameter
   |
   |
LL | pub struct OptionUsize<const OU: Option<usize>>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(const_param_types)]`

error: `Foo` is forbidden as the type of a const generic parameter
   |
   |
LL | pub struct Foo_<const F: Foo>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(const_param_types)]`

error: `Bar` is forbidden as the type of a const generic parameter
   |
   |
LL |     pub struct Bar_<const B: Bar>;
...
...
LL | duplicate_field_name_test!(x);
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(const_param_types)]`
   = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0557`.

---
test result: FAILED. 11950 passed; 2 failed; 102 ignored; 0 measured; 0 filtered out; finished in 127.30s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:38
