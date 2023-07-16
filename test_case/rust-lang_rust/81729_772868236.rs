plain
normalized stderr:
warning: unknown lint: `panic_fmt`
  --> $DIR/format-args-capture.rs:34:13
   |
LL |     #[allow(panic_fmt)]
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: unknown lint: `panic_fmt`
warning: unknown lint: `panic_fmt`
  --> $DIR/format-args-capture.rs:34:13
   |
LL |     #[allow(panic_fmt)]

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
warning: panic message contains an unused formatting placeholder
  --> $DIR/format-args-capture.rs:36:17
  --> $DIR/format-args-capture.rs:36:17
   |
LL |         panic!("{foo}");
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |         panic!("{foo}", ...);
help: or add a "{}" format string to use the message literally
   |
   |
LL |         panic!("{}", "{foo}");

warning: 3 warnings emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture/format-args-capture.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fmt/format-args-capture.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-args-capture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unknown lint: `panic_fmt`
  --> /checkout/src/test/ui/fmt/format-args-capture.rs:34:13
   |
LL |     #[allow(panic_fmt)]
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: unknown lint: `panic_fmt`
warning: unknown lint: `panic_fmt`
  --> /checkout/src/test/ui/fmt/format-args-capture.rs:34:13
   |
LL |     #[allow(panic_fmt)]

warning: panic message contains an unused formatting placeholder
  --> /checkout/src/test/ui/fmt/format-args-capture.rs:36:17
   |
   |
LL |         panic!("{foo}");
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |         panic!("{foo}", ...);
help: or add a "{}" format string to use the message literally
   |
   |
LL |         panic!("{}", "{foo}");

warning: 3 warnings emitted


---
normalized stderr:
warning: unknown lint: `panic_fmt`
  --> $DIR/macro-comma-behavior-rpass.rs:60:9
   |
LL | #[allow(panic_fmt)]
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: panic message contains an unused formatting placeholder
warning: panic message contains an unused formatting placeholder
  --> $DIR/macro-comma-behavior-rpass.rs:69:20
   |
LL |     assert!(true, "{}",);
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     assert!(true, "{}", ...,);
help: or add a "{}" format string to use the message literally
   |
   |
LL |     assert!(true, "{}", "{}",);

warning: panic message contains an unused formatting placeholder
  --> $DIR/macro-comma-behavior-rpass.rs:74:26
   |
   |
LL |     debug_assert!(true, "{}",);
   |
   |
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     debug_assert!(true, "{}", ...,);
help: or add a "{}" format string to use the message literally
   |
   |
LL |     debug_assert!(true, "{}", "{}",);

warning: panic message contains an unused formatting placeholder
  --> $DIR/macro-comma-behavior-rpass.rs:83:27
   |
   |
LL |     if falsum() { panic!("{}",); }
   |
   |
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     if falsum() { panic!("{}", ...,); }
help: or add a "{}" format string to use the message literally
   |
   |
LL |     if falsum() { panic!("{}", "{}",); }

warning: 4 warnings emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.core/macro-comma-behavior-rpass.core.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-comma-behavior-rpass.rs`

error in revision `core`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "core" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.core/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.core/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unknown lint: `panic_fmt`
  --> /checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs:60:9
   |
LL | #[allow(panic_fmt)]
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: panic message contains an unused formatting placeholder
warning: panic message contains an unused formatting placeholder
  --> /checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs:69:20
   |
LL |     assert!(true, "{}",);
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     assert!(true, "{}", ...,);
help: or add a "{}" format string to use the message literally
   |
   |
LL |     assert!(true, "{}", "{}",);

warning: panic message contains an unused formatting placeholder
  --> /checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs:74:26
   |
   |
LL |     debug_assert!(true, "{}",);
   |
   |
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     debug_assert!(true, "{}", ...,);
help: or add a "{}" format string to use the message literally
   |
   |
LL |     debug_assert!(true, "{}", "{}",);

warning: panic message contains an unused formatting placeholder
  --> /checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs:83:27
   |
   |
LL |     if falsum() { panic!("{}",); }
   |
   |
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     if falsum() { panic!("{}", ...,); }
help: or add a "{}" format string to use the message literally
   |
   |
LL |     if falsum() { panic!("{}", "{}",); }

warning: 4 warnings emitted


---
normalized stderr:
warning: unknown lint: `panic_fmt`
  --> $DIR/macro-comma-behavior-rpass.rs:60:9
   |
LL | #[allow(panic_fmt)]
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: panic message contains an unused formatting placeholder
warning: panic message contains an unused formatting placeholder
  --> $DIR/macro-comma-behavior-rpass.rs:69:20
   |
LL |     assert!(true, "{}",);
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     assert!(true, "{}", ...,);
help: or add a "{}" format string to use the message literally
   |
   |
LL |     assert!(true, "{}", "{}",);

warning: panic message contains an unused formatting placeholder
  --> $DIR/macro-comma-behavior-rpass.rs:74:26
   |
   |
LL |     debug_assert!(true, "{}",);
   |
   |
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     debug_assert!(true, "{}", ...,);
help: or add a "{}" format string to use the message literally
   |
   |
LL |     debug_assert!(true, "{}", "{}",);

warning: panic message contains an unused formatting placeholder
  --> $DIR/macro-comma-behavior-rpass.rs:83:27
   |
   |
LL |     if falsum() { panic!("{}",); }
   |
   |
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     if falsum() { panic!("{}", ...,); }
help: or add a "{}" format string to use the message literally
   |
   |
LL |     if falsum() { panic!("{}", "{}",); }

warning: 4 warnings emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.std/macro-comma-behavior-rpass.std.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-comma-behavior-rpass.rs`

