plain
status: exit code: 2
command: "make"
stdout:
------------------------------------------
# Compile the test program with coverage instrumentation and generate relevant MIR.
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base  ../coverage/if.rs \
  -Zinstrument-coverage \
  -Clink-dead-code=no
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/if.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base/if || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/if.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/if.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/if.profdata
# Generate a coverage report using `llvm-cov show`. The output ordering
# can be non-deterministic, so ignore the return status. If the test fails
# when comparing the JSON `export`, the `show` output may be useful when
# debugging.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/if.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/if \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_show_coverage.if.txt
# Compare the show coverage output (`--bless` refreshes `typical` files)
# Note `llvm-cov show` output for some programs can vary, but can be ignored
# by inserting `// ignore-llvm-cov-show-diffs` at the top of the source file.
diff -u --strip-trailing-cr expected_show_coverage.if.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_show_coverage.if.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/if.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/if.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/if.rs'; \
 )
 )
# Generate a coverage report in JSON, using `llvm-cov export`, and fail if
# there are differences from the expected output.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov export \
  --summary-only \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/if.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/if \
 | "/usr/bin/python3" ../coverage-reports-base/prettify_json.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_export_coverage.if.json
# Check that exported JSON coverage data matches what we expect (`--bless` refreshes `expected`)
diff -u --strip-trailing-cr expected_export_coverage.if.json "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_export_coverage.if.json
# Compile the test program with coverage instrumentation and generate relevant MIR.
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base  ../coverage/drop_trait.rs \
  -Zinstrument-coverage \
  -Clink-dead-code=no
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/drop_trait.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base/drop_trait || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/drop_trait.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
Exiting with error...
Exiting with error...
BOOM times 100!!!
BOOM times 1!!!
# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/drop_trait.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/drop_trait.profdata
# Generate a coverage report using `llvm-cov show`. The output ordering
# can be non-deterministic, so ignore the return status. If the test fails
# when comparing the JSON `export`, the `show` output may be useful when
# debugging.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/drop_trait.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/drop_trait \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_show_coverage.drop_trait.txt
# Compare the show coverage output (`--bless` refreshes `typical` files)
# Note `llvm-cov show` output for some programs can vary, but can be ignored
# by inserting `// ignore-llvm-cov-show-diffs` at the top of the source file.
diff -u --strip-trailing-cr expected_show_coverage.drop_trait.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_show_coverage.drop_trait.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/drop_trait.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/drop_trait.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/drop_trait.rs'; \
 )
 )
# Generate a coverage report in JSON, using `llvm-cov export`, and fail if
# there are differences from the expected output.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov export \
  --summary-only \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/drop_trait.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/drop_trait \
 | "/usr/bin/python3" ../coverage-reports-base/prettify_json.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_export_coverage.drop_trait.json
# Check that exported JSON coverage data matches what we expect (`--bless` refreshes `expected`)
diff -u --strip-trailing-cr expected_export_coverage.drop_trait.json "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_export_coverage.drop_trait.json
# Compile the test program with coverage instrumentation and generate relevant MIR.
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base  ../coverage/lazy_boolean.rs \
  -Zinstrument-coverage \
  -Clink-dead-code=no
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/lazy_boolean.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base/lazy_boolean || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/lazy_boolean.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/lazy_boolean.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/lazy_boolean.profdata
# Generate a coverage report using `llvm-cov show`. The output ordering
# can be non-deterministic, so ignore the return status. If the test fails
# when comparing the JSON `export`, the `show` output may be useful when
# debugging.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/lazy_boolean.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/lazy_boolean \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_show_coverage.lazy_boolean.txt
# Compare the show coverage output (`--bless` refreshes `typical` files)
# Note `llvm-cov show` output for some programs can vary, but can be ignored
# by inserting `// ignore-llvm-cov-show-diffs` at the top of the source file.
diff -u --strip-trailing-cr expected_show_coverage.lazy_boolean.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_show_coverage.lazy_boolean.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/lazy_boolean.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/lazy_boolean.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/lazy_boolean.rs'; \
 )
 )
# Generate a coverage report in JSON, using `llvm-cov export`, and fail if
# there are differences from the expected output.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov export \
  --summary-only \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/lazy_boolean.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/lazy_boolean \
 | "/usr/bin/python3" ../coverage-reports-base/prettify_json.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_export_coverage.lazy_boolean.json
