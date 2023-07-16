plain
downloading https://static.rust-lang.org/dist/2021-03-25/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-03-25/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating git repository `https://github.com/eddyb/rustc-demangle`
---
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.20 (https://github.com/eddyb/rustc-demangle?rev=fd906f850f90f6d4845c7b8219d218293e0ab3ed#fd906f85)
   Compiling panic_abort v0.0.0 (/checkout/library/panic_abort)
   Compiling panic_unwind v0.0.0 (/checkout/library/panic_unwind)
   Compiling gimli v0.23.0
   Compiling object v0.22.0
---
   Compiling ansi_term v0.12.1
   Compiling crc32fast v1.2.0
   Compiling fixedbitset v0.2.0
   Compiling build_helper v0.1.0 (/checkout/src/build_helper)
   Compiling rustc-demangle v0.1.20 (https://github.com/eddyb/rustc-demangle?rev=fd906f850f90f6d4845c7b8219d218293e0ab3ed#fd906f85)
   Compiling punycode v0.4.1
   Compiling pathdiff v0.2.0
   Compiling rustc_error_codes v0.0.0 (/checkout/compiler/rustc_error_codes)
   Compiling lock_api v0.4.1
---
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.20 (https://github.com/eddyb/rustc-demangle?rev=fd906f850f90f6d4845c7b8219d218293e0ab3ed#fd906f85)
   Compiling panic_unwind v0.0.0 (/checkout/library/panic_unwind)
   Compiling panic_abort v0.0.0 (/checkout/library/panic_abort)
   Compiling gimli v0.23.0
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
---
   Compiling fixedbitset v0.2.0
   Compiling build_helper v0.1.0 (/checkout/src/build_helper)
   Compiling unicode-script v0.5.3
   Compiling punycode v0.4.1
   Compiling rustc-demangle v0.1.20 (https://github.com/eddyb/rustc-demangle?rev=fd906f850f90f6d4845c7b8219d218293e0ab3ed#fd906f85)
   Compiling rustc_error_codes v0.0.0 (/checkout/compiler/rustc_error_codes)
   Compiling lock_api v0.4.1
   Compiling tracing-core v0.1.17
   Compiling thread_local v1.0.1
---
....................i.i............................................................................. 12100/12131
...............................
failures:

---- [ui] ui/symbol-names/const-generics-structural-demangling.rs stdout ----

70 LL | #[rustc_symbol_name]
71    | ^^^^^^^^^^^^^^^^^^^^
72 
72 
- error: symbol-name(_RMs2_Csno73SFvQKx_1cINtB3_11OptionUsizeKVNtINtNtCscK5tvALCJol_4core6option6OptionjE4NoneUE)
+ error: symbol-name(_RMs2_Csno73SFvQKx_1cINtB3_11OptionUsizeKVNtINtNtCsfr1t8LusWT0_4core6option6OptionjE4NoneUE)
74   --> $DIR/const-generics-structural-demangling.rs:42:1
76 LL | #[rustc_symbol_name]

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
77    | ^^^^^^^^^^^^^^^^^^^^
77    | ^^^^^^^^^^^^^^^^^^^^
78 
- error: demangling(<c[464da6a86cb672f]::OptionUsize<{core[946e071ead6ce31b]::option::Option::<usize>::None}>>)
+ error: demangling(<c[464da6a86cb672f]::OptionUsize<{core[b3ca84578affe9dc]::option::Option::<usize>::None}>>)
80   --> $DIR/const-generics-structural-demangling.rs:42:1
82 LL | #[rustc_symbol_name]

88 LL | #[rustc_symbol_name]
89    | ^^^^^^^^^^^^^^^^^^^^
89    | ^^^^^^^^^^^^^^^^^^^^
90 
- error: symbol-name(_RMs3_Csno73SFvQKx_1cINtB3_11OptionUsizeKVNtINtNtCscK5tvALCJol_4core6option6OptionjE4SomeTj0_EE)
+ error: symbol-name(_RMs3_Csno73SFvQKx_1cINtB3_11OptionUsizeKVNtINtNtCsfr1t8LusWT0_4core6option6OptionjE4SomeTj0_EE)
92   --> $DIR/const-generics-structural-demangling.rs:48:1
94 LL | #[rustc_symbol_name]

95    | ^^^^^^^^^^^^^^^^^^^^
96 
96 
- error: demangling(<c[464da6a86cb672f]::OptionUsize<{core[946e071ead6ce31b]::option::Option::<usize>::Some(0usize)}>>)
+ error: demangling(<c[464da6a86cb672f]::OptionUsize<{core[b3ca84578affe9dc]::option::Option::<usize>::Some(0usize)}>>)
98   --> $DIR/const-generics-structural-demangling.rs:48:1
100 LL | #[rustc_symbol_name]


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-structural-demangling/const-generics-structural-demangling.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args symbol-names/const-generics-structural-demangling.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-structural-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "--crate-name=c" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-structural-demangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RMCsno73SFvQKx_1cINtB0_7RefByteKRh7b_E)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:8:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::RefByte<{&123u8}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:8:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::RefByte<{&123}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:8:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_Csno73SFvQKx_1cINtB2_6RefZstKRAEE)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:18:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::RefZst<{&[]}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:18:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::RefZst<{&[]}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:18:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_Csno73SFvQKx_1cINtB3_11Array3BytesKAh1_h2_h3_EE)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:26:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::Array3Bytes<{[1u8, 2u8, 3u8]}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:26:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Array3Bytes<{[1, 2, 3]}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:26:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs1_Csno73SFvQKx_1cINtB3_13TupleByteBoolKTh1_b0_EE)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:34:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::TupleByteBool<{(1u8, false)}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:34:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::TupleByteBool<{(1, false)}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:34:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs2_Csno73SFvQKx_1cINtB3_11OptionUsizeKVNtINtNtCsfr1t8LusWT0_4core6option6OptionjE4NoneUE)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:42:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::OptionUsize<{core[b3ca84578affe9dc]::option::Option::<usize>::None}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:42:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::OptionUsize<{core::option::Option::<usize>::None}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:42:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs3_Csno73SFvQKx_1cINtB3_11OptionUsizeKVNtINtNtCsfr1t8LusWT0_4core6option6OptionjE4SomeTj0_EE)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:48:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::OptionUsize<{core[b3ca84578affe9dc]::option::Option::<usize>::Some(0usize)}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:48:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::OptionUsize<{core::option::Option::<usize>::Some(0)}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:48:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs4_Csno73SFvQKx_1cINtB3_4Foo_KVNtB3_3FooS1sRe616263_2chc78_5sliceRAh1_h2_h3_EEE)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:62:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::Foo_<{c[464da6a86cb672f]::Foo { s: "abc", ch: 'x', slice: &[1u8, 2u8, 3u8] }}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:62:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Foo_<{c::Foo { s: "abc", ch: 'x', slice: &[1, 2, 3] }}>>)
  --> /checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs:62:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^

error: aborting due to 21 previous errors
---
test result: FAILED. 12027 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 126.07s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:52
