plain
.TEST [ui] ui/array-slice-vec/vec-macro-with-brackets.rs
.TEST [ui] ui/array-slice-vec/vec-matching-legal-tail-element-borrow.rs
.TEST [ui] ui/array-slice-vec/vec-res-add.rs
.TEST [ui] ui/array-slice-vec/vec-matching-fold.rs
.iiiiiiiiTEST [ui] ui/array-slice-vec/vec-matching-fixed.rs
.iiiiiiiiiTEST [ui] ui/array-slice-vec/vec-macro-rvalue-scope.rs
.TEST [ui] ui/array-slice-vec/vec-macro-with-trailing-comma.rs
.TEST [ui] ui/array-slice-vec/vec-matching-autoslice.rs
.TEST [ui] ui/asm/bad-arch.rs
.TEST [ui] ui/array-slice-vec/subslice-patterns-const-eval.rs
---
.TEST [ui] ui/rfc-2565-param-attrs/issue-64682-dropping-first-attrs-in-impl-fns.rs
.TEST [ui] ui/rfcs/rfc-2005-default-binding-mode/range.rs
.TEST [ui] ui/rfcs/rfc-2005-default-binding-mode/for.rs
.TEST [ui] ui/rfcs/rfc-2005-default-binding-mode/general.rs
.iiiiiiiTEST [ui] ui/rfcs/rfc-2005-default-binding-mode/reset-mode.rs
.TEST [ui] ui/rfcs/rfc-2005-default-binding-mode/ref-region.rs
. 9500/11471
TEST [ui] ui/rfcs/rfc-2005-default-binding-mode/struct.rs
.TEST [ui] ui/rfcs/rfc-2005-default-binding-mode/tuple-struct.rs
---
.TEST [ui] ui/rust-2018/remove-extern-crate.rs
.TEST [ui] ui/rust-2018/uniform-paths/prelude-fail.rs
.TEST [ui] ui/rust-2018/uniform-paths/prelude-fail-2.rs
.TEST [ui] ui/rust-2018/uniform-paths/macro-rules.rs
.iiiiiiiTEST [ui] ui/rust-2018/uniform-paths/cross-crate.rs
.TEST [ui] ui/rustc-error.rs
.iiiiiiTEST [ui] ui/rustc-args-required-const2.rs
.iTEST [ui] ui/rust-2018/uniform-paths/fn-local-enum.rs
.TEST [ui] ui/rust-2018/uniform-paths/issue-56596-2.rs
.TEST [ui] ui/rust-2018/uniform-paths/issue-56596.rs
.TEST [ui] ui/save-analysis/emit-notifications.rs
.TEST [ui] ui/sanitize/unsupported-target.rs
---
 finished in 3.909 seconds