# Check that exported JSON coverage data matches what we expect (`--bless` refreshes `expected`)
diff -u --strip-trailing-cr expected_export_coverage.lazy_boolean.json "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_export_coverage.lazy_boolean.json
# Compile the test program with coverage instrumentation and generate relevant MIR.
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base  ../coverage/generics.rs \
  -Zinstrument-coverage \
  -Clink-dead-code=no
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/generics.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base/generics || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/generics.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
Exiting with error...
Exiting with error...
BOOM times 300.3!!!
BOOM times 2!!!
# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/generics.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/generics.profdata
# Generate a coverage report using `llvm-cov show`. The output ordering
# can be non-deterministic, so ignore the return status. If the test fails
# when comparing the JSON `export`, the `show` output may be useful when
# debugging.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/generics.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/generics \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_show_coverage.generics.txt
# Compare the show coverage output (`--bless` refreshes `typical` files)
# Note `llvm-cov show` output for some programs can vary, but can be ignored
# by inserting `// ignore-llvm-cov-show-diffs` at the top of the source file.
diff -u --strip-trailing-cr expected_show_coverage.generics.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base"/actual_show_coverage.generics.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/generics.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/generics.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/generics.rs'; \
 )
 )
--- expected_show_coverage.generics.txt 2020-10-07 20:01:00.083534262 +0000
+++ /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-base/coverage-reports-base/actual_show_coverage.generics.txt 2020-10-07 21:47:00.224335891 +0000
@@ -29,12 +29,12 @@
    18|      2|        println!("BOOM times {}!!!", self.strength);
   ------------------
   ------------------
-  | <generics::Firework<i32> as core::ops::drop::Drop>::drop:
+  | <generics::Firework<i32> as core[a7a74cee373f048]::ops::drop::Drop>::drop:
   |   17|      1|    fn drop(&mut self) {
   |   18|      1|        println!("BOOM times {}!!!", self.strength);
   |   19|      1|    }
   ------------------
-  | <generics::Firework<f64> as core::ops::drop::Drop>::drop:
+  | <generics::Firework<f64> as core[a7a74cee373f048]::ops::drop::Drop>::drop:
   |   17|      1|    fn drop(&mut self) {
   |   18|      1|        println!("BOOM times {}!!!", self.strength);
   |   19|      1|    }
------------------------------------------
stderr:
------------------------------------------
Error: 1
Error: 1
Error: 1
diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/generics.rs
make: *** [Makefile:45: generics] Error 1
------------------------------------------


---- [run-make] run-make-fulldeps/coverage-reports-deadcode stdout ----
---- [run-make] run-make-fulldeps/coverage-reports-deadcode stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
# Compile the test program with coverage instrumentation and generate relevant MIR.
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode  ../coverage/if.rs \
  -Zinstrument-coverage \
  -Clink-dead-code=yes
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/if.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode/if || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/if.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/if.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/if.profdata
# Generate a coverage report using `llvm-cov show`. The output ordering
# can be non-deterministic, so ignore the return status. If the test fails
# when comparing the JSON `export`, the `show` output may be useful when
# debugging.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/if.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/if \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_show_coverage.if.txt
# Compare the show coverage output (`--bless` refreshes `typical` files)
# Note `llvm-cov show` output for some programs can vary, but can be ignored
# by inserting `// ignore-llvm-cov-show-diffs` at the top of the source file.
diff -u --strip-trailing-cr expected_show_coverage.if.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_show_coverage.if.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/if.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/if.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/if.rs'; \
 )
 )
# Generate a coverage report in JSON, using `llvm-cov export`, and fail if
# there are differences from the expected output.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov export \
  --summary-only \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/if.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/if \
 | "/usr/bin/python3" ../coverage-reports-base/prettify_json.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_export_coverage.if.json
# Check that exported JSON coverage data matches what we expect (`--bless` refreshes `expected`)
diff -u --strip-trailing-cr expected_export_coverage.if.json "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_export_coverage.if.json
# Compile the test program with coverage instrumentation and generate relevant MIR.
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode  ../coverage/drop_trait.rs \
  -Zinstrument-coverage \
  -Clink-dead-code=yes
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/drop_trait.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode/drop_trait || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/drop_trait.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
Exiting with error...
Exiting with error...
BOOM times 100!!!
BOOM times 1!!!
# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/drop_trait.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/drop_trait.profdata
# Generate a coverage report using `llvm-cov show`. The output ordering
# can be non-deterministic, so ignore the return status. If the test fails
# when comparing the JSON `export`, the `show` output may be useful when
# debugging.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/drop_trait.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/drop_trait \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_show_coverage.drop_trait.txt
# Compare the show coverage output (`--bless` refreshes `typical` files)
# Note `llvm-cov show` output for some programs can vary, but can be ignored
# by inserting `// ignore-llvm-cov-show-diffs` at the top of the source file.
diff -u --strip-trailing-cr expected_show_coverage.drop_trait.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_show_coverage.drop_trait.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/drop_trait.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/drop_trait.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/drop_trait.rs'; \
 )
 )
# Generate a coverage report in JSON, using `llvm-cov export`, and fail if
# there are differences from the expected output.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov export \
  --summary-only \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/drop_trait.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/drop_trait \
 | "/usr/bin/python3" ../coverage-reports-base/prettify_json.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_export_coverage.drop_trait.json