error in revision `std`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "std" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.std/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.std/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unknown lint: `panic_fmt`
  --> /checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs:60:9
   |
LL | #[allow(panic_fmt)]
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: panic message contains an unused formatting placeholder
warning: panic message contains an unused formatting placeholder
  --> /checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs:69:20
   |
LL |     assert!(true, "{}",);
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     assert!(true, "{}", ...,);
help: or add a "{}" format string to use the message literally
   |
   |
LL |     assert!(true, "{}", "{}",);

warning: panic message contains an unused formatting placeholder
  --> /checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs:74:26
   |
   |
LL |     debug_assert!(true, "{}",);
   |
   |
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     debug_assert!(true, "{}", ...,);
help: or add a "{}" format string to use the message literally
   |
   |
LL |     debug_assert!(true, "{}", "{}",);

warning: panic message contains an unused formatting placeholder
  --> /checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs:83:27
   |
   |
LL |     if falsum() { panic!("{}",); }
   |
   |
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     if falsum() { panic!("{}", ...,); }
help: or add a "{}" format string to use the message literally
   |
   |
LL |     if falsum() { panic!("{}", "{}",); }

warning: 4 warnings emitted



------------------------------------------


---- [ui] ui/panic-brace.rs stdout ----
diff of stderr:

4 LL |     panic!("here's a brace: {");
6    |
6    |
-    = note: `#[warn(panic_fmt)]` on by default
8    = note: this message is not used as a format string, but will be in a future Rust edition
9 help: add a "{}" format string to use the message literally
10    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-brace/panic-brace.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panic-brace.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-brace.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-brace" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-brace/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: panic message contains a brace
  --> /checkout/src/test/ui/panic-brace.rs:11:29
   |
LL |     panic!("here's a brace: {"); //~ WARN panic message contains a brace
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this message is not used as a format string, but will be in a future Rust edition
help: add a "{}" format string to use the message literally
help: add a "{}" format string to use the message literally
   |
LL |     panic!("{}", "here's a brace: {"); //~ WARN panic message contains a brace

warning: panic message contains a brace
  --> /checkout/src/test/ui/panic-brace.rs:12:31
   |
   |
LL |     std::panic!("another one: }"); //~ WARN panic message contains a brace
   |
   = note: this message is not used as a format string, but will be in a future Rust edition
help: add a "{}" format string to use the message literally
   |
   |
LL |     std::panic!("{}", "another one: }"); //~ WARN panic message contains a brace

warning: panic message contains an unused formatting placeholder
  --> /checkout/src/test/ui/panic-brace.rs:13:25
   |
   |
LL |     core::panic!("Hello {}"); //~ WARN panic message contains an unused formatting placeholder
   |
   |
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     core::panic!("Hello {}", ...); //~ WARN panic message contains an unused formatting placeholder
help: or add a "{}" format string to use the message literally
   |
   |
LL |     core::panic!("{}", "Hello {}"); //~ WARN panic message contains an unused formatting placeholder

warning: panic message contains unused formatting placeholders
  --> /checkout/src/test/ui/panic-brace.rs:14:21
   |
   |
LL |     assert!(false, "{:03x} {test} bla");
   |                     ^^^^^^ ^^^^^^
   |
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     assert!(false, "{:03x} {test} bla", ...);
help: or add a "{}" format string to use the message literally
   |
   |
LL |     assert!(false, "{}", "{:03x} {test} bla");

warning: panic message contains braces
  --> /checkout/src/test/ui/panic-brace.rs:16:27
   |
   |
LL |     debug_assert!(false, "{{}} bla"); //~ WARN panic message contains braces
   |
   = note: this message is not used as a format string, but will be in a future Rust edition
help: add a "{}" format string to use the message literally
   |
   |
LL |     debug_assert!(false, "{}", "{{}} bla"); //~ WARN panic message contains braces

warning: panic message contains an unused formatting placeholder
  --> /checkout/src/test/ui/panic-brace.rs:19:12
   |
   |
LL |     panic!(concat!("{", "}")); //~ WARN panic message contains an unused formatting placeholder
   |
   |
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
   |
   |
LL |     panic!(concat!("{", "}"), ...); //~ WARN panic message contains an unused formatting placeholder
help: or add a "{}" format string to use the message literally
   |
   |
LL |     panic!("{}", concat!("{", "}")); //~ WARN panic message contains an unused formatting placeholder

warning: panic message contains braces
  --> /checkout/src/test/ui/panic-brace.rs:20:5
   |
   |
LL |     panic!(concat!("{", "{")); //~ WARN panic message contains braces
   |
   = note: this message is not used as a format string, but will be in a future Rust edition
help: add a "{}" format string to use the message literally
   |
   |
LL |     panic!("{}", concat!("{", "{")); //~ WARN panic message contains braces

warning: panic message contains an unused formatting placeholder
  --> /checkout/src/test/ui/panic-brace.rs:22:37
   |
   |
LL |     fancy_panic::fancy_panic!("test {} 123");
   |
   |
   = note: this message is not used as a format string when given without arguments, but will be in a future Rust edition
warning: 8 warnings emitted


------------------------------------------
---
test result: FAILED. 11280 passed; 4 failed; 92 ignored; 0 measured; 0 filtered out; finished in 135.81s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:53
