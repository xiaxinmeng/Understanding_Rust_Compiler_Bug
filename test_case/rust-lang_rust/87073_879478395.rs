plain
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-fp-fmt-parse/core-no-fp-fmt-parse:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-fp-fmt-parse/core-no-fp-fmt-parse -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-fp-fmt-parse/core-no-fp-fmt-parse  --edition=2018 --crate-type=rlib ../../../../library/core/src/lib.rs --cfg no_fp_fmt_parse
Makefile:4: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: environment variable `CARGO_PKG_NAME` not defined
   --> ../../../../library/core/src/../../std/src/primitive_docs.rs:103:62
    |
103 | #[doc = concat!("[`exit`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/process_exit.md")))]
    |
    = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
   --> ../../../../library/core/src/../../std/src/primitive_docs.rs:188:64
    |
188 | #[doc = concat!("[`String`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/string_string.md")))]
    |
    = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
   --> ../../../../library/core/src/../../std/src/primitive_docs.rs:264:62
    |
264 | #[doc = concat!("[`File`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/fs_file.md")))]
    |
    = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
   --> ../../../../library/core/src/../../std/src/primitive_docs.rs:303:64
    |
303 | #[doc = concat!("[`String`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/string_string.md")))]
    |
    = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
   --> ../../../../library/core/src/../../std/src/primitive_docs.rs:495:66
    |
495 | #[doc = concat!("[`into_raw`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/box_into_raw.md")))]
    |
    = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
    --> ../../../../library/core/src/../../std/src/primitive_docs.rs:1125:71
     |
1125 | #[doc = concat!("[`ToSocketAddrs`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/net_tosocketaddrs.md")))]
     |
     = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
    --> ../../../../library/core/src/../../std/src/primitive_docs.rs:1146:62
     |
1146 | #[doc = concat!("[`Seek`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_seek.md")))]
     |
     = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
    --> ../../../../library/core/src/../../std/src/primitive_docs.rs:1147:65
     |
1147 | #[doc = concat!("[`BufRead`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_bufread.md")))]
     |
     = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
    --> ../../../../library/core/src/../../std/src/primitive_docs.rs:1148:62
     |
1148 | #[doc = concat!("[`Read`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_read.md")))]
     |
     = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: environment variable `CARGO_PKG_NAME` not defined
    --> ../../../../library/core/src/../../std/src/primitive_docs.rs:1149:67
     |
1149 | #[doc = concat!("[`io::Write`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_write.md")))]
     |
     = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)


error: unexpected token: `(/*ERROR*/)`
   --> ../../../../library/core/src/../../std/src/primitive_docs.rs:103:9
    |
103 | #[doc = concat!("[`exit`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/process_exit.md")))]


error: unexpected token: `(/*ERROR*/)`
   --> ../../../../library/core/src/../../std/src/primitive_docs.rs:188:9
    |
188 | #[doc = concat!("[`String`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/string_string.md")))]


error: unexpected token: `(/*ERROR*/)`
   --> ../../../../library/core/src/../../std/src/primitive_docs.rs:264:9
    |
264 | #[doc = concat!("[`File`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/fs_file.md")))]


error: unexpected token: `(/*ERROR*/)`
   --> ../../../../library/core/src/../../std/src/primitive_docs.rs:303:9
    |
303 | #[doc = concat!("[`String`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/string_string.md")))]


error: unexpected token: `(/*ERROR*/)`
   --> ../../../../library/core/src/../../std/src/primitive_docs.rs:495:9
    |
495 | #[doc = concat!("[`into_raw`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/box_into_raw.md")))]


error: unexpected token: `(/*ERROR*/)`
    --> ../../../../library/core/src/../../std/src/primitive_docs.rs:1125:9
     |
1125 | #[doc = concat!("[`ToSocketAddrs`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/net_tosocketaddrs.md")))]


error: unexpected token: `(/*ERROR*/)`
    --> ../../../../library/core/src/../../std/src/primitive_docs.rs:1146:9
     |
1146 | #[doc = concat!("[`Seek`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_seek.md")))]


error: unexpected token: `(/*ERROR*/)`
    --> ../../../../library/core/src/../../std/src/primitive_docs.rs:1147:9
     |
1147 | #[doc = concat!("[`BufRead`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_bufread.md")))]


error: unexpected token: `(/*ERROR*/)`
    --> ../../../../library/core/src/../../std/src/primitive_docs.rs:1148:9
     |
1148 | #[doc = concat!("[`Read`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_read.md")))]


error: unexpected token: `(/*ERROR*/)`
    --> ../../../../library/core/src/../../std/src/primitive_docs.rs:1149:9
     |
1149 | #[doc = concat!("[`io::Write`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/io_write.md")))]

error: aborting due to 20 previous errors


make: *** [all] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/core-no-fp-fmt-parse

test result: FAILED. 203 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out; finished in 20.59s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-10/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:25:09