# Check that exported JSON coverage data matches what we expect (`--bless` refreshes `expected`)
diff -u --strip-trailing-cr expected_export_coverage.drop_trait.json "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_export_coverage.drop_trait.json
# Compile the test program with coverage instrumentation and generate relevant MIR.
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode  ../coverage/lazy_boolean.rs \
  -Zinstrument-coverage \
  -Clink-dead-code=yes
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/lazy_boolean.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode/lazy_boolean || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/lazy_boolean.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/lazy_boolean.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/lazy_boolean.profdata
# Generate a coverage report using `llvm-cov show`. The output ordering
# can be non-deterministic, so ignore the return status. If the test fails
# when comparing the JSON `export`, the `show` output may be useful when
# debugging.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/lazy_boolean.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/lazy_boolean \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_show_coverage.lazy_boolean.txt
# Compare the show coverage output (`--bless` refreshes `typical` files)
# Note `llvm-cov show` output for some programs can vary, but can be ignored
# by inserting `// ignore-llvm-cov-show-diffs` at the top of the source file.
diff -u --strip-trailing-cr expected_show_coverage.lazy_boolean.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_show_coverage.lazy_boolean.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/lazy_boolean.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/lazy_boolean.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/lazy_boolean.rs'; \
 )
 )
# Generate a coverage report in JSON, using `llvm-cov export`, and fail if
# there are differences from the expected output.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov export \
  --summary-only \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/lazy_boolean.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/lazy_boolean \
 | "/usr/bin/python3" ../coverage-reports-base/prettify_json.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_export_coverage.lazy_boolean.json
# Check that exported JSON coverage data matches what we expect (`--bless` refreshes `expected`)
diff -u --strip-trailing-cr expected_export_coverage.lazy_boolean.json "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_export_coverage.lazy_boolean.json
# Compile the test program with coverage instrumentation and generate relevant MIR.
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode  ../coverage/generics.rs \
  -Zinstrument-coverage \
  -Clink-dead-code=yes
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/generics.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode/generics || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/generics.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
Exiting with error...
Exiting with error...
BOOM times 300.3!!!
BOOM times 2!!!
# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/generics.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/generics.profdata
# Generate a coverage report using `llvm-cov show`. The output ordering
# can be non-deterministic, so ignore the return status. If the test fails
# when comparing the JSON `export`, the `show` output may be useful when
# debugging.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/generics.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/generics \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_show_coverage.generics.txt
# Compare the show coverage output (`--bless` refreshes `typical` files)
# Note `llvm-cov show` output for some programs can vary, but can be ignored
# by inserting `// ignore-llvm-cov-show-diffs` at the top of the source file.
diff -u --strip-trailing-cr expected_show_coverage.generics.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode"/actual_show_coverage.generics.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/generics.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/generics.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/generics.rs'; \
 )
 )
--- expected_show_coverage.generics.txt 2020-10-07 20:01:00.087534222 +0000
+++ /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode/actual_show_coverage.generics.txt 2020-10-07 21:47:00.748330724 +0000
@@ -29,12 +29,12 @@
    18|      2|        println!("BOOM times {}!!!", self.strength);
   ------------------
   ------------------
-  | <generics::Firework<i32> as core::ops::drop::Drop>::drop:
+  | <generics::Firework<i32> as core[a7a74cee373f048]::ops::drop::Drop>::drop:
   |   17|      1|    fn drop(&mut self) {
   |   18|      1|        println!("BOOM times {}!!!", self.strength);
   |   19|      1|    }
   ------------------
-  | <generics::Firework<f64> as core::ops::drop::Drop>::drop:
+  | <generics::Firework<f64> as core[a7a74cee373f048]::ops::drop::Drop>::drop:
   |   17|      1|    fn drop(&mut self) {
   |   18|      1|        println!("BOOM times {}!!!", self.strength);
   |   19|      1|    }
------------------------------------------
stderr:
------------------------------------------
Error: 1
Error: 1
Error: 1
diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/generics.rs
make: *** [../coverage-reports-base/Makefile:45: generics] Error 1
------------------------------------------



---
test result: FAILED. 210 passed; 2 failed; 7 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-aarch64-unknown-linux-gnu" "--mode" "run-make" "--target" "aarch64-unknown-linux-gnu" "--host" "aarch64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "11.0.0-rust-1.49.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC" "--ar" "ar" "--llvm-bin-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 1:42:51
Build completed unsuccessfully in 1:42:51
== clock drift check ==
  local time: Wed Oct  7 21:48:12 UTC 2020
  network time: Wed, 07 Oct 2020 21:48:13 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (3325) (node)
Terminate orphan process: pid (3334) (python)
