plain
.................................................................................................... 9200/11468
.................................................................................................... 9300/11468
.................................................................................................... 9400/11468
..........................i......i.................................................................. 9500/11468
.................................................................iiiiiii..iiiiii.i.................. 9600/11468
.................................................................................................... 9800/11468
.................................................................................................... 9900/11468
.................................................................................................... 10000/11468
.................................................................................................... 10100/11468
---
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/backtrace-debuginfo/a"
stdout:
------------------------------------------
---------------------------------------
trace does not match position list
still need to find ["backtrace-debuginfo.rs:188", "backtrace-debuginfo.rs:125", "backtrace-debuginfo.rs:108"]
--- stdout
backtrace-debuginfo.rs:108
backtrace-debuginfo.rs:125
backtrace-debuginfo.rs:188
backtrace-debuginfo.rs:188

--- stderr
test case 6
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:106:9
stack backtrace:
   0:     0x7f161570530f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h32fb358f98550f94
   1:     0x7f161577a25c - core::fmt::write::hdf5e1269d6382eee
   2:     0x7f16156f7a35 - std::io::Write::write_fmt::hc7a49dd5bf06903c
   3:     0x7f1615709d2c - std::panicking::default_hook::{{closure}}::h19bf3eaf2cef672a
   4:     0x7f1615709628 - std::panicking::default_hook::he5e1641ff566a835
   5:     0x7f161570a6cf - std::panicking::rust_panic_with_hook::he78cd914d76a310b
   6:     0x557b8c7da0a4 - std::panicking::begin_panic::{{closure}}::h64c2ea47c997ea63
                               at /checkout/library/std/src/panicking.rs:520:9
   7:     0x557b8c7d9ff8 - std::sys_common::backtrace::__rust_end_short_backtrace::h0e02d0e516c5d35c
                               at /checkout/library/std/src/sys_common/backtrace.rs:141:18
   8:     0x557b8c7da63c - std::panicking::begin_panic::h8ae4341e18039288
                               at /checkout/library/std/src/panicking.rs:519:12
   9:     0x557b8c7ce0fb - backtrace_debuginfo::inner_inlined::inner_further_inlined::h0aabe380fd0ff24c
                               at /checkout/src/test/ui/backtrace-debuginfo.rs:106:9
  10:     0x557b8c7ce0fb - backtrace_debuginfo::inner_inlined::he2e3ee617d0991b0
                               at /checkout/src/test/ui/backtrace-debuginfo.rs:125:5
  11:     0x557b8c7ce0fb - backtrace_debuginfo::outer::hb72dd12c6666bd5b
                               at /checkout/src/test/ui/backtrace-debuginfo.rs:125:5
  12:     0x557b8c7cee51 - backtrace_debuginfo::main::had8ee1c1e858a3e3
                               at /checkout/src/test/ui/backtrace-debuginfo.rs:188:9
  13:     0x557b8c7da0fe - core::ops::function::FnOnce::call_once::h8709ab346148f71c
                               at /checkout/library/core/src/ops/function.rs:227:5
  14:     0x557b8c7da031 - std::sys_common::backtrace::__rust_begin_short_backtrace::hea313081f5f1de1e
                               at /checkout/library/std/src/sys_common/backtrace.rs:125:18
  15:     0x557b8c7cf074 - std::rt::lang_start::{{closure}}::h7b22b36ef25729eb
                               at /checkout/library/std/src/rt.rs:66:18
  16:     0x7f161570ac60 - std::rt::lang_start_internal::h7951678b7808ea0e
  17:     0x557b8c7cf04f - std::rt::lang_start::hdc5f7aae00622047
                               at /checkout/library/std/src/rt.rs:65:5
  18:     0x557b8c7ceeff - main
  19:     0x7f161507bbf7 - __libc_start_main
  20:     0x557b8c7c2d3a - _start
  21:                0x0 - <unknown>
---------------------------------------
trace does not match position list
trace does not match position list
still need to find ["backtrace-debuginfo.rs:188", "backtrace-debuginfo.rs:125", "backtrace-debuginfo.rs:113"]
--- stdout
backtrace-debuginfo-aux.rs:13
backtrace-debuginfo.rs:113
backtrace-debuginfo.rs:125
backtrace-debuginfo.rs:125
backtrace-debuginfo.rs:188

