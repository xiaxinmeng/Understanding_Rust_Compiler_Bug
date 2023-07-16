plain
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 316 tests
............i..........ii...............................................................  88/316
........................i...i..i.i..i...F.................ii.....i...................... 176/316
.......ii........i...iiiiiii.iii.ii.i...............

failures:


---- [run-make] tests/run-make/coverage-reports stdout ----

error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make/coverage-reports" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC -m64" CXX="c++ -ffunction-sections -fdata-sections -fPIC -m64" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfologicalview debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwarflinkerparallel dwp engine executionengine extensions filecheck frontendhlsl frontendopenacc frontendopenmp fuzzercli fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irprinter irreader jitlink libdriver lineeditor linker loongarch loongarchasmparser loongarchcodegen loongarchdesc loongarchdisassembler loongarchinfo lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts objcopy object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvtargetmca runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target targetparser textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsdriver windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xray" LLVM_FILECHECK="/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="x86_64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" "make"
# Compile the test library with coverage instrumentation
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/lib/used_inline_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/used_inline_crate.rs ) \
  --crate-type rlib -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/lib/unused_mod_helper.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/unused_mod_helper.rs ) \
  --crate-type rlib -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/lib/doctest_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/doctest_crate.rs ) \
  --crate-type rlib -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/lib/inline_always_with_dead_code.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/inline_always_with_dead_code.rs ) \
  --crate-type rlib -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/lib/used_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/used_crate.rs ) \
  --crate-type rlib -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/dead_code.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/dead_code.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/dead_code.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/dead_code || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/dead_code.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/dead_code-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/dead_code.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/dead_code.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-dead_code
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/dead_code*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/dead_code.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/dead_code.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/dead_code \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-dead_code/*/rust_out*; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.dead_code.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 | "/usr/bin/python3" ../coverage-reports/sort_subviews.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.dead_code.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.dead_code.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.dead_code.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage_counters.dead_code.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# `llvm-cov show` normally prints instantiation groups in an unpredictable
# order, but we have used `sort_subviews.py` to sort them, so we can still
# check the output directly with `diff`.
#
# Some of the Rust test source samples have the `// ignore-llvm-cov-show-diffs`
# directive, which effectively disables those tests.
diff -u --strip-trailing-cr \
 expected_show_coverage.dead_code.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.dead_code.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/dead_code.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/dead_code.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/dead_code.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/uses_inline_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/uses_inline_crate.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/uses_inline_crate.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/uses_inline_crate || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/uses_inline_crate.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
used_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_with_same_type_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_with_same_type_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_only_from_this_lib_crate_generic_function with [5, 6, 7, 8]
used_only_from_this_lib_crate_generic_function with "used ONLY from library used_crate.rs"
used_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_with_same_type_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_only_from_this_lib_crate_generic_function with [5, 6, 7, 8]
used_only_from_this_lib_crate_generic_function with "used ONLY from library used_crate.rs"
used_only_from_bin_crate_generic_function with [1, 2, 3, 4]
used_only_from_bin_crate_generic_function with "used from bin uses_crate.rs"
used_from_bin_crate_and_lib_crate_generic_function with [1, 2, 3, 4]
used_with_same_type_from_bin_crate_and_lib_crate_generic_function with "interesting?"
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/uses_inline_crate-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/uses_inline_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/uses_inline_crate.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-uses_inline_crate
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/uses_inline_crate*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/uses_inline_crate.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/uses_inline_crate.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/uses_inline_crate \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-uses_inline_crate/*/rust_out*; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.uses_inline_crate.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 | "/usr/bin/python3" ../coverage-reports/sort_subviews.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.uses_inline_crate.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.uses_inline_crate.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.uses_inline_crate.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage_counters.uses_inline_crate.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# `llvm-cov show` normally prints instantiation groups in an unpredictable
# order, but we have used `sort_subviews.py` to sort them, so we can still
# check the output directly with `diff`.
#
# Some of the Rust test source samples have the `// ignore-llvm-cov-show-diffs`
# directive, which effectively disables those tests.
diff -u --strip-trailing-cr \
 expected_show_coverage.uses_inline_crate.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.uses_inline_crate.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/uses_inline_crate.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/uses_inline_crate.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/uses_inline_crate.rs'; \
 )
 )
--- expected_show_coverage.uses_inline_crate.txt 2023-05-07 02:36:24.059874694 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/actual_show_coverage.uses_inline_crate.txt 2023-05-07 02:48:22.317628225 +0000
@@ -42,8 +42,6 @@
    40|      2|    println!("used_only_from_bin_crate_generic_function with {:?}", arg);
    41|      2|}
   ------------------
