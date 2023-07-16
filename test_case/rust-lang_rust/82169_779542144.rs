plain
.................................................................................................... 9200/11452
.................................................................................................... 9300/11452
.................................................................................................... 9400/11452
...........i......i................................................................................. 9500/11452
.................................................iiiiiii..iiiiii.i.................................. 9600/11452
.................................................................................................... 9800/11452
.................................................................................................... 9900/11452
.................................................................................................... 10000/11452
.................................................................................................... 10100/11452
---
....................................................
failures:
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [ui] ui/macros/assert-format-lazy.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/assert-format-lazy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-format-lazy/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-format-lazy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unreachable expression
  --> /checkout/src/test/ui/macros/assert-format-lazy.rs:4:33
   |
LL |     assert!(true, "Failed: {}", panic!("assert! evaluated format expressions"));
   |                                 |
   |                                 unreachable expression
   |                                 any code following this expression is unreachable
   |
   |
   = note: `#[warn(unreachable_code)]` on by default

warning: unreachable expression
  --> /checkout/src/test/ui/macros/assert-format-lazy.rs:5:39
   |
LL |     debug_assert!(true, "Failed: {}", panic!("debug_assert! evaluated format expressions"));
   |                                       |
   |                                       unreachable expression
   |                                       any code following this expression is unreachable


warning: unreachable expression
  --> /checkout/src/test/ui/macros/assert-format-lazy.rs:6:36
   |
LL |     assert_eq!(1, 1, "Failed: {}", panic!("assert_eq! evaluated format expressions"));
   |                                    |
   |                                    unreachable expression
   |                                    any code following this expression is unreachable


warning: unreachable expression
  --> /checkout/src/test/ui/macros/assert-format-lazy.rs:7:42
   |
LL |     debug_assert_eq!(1, 1, "Failed: {}", panic!("debug_assert_eq! evaluated format expressions"));
   |                                          |
   |                                          unreachable expression
   |                                          any code following this expression is unreachable


warning: unreachable expression
  --> /checkout/src/test/ui/macros/assert-format-lazy.rs:8:36
   |
LL |     assert_ne!(1, 2, "Failed: {}", panic!("assert_ne! evaluated format expressions"));
   |                                    |
   |                                    unreachable expression
   |                                    any code following this expression is unreachable


warning: unreachable expression
  --> /checkout/src/test/ui/macros/assert-format-lazy.rs:9:42
   |
LL |     debug_assert_ne!(1, 2, "Failed: {}", panic!("debug_assert_ne! evaluated format expressions"));
   |                                          |
   |                                          unreachable expression
   |                                          any code following this expression is unreachable


error[E0277]: `()` doesn't implement `std::fmt::Display`
  --> /checkout/src/test/ui/macros/assert-format-lazy.rs:4:33
   |
LL |     assert!(true, "Failed: {}", panic!("assert! evaluated format expressions"));
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `()`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: required by `std::fmt::Display::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `()` doesn't implement `std::fmt::Display`
  --> /checkout/src/test/ui/macros/assert-format-lazy.rs:5:39
   |
LL |     debug_assert!(true, "Failed: {}", panic!("debug_assert! evaluated format expressions"));
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `()`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: required by `std::fmt::Display::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `()` doesn't implement `std::fmt::Display`
  --> /checkout/src/test/ui/macros/assert-format-lazy.rs:6:36
   |
LL |     assert_eq!(1, 1, "Failed: {}", panic!("assert_eq! evaluated format expressions"));
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `()`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: required by `std::fmt::Display::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `()` doesn't implement `std::fmt::Display`
  --> /checkout/src/test/ui/macros/assert-format-lazy.rs:7:42
   |
LL |     debug_assert_eq!(1, 1, "Failed: {}", panic!("debug_assert_eq! evaluated format expressions"));
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `()`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: required by `std::fmt::Display::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `()` doesn't implement `std::fmt::Display`
  --> /checkout/src/test/ui/macros/assert-format-lazy.rs:8:36
   |
LL |     assert_ne!(1, 2, "Failed: {}", panic!("assert_ne! evaluated format expressions"));
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `()`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: required by `std::fmt::Display::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `()` doesn't implement `std::fmt::Display`
  --> /checkout/src/test/ui/macros/assert-format-lazy.rs:9:42
   |
LL |     debug_assert_ne!(1, 2, "Failed: {}", panic!("debug_assert_ne! evaluated format expressions"));
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `()`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: required by `std::fmt::Display::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 6 previous errors; 6 warnings emitted

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 11358 passed; 1 failed; 93 ignored; 0 measured; 0 filtered out; finished in 139.81s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:40