--- stderr
test case 8
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:114:9
stack backtrace:
   0:     0x7fa1429f130f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h32fb358f98550f94
   1:     0x7fa142a6625c - core::fmt::write::hdf5e1269d6382eee
   2:     0x7fa1429e3a35 - std::io::Write::write_fmt::hc7a49dd5bf06903c
   3:     0x7fa1429f5d2c - std::panicking::default_hook::{{closure}}::h19bf3eaf2cef672a
   4:     0x7fa1429f5628 - std::panicking::default_hook::he5e1641ff566a835
   5:     0x7fa1429f66cf - std::panicking::rust_panic_with_hook::he78cd914d76a310b
   6:     0x55ba8ccee0a4 - std::panicking::begin_panic::{{closure}}::h64c2ea47c997ea63
                               at /checkout/library/std/src/panicking.rs:520:9
   7:     0x55ba8ccedff8 - std::sys_common::backtrace::__rust_end_short_backtrace::h0e02d0e516c5d35c
                               at /checkout/library/std/src/sys_common/backtrace.rs:141:18
   8:     0x55ba8ccee63c - std::panicking::begin_panic::h8ae4341e18039288
                               at /checkout/library/std/src/panicking.rs:519:12
   9:     0x55ba8cce1bb1 - backtrace_debuginfo::inner_inlined::{{closure}}::h3de2d288741ac471
                               at /checkout/src/test/ui/backtrace-debuginfo.rs:114:9
  10:     0x55ba8cce1f65 - backtrace_debuginfo::aux::callback_inlined::h035b80a00bc85268
                               at /checkout/src/test/ui/backtrace-debuginfo-aux.rs:13:5
  11:     0x55ba8cce1f65 - backtrace_debuginfo::inner_inlined::he2e3ee617d0991b0
                               at /checkout/src/test/ui/backtrace-debuginfo.rs:125:5
  12:     0x55ba8cce1f65 - backtrace_debuginfo::outer::hb72dd12c6666bd5b
                               at /checkout/src/test/ui/backtrace-debuginfo.rs:125:5
  13:     0x55ba8cce2e51 - backtrace_debuginfo::main::had8ee1c1e858a3e3
                               at /checkout/src/test/ui/backtrace-debuginfo.rs:188:9
  14:     0x55ba8ccee0fe - core::ops::function::FnOnce::call_once::h8709ab346148f71c
                               at /checkout/library/core/src/ops/function.rs:227:5
  15:     0x55ba8ccee031 - std::sys_common::backtrace::__rust_begin_short_backtrace::hea313081f5f1de1e
                               at /checkout/library/std/src/sys_common/backtrace.rs:125:18
  16:     0x55ba8cce3074 - std::rt::lang_start::{{closure}}::h7b22b36ef25729eb
                               at /checkout/library/std/src/rt.rs:66:18
  17:     0x7fa1429f6c60 - std::rt::lang_start_internal::h7951678b7808ea0e
  18:     0x55ba8cce304f - std::rt::lang_start::hdc5f7aae00622047
                               at /checkout/library/std/src/rt.rs:65:5
  19:     0x55ba8cce2eff - main
  20:     0x7fa142367bf7 - __libc_start_main
  21:     0x55ba8ccd6d3a - _start
  22:                0x0 - <unknown>
---------------------------------------
trace does not match position list
trace does not match position list
still need to find ["backtrace-debuginfo.rs:188", "backtrace-debuginfo.rs:125", "backtrace-debuginfo.rs:119"]
--- stdout
backtrace-debuginfo.rs:119
backtrace-debuginfo.rs:125
backtrace-debuginfo.rs:188
backtrace-debuginfo.rs:188

--- stderr
test case 9
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:106:9
stack backtrace:
   0:     0x7fd55f9ba30f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h32fb358f98550f94
   1:     0x7fd55fa2f25c - core::fmt::write::hdf5e1269d6382eee
   2:     0x7fd55f9aca35 - std::io::Write::write_fmt::hc7a49dd5bf06903c
   3:     0x7fd55f9bed2c - std::panicking::default_hook::{{closure}}::h19bf3eaf2cef672a
   4:     0x7fd55f9be628 - std::panicking::default_hook::he5e1641ff566a835
   5:     0x7fd55f9bf6cf - std::panicking::rust_panic_with_hook::he78cd914d76a310b
   6:     0x55b17ef980a4 - std::panicking::begin_panic::{{closure}}::h64c2ea47c997ea63
                               at /checkout/library/std/src/panicking.rs:520:9
   7:     0x55b17ef97ff8 - std::sys_common::backtrace::__rust_end_short_backtrace::h0e02d0e516c5d35c
                               at /checkout/library/std/src/sys_common/backtrace.rs:141:18
   8:     0x55b17ef9863c - std::panicking::begin_panic::h8ae4341e18039288
                               at /checkout/library/std/src/panicking.rs:519:12
   9:     0x55b17ef8c2ae - backtrace_debuginfo::inner_inlined::inner_further_inlined::h0aabe380fd0ff24c
                               at /checkout/src/test/ui/backtrace-debuginfo.rs:106:9
  10:     0x55b17ef8c2ae - backtrace_debuginfo::inner_inlined::he2e3ee617d0991b0
                               at /checkout/src/test/ui/backtrace-debuginfo.rs:125:5
  11:     0x55b17ef8c2ae - backtrace_debuginfo::outer::hb72dd12c6666bd5b
                               at /checkout/src/test/ui/backtrace-debuginfo.rs:125:5
  12:     0x55b17ef8ce51 - backtrace_debuginfo::main::had8ee1c1e858a3e3
                               at /checkout/src/test/ui/backtrace-debuginfo.rs:188:9
  13:     0x55b17ef980fe - core::ops::function::FnOnce::call_once::h8709ab346148f71c
                               at /checkout/library/core/src/ops/function.rs:227:5
  14:     0x55b17ef98031 - std::sys_common::backtrace::__rust_begin_short_backtrace::hea313081f5f1de1e
                               at /checkout/library/std/src/sys_common/backtrace.rs:125:18
  15:     0x55b17ef8d074 - std::rt::lang_start::{{closure}}::h7b22b36ef25729eb
                               at /checkout/library/std/src/rt.rs:66:18
  16:     0x7fd55f9bfc60 - std::rt::lang_start_internal::h7951678b7808ea0e
  17:     0x55b17ef8d04f - std::rt::lang_start::hdc5f7aae00622047
                               at /checkout/library/std/src/rt.rs:65:5
  18:     0x55b17ef8ceff - main
  19:     0x7fd55f330bf7 - __libc_start_main
  20:     0x55b17ef80d3a - _start
  21:                0x0 - <unknown>

------------------------------------------
stderr:
------------------------------------------
------------------------------------------
thread 'main' panicked at 'found some errors', /checkout/src/test/ui/backtrace-debuginfo.rs:178:9

------------------------------------------


---
test result: FAILED. 11374 passed; 1 failed; 93 ignored; 0 measured; 0 filtered out; finished in 134.92s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:01