-  | Unexecuted instantiation: used_inline_crate::used_only_from_bin_crate_generic_function::<_>
-  ------------------
   | used_inline_crate::used_only_from_bin_crate_generic_function::<&alloc::vec::Vec<i32>>:
   |   39|      1|pub fn used_only_from_bin_crate_generic_function<T: Debug>(arg: T) {
   |   40|      1|    println!("used_only_from_bin_crate_generic_function with {:?}", arg);
@@ -105,27 +103,27 @@
    58|       |
    58|       |
    59|       |#[inline(always)]
-   60|      0|pub fn unused_generic_function<T: Debug>(arg: T) {
-   61|      0|    println!("unused_generic_function with {:?}", arg);
-   62|      0|}
+   60|       |pub fn unused_generic_function<T: Debug>(arg: T) {
+   61|       |    println!("unused_generic_function with {:?}", arg);
+   62|       |}
    63|       |
    64|       |#[inline(always)]
-   65|      0|pub fn unused_function() {
-   66|      0|    let is_true = std::env::args().len() == 1;
-   67|      0|    let mut countdown = 2;
-   68|      0|    if !is_true {
-   69|      0|        countdown = 20;
-   71|      0|}
+   65|       |pub fn unused_function() {
+   66|       |    let is_true = std::env::args().len() == 1;
+   67|       |    let mut countdown = 2;
+   67|       |    let mut countdown = 2;
+   68|       |    if !is_true {
+   69|       |        countdown = 20;
+   70|       |    }
+   71|       |}
    72|       |
    73|       |#[inline(always)]
-   74|      0|fn unused_private_function() {
-   75|      0|    let is_true = std::env::args().len() == 1;
-   76|      0|    let mut countdown = 2;
-   77|      0|    if !is_true {
-   78|      0|        countdown = 20;
-   80|      0|}
+   74|       |fn unused_private_function() {
+   75|       |    let is_true = std::env::args().len() == 1;
+   76|       |    let mut countdown = 2;
+   76|       |    let mut countdown = 2;
+   77|       |    if !is_true {
+   78|       |        countdown = 20;
+   79|       |    }
+   80|       |}
    81|       |
    82|      2|fn use_this_lib_crate() {
    83|      2|    used_from_bin_crate_and_lib_crate_generic_function("used from library used_crate.rs");
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/if.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/if.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/if.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/if || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/if.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/if-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/if.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/if.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-if
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/if*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/if.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/if.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/if \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-if/*/rust_out*; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.if.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 | "/usr/bin/python3" ../coverage-reports/sort_subviews.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.if.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.if.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.if.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage_counters.if.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# `llvm-cov show` normally prints instantiation groups in an unpredictable
# order, but we have used `sort_subviews.py` to sort them, so we can still
# check the output directly with `diff`.
#
# Some of the Rust test source samples have the `// ignore-llvm-cov-show-diffs`
# directive, which effectively disables those tests.
diff -u --strip-trailing-cr \
 expected_show_coverage.if.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.if.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/if.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/if.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/if.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/yield.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/yield.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/yield.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/yield || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/yield.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/yield-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/yield.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/yield.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-yield
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/yield*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/yield.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/yield.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/yield \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-yield/*/rust_out*; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.yield.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 | "/usr/bin/python3" ../coverage-reports/sort_subviews.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.yield.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.yield.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.yield.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage_counters.yield.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# `llvm-cov show` normally prints instantiation groups in an unpredictable
# order, but we have used `sort_subviews.py` to sort them, so we can still
# check the output directly with `diff`.
#
# Some of the Rust test source samples have the `// ignore-llvm-cov-show-diffs`
# directive, which effectively disables those tests.
diff -u --strip-trailing-cr \
 expected_show_coverage.yield.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.yield.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/yield.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/yield.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/yield.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/doctest.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/doctest.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/doctest.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/doctest || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/doctest.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/doctest-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/doctest.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/doctest.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-doctest
running 5 tests
test ../coverage/doctest.rs - main (line 68) ... ok
test ../coverage/doctest.rs - (line 44) ... ok
test ../coverage/doctest.rs - (line 16) ... ok
test ../coverage/doctest.rs - (line 16) ... ok
test ../coverage/doctest.rs - (line 5) ... ok
test ../coverage/doctest.rs - (line 22) ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.33s

# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/doctest*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/doctest.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/doctest.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/doctest \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-doctest/*/rust_out*; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.doctest.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 | "/usr/bin/python3" ../coverage-reports/sort_subviews.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.doctest.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.doctest.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.doctest.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage_counters.doctest.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# `llvm-cov show` normally prints instantiation groups in an unpredictable
# order, but we have used `sort_subviews.py` to sort them, so we can still
# check the output directly with `diff`.
#
# Some of the Rust test source samples have the `// ignore-llvm-cov-show-diffs`
# directive, which effectively disables those tests.
diff -u --strip-trailing-cr \
 expected_show_coverage.doctest.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.doctest.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/doctest.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/doctest.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/doctest.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/while_early_ret.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/while_early_ret.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/while_early_ret.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/while_early_ret || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/while_early_ret.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/while_early_ret-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/while_early_ret.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/while_early_ret.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-while_early_ret
running 0 tests
---
called and covered
called but not covered
called and covered
called and covered
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/no_cov_crate-%p-%m.profraw \
Build completed unsuccessfully in 0:10:15
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/no_cov_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/no_cov_crate.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-no_cov_crate
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/no_cov_crate*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/no_cov_crate.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/no_cov_crate.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/no_cov_crate \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-no_cov_crate/*/rust_out*; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.no_cov_crate.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 | "/usr/bin/python3" ../coverage-reports/sort_subviews.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.no_cov_crate.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.no_cov_crate.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.no_cov_crate.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage_counters.no_cov_crate.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# `llvm-cov show` normally prints instantiation groups in an unpredictable
# order, but we have used `sort_subviews.py` to sort them, so we can still
# check the output directly with `diff`.
#
# Some of the Rust test source samples have the `// ignore-llvm-cov-show-diffs`
# directive, which effectively disables those tests.
diff -u --strip-trailing-cr \
 expected_show_coverage.no_cov_crate.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.no_cov_crate.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/no_cov_crate.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/no_cov_crate.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/no_cov_crate.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/test_harness.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/test_harness.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/test_harness.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/test_harness || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/test_harness.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )

running 1 test
running 1 test
test my_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/test_harness-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/test_harness.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/test_harness.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-test_harness
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/test_harness*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/test_harness.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/test_harness.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/test_harness \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-test_harness/*/rust_out*; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.test_harness.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 | "/usr/bin/python3" ../coverage-reports/sort_subviews.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.test_harness.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.test_harness.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.test_harness.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage_counters.test_harness.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# `llvm-cov show` normally prints instantiation groups in an unpredictable
# order, but we have used `sort_subviews.py` to sort them, so we can still
# check the output directly with `diff`.
#
# Some of the Rust test source samples have the `// ignore-llvm-cov-show-diffs`
# directive, which effectively disables those tests.
diff -u --strip-trailing-cr \
 expected_show_coverage.test_harness.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.test_harness.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/test_harness.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/test_harness.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/test_harness.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/issue-93054.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/issue-93054.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/issue-93054.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/issue-93054 || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/issue-93054.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/issue-93054-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/issue-93054.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/issue-93054.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-issue-93054
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/issue-93054*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/issue-93054.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/issue-93054.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/issue-93054 \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-issue-93054/*/rust_out*; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.issue-93054.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 | "/usr/bin/python3" ../coverage-reports/sort_subviews.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.issue-93054.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.issue-93054.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.issue-93054.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage_counters.issue-93054.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# `llvm-cov show` normally prints instantiation groups in an unpredictable
# order, but we have used `sort_subviews.py` to sort them, so we can still
# check the output directly with `diff`.
#
# Some of the Rust test source samples have the `// ignore-llvm-cov-show-diffs`
# directive, which effectively disables those tests.
diff -u --strip-trailing-cr \
 expected_show_coverage.issue-93054.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.issue-93054.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/issue-93054.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/issue-93054.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/issue-93054.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports  ../coverage/uses_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/uses_crate.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage --target x86_64-unknown-linux-gnu
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/uses_crate.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/uses_crate || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/uses_crate.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
used_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_with_same_type_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_with_same_type_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_only_from_this_lib_crate_generic_function with [5, 6, 7, 8]
used_only_from_this_lib_crate_generic_function with "used ONLY from library used_crate.rs"
used_only_from_bin_crate_generic_function with [1, 2, 3, 4]
used_only_from_bin_crate_generic_function with "used from bin uses_crate.rs"
used_from_bin_crate_and_lib_crate_generic_function with [1, 2, 3, 4]
used_with_same_type_from_bin_crate_and_lib_crate_generic_function with "interesting?"
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/uses_crate-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/uses_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/uses_crate.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-uses_crate
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/uses_crate*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/uses_crate.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/uses_crate.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/uses_crate \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/rustdoc-uses_crate/*/rust_out*; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.uses_crate.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 | "/usr/bin/python3" ../coverage-reports/sort_subviews.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.uses_crate.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.uses_crate.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/show_coverage_stderr.uses_crate.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage_counters.uses_crate.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# `llvm-cov show` normally prints instantiation groups in an unpredictable
# order, but we have used `sort_subviews.py` to sort them, so we can still
# check the output directly with `diff`.
#
# Some of the Rust test source samples have the `// ignore-llvm-cov-show-diffs`
# directive, which effectively disables those tests.
diff -u --strip-trailing-cr \
 expected_show_coverage.uses_crate.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports"/actual_show_coverage.uses_crate.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/uses_crate.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/uses_crate.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/uses_crate.rs'; \
 )
 )
--- expected_show_coverage.uses_crate.txt 2023-05-07 02:36:24.059874694 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/coverage-reports/coverage-reports/actual_show_coverage.uses_crate.txt 2023-05-07 02:48:26.289618970 +0000
@@ -19,8 +19,6 @@
    18|      2|    println!("used_only_from_bin_crate_generic_function with {:?}", arg);
    19|      2|}
   ------------------
-  | Unexecuted instantiation: used_crate::used_only_from_bin_crate_generic_function::<_>
-  ------------------
   | used_crate::used_only_from_bin_crate_generic_function::<&alloc::vec::Vec<i32>>:
   |   17|      1|pub fn used_only_from_bin_crate_generic_function<T: Debug>(arg: T) {
   |   18|      1|    println!("used_only_from_bin_crate_generic_function with {:?}", arg);
@@ -77,9 +75,9 @@
   |   31|      1|}
    32|       |
    32|       |
-   33|      0|pub fn unused_generic_function<T: Debug>(arg: T) {
-   34|      0|    println!("unused_generic_function with {:?}", arg);
-   35|      0|}
+   33|       |pub fn unused_generic_function<T: Debug>(arg: T) {
+   34|       |    println!("unused_generic_function with {:?}", arg);
+   35|       |}
    37|      0|pub fn unused_function() {
    38|      0|    let is_true = std::env::args().len() == 1;
    38|      0|    let is_true = std::env::args().len() == 1;
@@ -89,13 +87,13 @@
    43|      0|}
    44|       |
-   45|      0|fn unused_private_function() {
-   46|      0|    let is_true = std::env::args().len() == 1;
---
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

warning: function `unused_fn` is never used
  --> ../coverage/dead_code.rs:15:4
15 | fn unused_fn() {
   |    ^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/uses_inline_crate.rs
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `2`,
  left: `2`,
 right: `3`: the argument was wrong', ../coverage/assert.rs:6:5
Error: 1
warning: function `do_not_add_coverage_not_called` is never used
warning: function `do_not_add_coverage_not_called` is never used
  --> ../coverage/no_cov_crate.rs:15:4
   |
15 | fn do_not_add_coverage_not_called() {
   |
   = note: `#[warn(dead_code)]` on by default

warning: function `add_coverage_not_called` is never used
warning: function `add_coverage_not_called` is never used
  --> ../coverage/no_cov_crate.rs:27:4
27 | fn add_coverage_not_called() {
   |    ^^^^^^^^^^^^^^^^^^^^^^^

warning: 2 warnings emitted
warning: 2 warnings emitted

warning: unreachable statement
  --> ../coverage/issue-93054.rs:12:9
   |
11 |         match self { }
   |         -------------- any code following this expression is unreachable
12 |         make().map(|never| match never { });
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
   = note: `#[warn(unreachable_code)]` on by default


warning: enum `Never` is never used
 --> ../coverage/issue-93054.rs:7:6
  |
7 | enum Never { }
  |
  = note: `#[warn(dead_code)]` on by default


warning: methods `foo` and `bar` are never used
  --> ../coverage/issue-93054.rs:10:8
9  | impl Never {
   | ---------- methods in this implementation
10 |     fn foo(self) {
   |        ^^^
   |        ^^^
...
15 |     fn bar(&self) {
   |        ^^^

warning: function `foo2` is never used
  --> ../coverage/issue-93054.rs:20:10
   |
20 | async fn foo2(never: Never) {


warning: function `make` is never used
  --> ../coverage/issue-93054.rs:24:4
   |
24 | fn make() -> Option<Never> {

warning: 5 warnings emitted


diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/uses_crate.rs
make: *** [Makefile:119: uses_crate] Error 1



failures:
