plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMqazA+fP6Od

hw.ncpu: 3
hw.byteorder: 1234
+ ./x.py --stage 2 test
---
status: exit code: 2
command: "make"
stdout:
------------------------------------------
# Compile the test library with coverage instrumentation
DYLD_LIBRARY_PATH="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports -L /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/doctest_crate.rs \
   $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lib/doctest_crate.rs && echo "--edition=2018" ) \
   --crate-type rlib -Zinstrument-coverage
# Compile the test library with coverage instrumentation
DYLD_LIBRARY_PATH="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports -L /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/used_crate.rs \
   $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lib/used_crate.rs && echo "--edition=2018" ) \
   --crate-type rlib -Zinstrument-coverage
# Compile the test program with coverage instrumentation
DYLD_LIBRARY_PATH="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports -L /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/abort.rs \
   $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/abort.rs && echo "--edition=2018" ) \
   -L "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports"/abort-%p.profraw \
   DYLD_LIBRARY_PATH="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib:" /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports/abort || \
   ( \
    status=$?; \
    grep -q "^\/\/ expect-exit-status-$status" ../coverage/abort.rs || \
    ( >&2 echo "program exited with an unexpected exit status: $status"; \
    ) \
   )
Don't Panic
Don't Panic
Don't Panic
Don't Panic
Don't Panic
Don't Panic
Don't Panic
Don't Panic
Don't Panic
Don't Panic
Don't Panic
Don't Panic
Don't Panic
# Run it through rustdoc as well to cover doctests
LLVM_PROFILE_FILE="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports"/abort-%p.profraw \
   DYLD_LIBRARY_PATH="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc' -L /Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib --crate-name workaround_for_79771 --test ../coverage/abort.rs \
   $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/abort.rs && echo "--edition=2018" ) \
   -L "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage \
   -Z unstable-options --persist-doctests=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-abort
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin"/llvm-profdata merge --sparse \
   "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports"/abort-*.profraw \
   -o "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports"/abort.profdata
# Generate a coverage report using `llvm-cov show`.
"/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin"/llvm-cov show \
    \
   --ignore-filename-regex=uses_crate.rs \
   --Xdemangler="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-tools-bin/rust-demangler" \
   --show-line-counts-or-regions \
   --instr-profile="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports"/abort.profdata \
   "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports"/abort \
   $( for file in /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-abort/*/rust_out; do [[ -x $file ]] && printf "%s %s " -object $file; done ) \
  2> "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.abort.txt \
  | "/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python" ../coverage-reports/normalize_paths.py \
  > "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.abort.txt || \
 ( status=$? ; \
  >&2 cat "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.abort.txt ; \
  exit $status \

------------------------------------------
stderr:
------------------------------------------
------------------------------------------
warning: function is never used: `unused_private_function`
  --> ../coverage/lib/used_crate.rs:45:4
45 | fn unused_private_function() {
   |    ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

  File "../coverage-reports/normalize_paths.py", line 8
    print(line.replace("\\", "/"), end='')
SyntaxError: invalid syntax
SyntaxError: invalid syntax
make: *** [abort] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/coverage-reports

test result: FAILED. 198 passed; 1 failed; 17 ignored; 0 measured; 0 filtered out; finished in 111.52s



command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--rust-demangler-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-tools-bin/rust-demangler" "--src-base" "/Users/runner/work/rust/rust/src/test/run-make-fulldeps" "--build-base" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-apple-darwin" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python" "--lldb-python" "/usr/bin/python3" "--lldb-version" "lldb-1200.0.41\nApple Swift version 5.3.1 (swiftlang-1200.0.41 clang-1200.0.32.8)\n" "--lldb-python-dir" "/Applications/Xcode_12.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python3" "--llvm-version" "11.0.0-rust-1.50.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin" "--cc" "/Users/runner/work/rust/rust/clang+llvm-10.0.0-x86_64-apple-darwin/bin/clang" "--cxx" "/Users/runner/work/rust/rust/clang+llvm-10.0.0-x86_64-apple-darwin/bin/clang++" "--cflags" "-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 2:15:47