Check compiletest suite=codegen mode=codegen (i686-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 242 tests
iiiiiiTEST [codegen] codegen/abi-efiapi.rs#arm
.TEST [codegen] codegen/abi-efiapi.rs#aarch64
.TEST [codegen] codegen/abi-efiapi.rs#i686
.TEST [codegen] codegen/adjustments.rs
.TEST [codegen] codegen/abi-sysv64.rs
---
.iTEST [codegen] codegen/non-terminate/infinite-loop-1.rs
.iTEST [codegen] codegen/non-terminate/infinite-recursion.rs
.TEST [codegen] codegen/mainsubprogram.rs
TEST [codegen] codegen/mir_zst_stores.rs
..TEST [codegen] codegen/noreturnflag.rs
.iiiTEST [codegen] codegen/non-terminate/infinite-loop-2.rs
.iiiiiiiiiiiiiiiiiTEST [codegen] codegen/nrvo.rs
.TEST [codegen] codegen/optimize-attr-1.rs#SPEED-OPT
TEST [codegen] codegen/optimize-attr-1.rs#NO-OPT
..TEST [codegen] codegen/noreturn-uninhabited.rs
.TEST [codegen] codegen/refs.rs
---
 finished in 0.454 seconds
Check compiletest suite=assembly mode=assembly (i686-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 29 tests
iiiiiiiiiTEST [assembly] assembly/asm/wasm-types.rs
.iiiTEST [assembly] assembly/asm/mips-types.rs#mips32
.TEST [assembly] assembly/asm/riscv-types.rs#riscv64
.TEST [assembly] assembly/asm/riscv-types.rs#riscv32
.TEST [assembly] assembly/stack-probes.rs#i686
---
 finished in 6.121 seconds
Check compiletest suite=debuginfo mode=debuginfo (i686-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 116 tests
iiiiiiiiiiTEST [debuginfo-gdb] debuginfo/constant-debug-locs.rs
.iiiTEST [debuginfo-gdb] debuginfo/c-style-enum-in-composite.rs
.iTEST [debuginfo-gdb] debuginfo/associated-types.rs
.iiiTEST [debuginfo-gdb] debuginfo/borrowed-basic.rs
.iiTEST [debuginfo-gdb] debuginfo/borrowed-struct.rs
TEST [debuginfo-gdb] debuginfo/by-value-self-argument-in-trait-impl.rs
---

   Doc-tests core

running 2838 tests
iiiiiiTEST src/alloc/global.rs - alloc::global::GlobalAlloc (line 23)
.TEST src/../../stdarch/crates/core_arch/src/x86/mod.rs - core_arch::x86::__m128i (line 33)
.TEST src/../../stdarch/crates/core_arch/src/x86/mod.rs - core_arch::x86::__m128 (line 26)
.TEST src/../../stdarch/crates/core_arch/src/x86/mod.rs - core_arch::x86::__m256 (line 26)
.TEST src/../../stdarch/crates/core_arch/src/x86/mod.rs - core_arch::x86::__m256d (line 26)
---
TEST whatever
TEST tests::test_should_not_report_time
.TEST whatever
TEST tests::test_error_on_exceed
...TEST whatever
TEST whatever
TEST tests::test_should_panic_but_succeeds
...TEST whatever
...TEST tests::test_should_panic_bad_message
TEST whatever
..TEST tests::test_time_options_threshold
.TEST whatever
TEST tests::test_should_report_time
TEST whatever
TEST tests::test_should_panic_non_string_message_type
TEST tests::test_should_panic_non_string_message_type
........TEST whatever
.TEST tests::test_bench_iter
.
test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.54s

---
.TEST [run-make] run-make-fulldeps/lto-readonly-lib
.TEST [run-make] run-make-fulldeps/lto-no-link-whole-rlib
.TEST [run-make] run-make-fulldeps/output-with-hyphens
.TEST [run-make] run-make-fulldeps/mixing-libs
.iiiiiiTEST [run-make] run-make-fulldeps/output-filename-overwrites-input
.TEST [run-make] run-make-fulldeps/panic-impl-transitive
.TEST [run-make] run-make-fulldeps/pretty-print-to-file
.TEST [run-make] run-make-fulldeps/pretty-expanded
.TEST [run-make] run-make-fulldeps/prefer-dylib
---
status: exit code: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib:/checkout/obj/build/i686-unknown-linux-gnu/stage0-bootstrap-tools/i686-unknown-linux-gnu/release/deps:/checkout/obj/build/i686-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json -L /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json  --test f.rs
RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib:/checkout/obj/build/i686-unknown-linux-gnu/stage0-bootstrap-tools/i686-unknown-linux-gnu/release/deps:/checkout/obj/build/i686-unknown-linux-gnu/stage0/lib" /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json > /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-default.json || true
RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib:/checkout/obj/build/i686-unknown-linux-gnu/stage0-bootstrap-tools/i686-unknown-linux-gnu/release/deps:/checkout/obj/build/i686-unknown-linux-gnu/stage0/lib" /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json --show-output > /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-stdout-success.json || true
cat /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-default.json | "/usr/bin/python3" validate_json.py
Makefile:9: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "validate_json.py", line 8, in <module>
    json.loads(line)
  File "/usr/lib/python3.5/json/__init__.py", line 319, in loads
    return _default_decoder.decode(s)
  File "/usr/lib/python3.5/json/decoder.py", line 339, in decode
    obj, end = self.raw_decode(s, idx=_w(s, 0).end())
  File "/usr/lib/python3.5/json/decoder.py", line 357, in raw_decode
    raise JSONDecodeError("Expecting value", s, err.value) from None
json.decoder.JSONDecodeError: Expecting value: line 1 column 1 (char 0)
make: *** [all] Error 1
------------------------------------------


---- [run-make] run-make-fulldeps/test-harness stdout ----
---- [run-make] run-make-fulldeps/test-harness stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
# check that #[cfg_attr(..., ignore)] does the right thing.
LD_LIBRARY_PATH="/checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/test-harness/test-harness:/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib:/checkout/obj/build/i686-unknown-linux-gnu/stage0-bootstrap-tools/i686-unknown-linux-gnu/release/deps:/checkout/obj/build/i686-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/test-harness/test-harness -L /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/test-harness/test-harness  --test test-ignore-cfg.rs --cfg ignorecfg
LD_LIBRARY_PATH="/checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/test-harness/test-harness:/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib:/checkout/obj/build/i686-unknown-linux-gnu/stage0-bootstrap-tools/i686-unknown-linux-gnu/release/deps:/checkout/obj/build/i686-unknown-linux-gnu/stage0/lib" /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/test-harness/test-harness/test-ignore-cfg | "/checkout/src/etc/cat-and-grep.sh" 'shouldnotignore ... ok' 'shouldignore ... ignored'
[[[ begin stdout ]]]
running 2 tests
test shouldignore ... ignored
TEST shouldnotignore
test shouldnotignore ... ok
test shouldnotignore ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.01s


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/test-harness/test-harness:/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib:/checkout/obj/build/i686-unknown-linux-gnu/stage0-bootstrap-tools/i686-unknown-linux-gnu/release/deps:/checkout/obj/build/i686-unknown-linux-gnu/stage0/lib" /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/test-harness/test-harness/test-ignore-cfg --quiet | "/checkout/src/etc/cat-and-grep.sh" -e "^i\.$"
[[[ begin stdout ]]]
running 2 tests
iTEST shouldnotignore
.
test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s


[[[ end stdout ]]]
Error: cannot match: ^i\.$
Makefile:4: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
make: *** [all] Error 1
------------------------------------------



---
test result: FAILED. 195 passed; 2 failed; 20 ignored; 0 measured; 0 filtered out; finished in 36.48s



command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "11.0.1-rust-1.52.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m32 -march=i686" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:46:33
