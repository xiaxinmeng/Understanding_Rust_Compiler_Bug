plain
status: exit status: 2
command: "make"
stdout:
------------------------------------------
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/lib/used_inline_crate.rs \
    $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/used_inline_crate.rs ) \
  --crate-type rlib \
  -Ztrim-diagnostic-paths=no \
  -Zdump-mir=InstrumentCoverage -Zdump-mir-spanview -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_inline_crate \
  -Zinstrument-coverage
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_inline_crate/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "used_inline_crate" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_inline_crate
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_inline_crate/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_inline_crate/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_inline_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.used_inline_crate/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_inline_crate/
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/lib/doctest_crate.rs \
    $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/doctest_crate.rs ) \
  --crate-type rlib \
  -Ztrim-diagnostic-paths=no \
  -Zdump-mir=InstrumentCoverage -Zdump-mir-spanview -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate \
  -Zinstrument-coverage
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "doctest_crate" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.doctest_crate/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate/
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/lib/used_crate.rs \
    $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/used_crate.rs ) \
  --crate-type rlib \
  -Ztrim-diagnostic-paths=no \
  -Zdump-mir=InstrumentCoverage -Zdump-mir-spanview -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_crate \
  -Zinstrument-coverage
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_crate/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "used_crate" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_crate
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_crate/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_crate/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.used_crate/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_crate/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/dead_code.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/dead_code.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zdump-mir=InstrumentCoverage -Zdump-mir-spanview -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.dead_code \
  -Zinstrument-coverage
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.dead_code/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "dead_code" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.dead_code
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.dead_code/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.dead_code/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.dead_code/
diff -u --strip-trailing-cr -r expected_mir_dump.dead_code/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.dead_code/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/inline.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/inline.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zdump-mir=InstrumentCoverage -Zdump-mir-spanview -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.inline \
  -Zinstrument-coverage
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.inline/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "inline" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.inline
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.inline/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.inline/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.inline/
diff -u --strip-trailing-cr -r expected_mir_dump.inline/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.inline/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/if.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/if.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zdump-mir=InstrumentCoverage -Zdump-mir-spanview -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if \
  -Zinstrument-coverage
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "if" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if/
diff -u --strip-trailing-cr -r expected_mir_dump.if/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/while_early_ret.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/while_early_ret.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zdump-mir=InstrumentCoverage -Zdump-mir-spanview -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.while_early_ret \
  -Zinstrument-coverage
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.while_early_ret/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "while_early_ret" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.while_early_ret
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.while_early_ret/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.while_early_ret/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.while_early_ret/
diff -u --strip-trailing-cr -r expected_mir_dump.while_early_ret/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.while_early_ret/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/assert.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/assert.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zdump-mir=InstrumentCoverage -Zdump-mir-spanview -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.assert \
  -Zinstrument-coverage
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.assert/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "assert" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.assert
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.assert/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.assert/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.assert/
diff -u --strip-trailing-cr -r expected_mir_dump.assert/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.assert/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/drop_trait.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/drop_trait.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zdump-mir=InstrumentCoverage -Zdump-mir-spanview -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.drop_trait \
  -Zinstrument-coverage
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.drop_trait/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "drop_trait" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.drop_trait
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.drop_trait/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.drop_trait/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.drop_trait/
diff -u --strip-trailing-cr -r expected_mir_dump.drop_trait/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.drop_trait/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/uses_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/uses_crate.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zdump-mir=InstrumentCoverage -Zdump-mir-spanview -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.uses_crate \
  -Zinstrument-coverage
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.uses_crate/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "uses_crate" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.uses_crate
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.uses_crate/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.uses_crate/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.uses_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.uses_crate/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.uses_crate/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/lazy_boolean.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lazy_boolean.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zdump-mir=InstrumentCoverage -Zdump-mir-spanview -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.lazy_boolean \
  -Zinstrument-coverage
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.lazy_boolean/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "lazy_boolean" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.lazy_boolean
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.lazy_boolean/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.lazy_boolean/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.lazy_boolean/
diff -u --strip-trailing-cr -r expected_mir_dump.lazy_boolean/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.lazy_boolean/
diff -u --strip-trailing-cr -r expected_mir_dump.lazy_boolean/lazy_boolean.main.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.lazy_boolean/lazy_boolean.main.-------.InstrumentCoverage.0.html
--- expected_mir_dump.lazy_boolean/lazy_boolean.main.-------.InstrumentCoverage.0.html 2021-03-30 23:04:06.260382400 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.lazy_boolean/lazy_boolean.main.-------.InstrumentCoverage.0.html 2021-03-30 23:45:54.319648267 +0000
@@ -69,9 +69,9 @@
 </style>
 </head>
 <body>
-<div class="code" style="counter-reset: line 2"><span class="line"><span><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb36]
+<div class="code" style="counter-reset: line 2"><span class="line"><span><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb32]
 7:19-7:35: @1[0]: _3 = &amp;_4
-7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb35]
+7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb31]
 7:19-7:46: @2[1]: _1 = Eq(move _2, const 1_usize)
 7:9-7:16: @2[3]: FakeRead(ForLet, _1)
 9:33-9:42: @3[2]: _8 = (const 0_i32, const 0_i32, const 0_i32)
@@ -79,9 +79,9 @@
 9:17-9:22: @3[6]: _6 = (_8.1: i32)
 9:24-9:29: @3[8]: _7 = (_8.2: i32)
 10:8-10:15: @3[12]: _10 = _1"><span class="annotation">@0,1,2,3⦊</span>fn main() {</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb36]
+<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb32]
 7:19-7:35: @1[0]: _3 = &amp;_4
-7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb35]
+7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb31]
 7:19-7:46: @2[1]: _1 = Eq(move _2, const 1_usize)
 7:9-7:16: @2[3]: FakeRead(ForLet, _1)
 9:33-9:42: @3[2]: _8 = (const 0_i32, const 0_i32, const 0_i32)
@@ -89,9 +89,9 @@
 9:17-9:22: @3[6]: _6 = (_8.1: i32)
 9:24-9:29: @3[8]: _7 = (_8.2: i32)
 10:8-10:15: @3[12]: _10 = _1">    // Initialize test constants in a way that cannot be determined at compile time, to ensure</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb36]
+<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb32]
 7:19-7:35: @1[0]: _3 = &amp;_4
-7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb35]
+7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb31]
 7:19-7:46: @2[1]: _1 = Eq(move _2, const 1_usize)
 7:9-7:16: @2[3]: FakeRead(ForLet, _1)
 9:33-9:42: @3[2]: _8 = (const 0_i32, const 0_i32, const 0_i32)
@@ -99,9 +99,9 @@
 9:17-9:22: @3[6]: _6 = (_8.1: i32)
 9:24-9:29: @3[8]: _7 = (_8.2: i32)
 10:8-10:15: @3[12]: _10 = _1">    // rustc and LLVM cannot optimize out statements (or coverage counters) downstream from</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb36]
+<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb32]
 7:19-7:35: @1[0]: _3 = &amp;_4
-7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb35]
+7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb31]
 7:19-7:46: @2[1]: _1 = Eq(move _2, const 1_usize)
 7:9-7:16: @2[3]: FakeRead(ForLet, _1)
 9:33-9:42: @3[2]: _8 = (const 0_i32, const 0_i32, const 0_i32)
@@ -109,9 +109,9 @@
 9:17-9:22: @3[6]: _6 = (_8.1: i32)
 9:24-9:29: @3[8]: _7 = (_8.2: i32)
 10:8-10:15: @3[12]: _10 = _1">    // dependent conditions.</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb36]
+<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb32]
 7:19-7:35: @1[0]: _3 = &amp;_4
-7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb35]
+7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb31]
 7:19-7:46: @2[1]: _1 = Eq(move _2, const 1_usize)
 7:9-7:16: @2[3]: FakeRead(ForLet, _1)
 9:33-9:42: @3[2]: _8 = (const 0_i32, const 0_i32, const 0_i32)
@@ -119,9 +119,9 @@
 9:17-9:22: @3[6]: _6 = (_8.1: i32)
 9:24-9:29: @3[8]: _7 = (_8.2: i32)
 10:8-10:15: @3[12]: _10 = _1">    let is_true = std::env::args().len() == 1;</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb36]
+<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb32]
 7:19-7:35: @1[0]: _3 = &amp;_4
-7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb35]
+7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb31]
 7:19-7:46: @2[1]: _1 = Eq(move _2, const 1_usize)
 7:9-7:16: @2[3]: FakeRead(ForLet, _1)
 9:33-9:42: @3[2]: _8 = (const 0_i32, const 0_i32, const 0_i32)
@@ -129,9 +129,9 @@
 9:17-9:22: @3[6]: _6 = (_8.1: i32)
 9:24-9:29: @3[8]: _7 = (_8.2: i32)
 10:8-10:15: @3[12]: _10 = _1"></span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb36]
+<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb32]
 7:19-7:35: @1[0]: _3 = &amp;_4
-7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb35]
+7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb31]
 7:19-7:46: @2[1]: _1 = Eq(move _2, const 1_usize)
 7:9-7:16: @2[3]: FakeRead(ForLet, _1)
 9:33-9:42: @3[2]: _8 = (const 0_i32, const 0_i32, const 0_i32)
@@ -139,9 +139,9 @@
 9:17-9:22: @3[6]: _6 = (_8.1: i32)
 9:24-9:29: @3[8]: _7 = (_8.2: i32)
 10:8-10:15: @3[12]: _10 = _1">    let (mut a, mut b, mut c) = (0, 0, 0);</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb36]
+<span class="line"><span class="code even" style="--layer: 1" title="7:19-7:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb32]
 7:19-7:35: @1[0]: _3 = &amp;_4
-7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb35]
+7:19-7:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb31]
 7:19-7:46: @2[1]: _1 = Eq(move _2, const 1_usize)
 7:9-7:16: @2[3]: FakeRead(ForLet, _1)
 9:33-9:42: @3[2]: _8 = (const 0_i32, const 0_i32, const 0_i32)
@@ -169,91 +169,91 @@
 13:9-13:16: @4[2]: _7 = const 100_i32
 10:16-14:6: @4[3]: _9 = const ()">    }<span class="annotation">⦉@4</span></span></span><span><span class="code even" style="--layer: 1" title="14:6-14:6: @5[0]: _9 = const ()"><span class="annotation">@5⦊</span>‸<span class="annotation">⦉@5</span></span></span><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0">    let</span></span>
-<span class="line"><span class="code" style="--layer: 0">        </span><span><span class="code odd" style="--layer: 1" title="16:9-16:17: @10[2]: FakeRead(ForLet, _11)"><span class="annotation">@10⦊</span>somebool<span class="annotation">⦉@10</span></span></span><span class="code" style="--layer: 0"></span></span>
+<span class="line"><span class="code" style="--layer: 0">        </span><span><span class="code odd" style="--layer: 1" title="16:9-16:17: @9[2]: FakeRead(ForLet, _11)"><span class="annotation">@9⦊</span>somebool<span class="annotation">⦉@9</span></span></span><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0">        =</span></span>
 <span class="line"><span class="code" style="--layer: 0">            </span><span><span class="code even" style="--layer: 1" title="18:13-18:14: @6[5]: _13 = _5
 18:17-18:18: @6[7]: _14 = _6
 18:13-18:18: @6[8]: _12 = Lt(move _13, move _14)"><span class="annotation">@6⦊</span>a &lt; b<span class="annotation">⦉@6</span></span></span><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0">        ||</span></span>
-<span class="line"><span class="code" style="--layer: 0">            </span><span><span class="code odd" style="--layer: 1" title="20:13-20:14: @9[2]: _16 = _6
-20:17-20:18: @9[4]: _17 = _7
-20:13-20:18: @9[5]: _15 = Lt(move _16, move _17)"><span class="annotation">@9⦊</span>b &lt; c<span class="annotation">⦉@9</span></span></span><span class="code" style="--layer: 0"></span></span>
+<span class="line"><span class="code" style="--layer: 0">            </span><span><span class="code odd" style="--layer: 1" title="20:13-20:14: @8[2]: _16 = _6
+20:17-20:18: @8[4]: _17 = _7
+20:13-20:18: @8[5]: _15 = Lt(move _16, move _17)"><span class="annotation">@8⦊</span>b &lt; c<span class="annotation">⦉@8</span></span></span><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0">    ;</span></span>
 <span class="line"><span class="code" style="--layer: 0">    let</span></span>
-<span class="line"><span class="code" style="--layer: 0">        </span><span><span class="code even" style="--layer: 1" title="23:9-23:17: @14[2]: FakeRead(ForLet, _18)"><span class="annotation">@14⦊</span>somebool<span class="annotation">⦉@14</span></span></span><span class="code" style="--layer: 0"></span></span>
+<span class="line"><span class="code" style="--layer: 0">        </span><span><span class="code even" style="--layer: 1" title="23:9-23:17: @12[2]: FakeRead(ForLet, _18)"><span class="annotation">@12⦊</span>somebool<span class="annotation">⦉@12</span></span></span><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0">        =</span></span>
-<span class="line"><span class="code" style="--layer: 0">            </span><span><span class="code odd" style="--layer: 1" title="25:13-25:14: @10[6]: _20 = _6
-25:17-25:18: @10[8]: _21 = _5
-25:13-25:18: @10[9]: _19 = Lt(move _20, move _21)"><span class="annotation">@10⦊</span>b &lt; a<span class="annotation">⦉@10</span></span></span><span class="code" style="--layer: 0"></span></span>
+<span class="line"><span class="code" style="--layer: 0">            </span><span><span class="code odd" style="--layer: 1" title="25:13-25:14: @9[6]: _20 = _6
+25:17-25:18: @9[8]: _21 = _5
+25:13-25:18: @9[9]: _19 = Lt(move _20, move _21)"><span class="annotation">@9⦊</span>b &lt; a<span class="annotation">⦉@9</span></span></span><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0">        ||</span></span>
-<span class="line"><span class="code" style="--layer: 0">            </span><span><span class="code even" style="--layer: 1" title="27:13-27:14: @13[2]: _23 = _6
-27:17-27:18: @13[4]: _24 = _7
-27:13-27:18: @13[5]: _22 = Lt(move _23, move _24)"><span class="annotation">@13⦊</span>b &lt; c<span class="annotation">⦉@13</span></span></span><span class="code" style="--layer: 0"></span></span>
+<span class="line"><span class="code" style="--layer: 0">            </span><span><span class="code even" style="--layer: 1" title="27:13-27:14: @11[2]: _23 = _6
+27:17-27:18: @11[4]: _24 = _7
+27:13-27:18: @11[5]: _22 = Lt(move _23, move _24)"><span class="annotation">@11⦊</span>b &lt; c<span class="annotation">⦉@11</span></span></span><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0">    ;</span></span>
-<span class="line"><span class="code" style="--layer: 0">    let </span><span><span class="code odd" style="--layer: 1" title="29:9-29:17: @18[2]: FakeRead(ForLet, _25)"><span class="annotation">@18⦊</span>somebool<span class="annotation">⦉@18</span></span></span><span class="code" style="--layer: 0"> = </span><span><span class="code even" style="--layer: 1" title="29:20-29:21: @14[6]: _27 = _5
-29:24-29:25: @14[8]: _28 = _6
-29:20-29:25: @14[9]: _26 = Lt(move _27, move _28)"><span class="annotation">@14⦊</span>a &lt; b<span class="annotation">⦉@14</span></span></span><span class="code" style="--layer: 0"> &amp;&amp; </span><span><span class="code odd" style="--layer: 1" title="29:29-29:30: @17[2]: _30 = _6
-29:33-29:34: @17[4]: _31 = _7
-29:29-29:34: @17[5]: _29 = Lt(move _30, move _31)"><span class="annotation">@17⦊</span>b &lt; c<span class="annotation">⦉@17</span></span></span><span class="code" style="--layer: 0">;</span></span>
-<span class="line"><span class="code" style="--layer: 0">    let </span><span><span class="code even" style="--layer: 1" title="30:9-30:17: @22[2]: FakeRead(ForLet, _32)"><span class="annotation">@22⦊</span>somebool<span class="annotation">⦉@22</span></span></span><span class="code" style="--layer: 0"> = </span><span><span class="code odd" style="--layer: 1" title="30:20-30:21: @18[6]: _34 = _6
-30:24-30:25: @18[8]: _35 = _5
-30:20-30:25: @18[9]: _33 = Lt(move _34, move _35)"><span class="annotation">@18⦊</span>b &lt; a<span class="annotation">⦉@18</span></span></span><span class="code" style="--layer: 0"> &amp;&amp; </span><span><span class="code even" style="--layer: 1" title="30:29-30:30: @21[2]: _37 = _6
-30:33-30:34: @21[4]: _38 = _7
-30:29-30:34: @21[5]: _36 = Lt(move _37, move _38)"><span class="annotation">@21⦊</span>b &lt; c<span class="annotation">⦉@21</span></span></span><span class="code" style="--layer: 0">;</span></span>
+<span class="line"><span class="code" style="--layer: 0">    let </span><span><span class="code odd" style="--layer: 1" title="29:9-29:17: @15[2]: FakeRead(ForLet, _25)"><span class="annotation">@15⦊</span>somebool<span class="annotation">⦉@15</span></span></span><span class="code" style="--layer: 0"> = </span><span><span class="code even" style="--layer: 1" title="29:20-29:21: @12[6]: _27 = _5
+29:24-29:25: @12[8]: _28 = _6
+29:20-29:25: @12[9]: _26 = Lt(move _27, move _28)"><span class="annotation">@12⦊</span>a &lt; b<span class="annotation">⦉@12</span></span></span><span class="code" style="--layer: 0"> &amp;&amp; </span><span><span class="code odd" style="--layer: 1" title="29:29-29:30: @14[2]: _30 = _6
+29:33-29:34: @14[4]: _31 = _7
+29:29-29:34: @14[5]: _29 = Lt(move _30, move _31)"><span class="annotation">@14⦊</span>b &lt; c<span class="annotation">⦉@14</span></span></span><span class="code" style="--layer: 0">;</span></span>
+<span class="line"><span class="code" style="--layer: 0">    let </span><span><span class="code even" style="--layer: 1" title="30:9-30:17: @18[2]: FakeRead(ForLet, _32)"><span class="annotation">@18⦊</span>somebool<span class="annotation">⦉@18</span></span></span><span class="code" style="--layer: 0"> = </span><span><span class="code odd" style="--layer: 1" title="30:20-30:21: @15[6]: _34 = _6
+30:24-30:25: @15[8]: _35 = _5
+30:20-30:25: @15[9]: _33 = Lt(move _34, move _35)"><span class="annotation">@15⦊</span>b &lt; a<span class="annotation">⦉@15</span></span></span><span class="code" style="--layer: 0"> &amp;&amp; </span><span><span class="code even" style="--layer: 1" title="30:29-30:30: @17[2]: _37 = _6
+30:33-30:34: @17[4]: _38 = _7
+30:29-30:34: @17[5]: _36 = Lt(move _37, move _38)"><span class="annotation">@17⦊</span>b &lt; c<span class="annotation">⦉@17</span></span></span><span class="code" style="--layer: 0">;</span></span>
 <span class="line"><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0">    if</span></span>
-<span class="line"><span class="code" style="--layer: 0">        </span><span><span class="code odd" style="--layer: 1" title="34:9-34:16: @22[6]: _41 = _1
-33:9-34:16: @22[7]: _40 = Not(move _41)"><span class="annotation">@22⦊</span>!</span></span>
-<span class="line"><span class="code odd" style="--layer: 1" title="34:9-34:16: @22[6]: _41 = _1
-33:9-34:16: @22[7]: _40 = Not(move _41)">        is_true<span class="annotation">⦉@22</span></span></span><span class="code" style="--layer: 0"></span></span>
-<span class="line"><span class="code" style="--layer: 0">    </span><span><span class="code even" style="--layer: 1" title="36:9-36:14: @23[0]: _5 = const 2_i32
-35:5-38:6: @23[1]: _39 = const ()"><span class="annotation">@23⦊</span>{</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="36:9-36:14: @23[0]: _5 = const 2_i32
-35:5-38:6: @23[1]: _39 = const ()">        a = 2</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="36:9-36:14: @23[0]: _5 = const 2_i32
-35:5-38:6: @23[1]: _39 = const ()">        ;</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="36:9-36:14: @23[0]: _5 = const 2_i32
-35:5-38:6: @23[1]: _39 = const ()">    }<span class="annotation">⦉@23</span></span></span><span><span class="code odd" style="--layer: 1" title="38:6-38:6: @24[0]: _39 = const ()"><span class="annotation">@24⦊</span>‸<span class="annotation">⦉@24</span></span></span><span class="code" style="--layer: 0"></span></span>
+<span class="line"><span class="code" style="--layer: 0">        </span><span><span class="code odd" style="--layer: 1" title="34:9-34:16: @18[6]: _41 = _1
+33:9-34:16: @18[7]: _40 = Not(move _41)"><span class="annotation">@18⦊</span>!</span></span>
+<span class="line"><span class="code odd" style="--layer: 1" title="34:9-34:16: @18[6]: _41 = _1
+33:9-34:16: @18[7]: _40 = Not(move _41)">        is_true<span class="annotation">⦉@18</span></span></span><span class="code" style="--layer: 0"></span></span>
+<span class="line"><span class="code" style="--layer: 0">    </span><span><span class="code even" style="--layer: 1" title="36:9-36:14: @19[0]: _5 = const 2_i32
+35:5-38:6: @19[1]: _39 = const ()"><span class="annotation">@19⦊</span>{</span></span>
+<span class="line"><span class="code even" style="--layer: 1" title="36:9-36:14: @19[0]: _5 = const 2_i32
+35:5-38:6: @19[1]: _39 = const ()">        a = 2</span></span>
+<span class="line"><span class="code even" style="--layer: 1" title="36:9-36:14: @19[0]: _5 = const 2_i32
+35:5-38:6: @19[1]: _39 = const ()">        ;</span></span>
+<span class="line"><span class="code even" style="--layer: 1" title="36:9-36:14: @19[0]: _5 = const 2_i32
+35:5-38:6: @19[1]: _39 = const ()">    }<span class="annotation">⦉@19</span></span></span><span><span class="code odd" style="--layer: 1" title="38:6-38:6: @20[0]: _39 = const ()"><span class="annotation">@20⦊</span>‸<span class="annotation">⦉@20</span></span></span><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0">    if</span></span>
-<span class="line"><span class="code" style="--layer: 0">        </span><span><span class="code even" style="--layer: 1" title="41:9-41:16: @25[4]: _43 = _1"><span class="annotation">@25⦊</span>is_true<span class="annotation">⦉@25</span></span></span><span class="code" style="--layer: 0"></span></span>
-<span class="line"><span class="code" style="--layer: 0">    </span><span><span class="code odd" style="--layer: 1" title="43:9-43:15: @26[0]: _6 = const 30_i32
-42:5-45:6: @26[1]: _42 = const ()"><span class="annotation">@26⦊</span>{</span></span>
-<span class="line"><span class="code odd" style="--layer: 1" title="43:9-43:15: @26[0]: _6 = const 30_i32
-42:5-45:6: @26[1]: _42 = const ()">        b = 30</span></span>
-<span class="line"><span class="code odd" style="--layer: 1" title="43:9-43:15: @26[0]: _6 = const 30_i32
-42:5-45:6: @26[1]: _42 = const ()">        ;</span></span>
-<span class="line"><span class="code odd" style="--layer: 1" title="43:9-43:15: @26[0]: _6 = const 30_i32
-42:5-45:6: @26[1]: _42 = const ()">    }<span class="annotation">⦉@26</span></span></span><span class="code" style="--layer: 0"></span></span>
+<span class="line"><span class="code" style="--layer: 0">        </span><span><span class="code even" style="--layer: 1" title="41:9-41:16: @21[4]: _43 = _1"><span class="annotation">@21⦊</span>is_true<span class="annotation">⦉@21</span></span></span><span class="code" style="--layer: 0"></span></span>
+<span class="line"><span class="code" style="--layer: 0">    </span><span><span class="code odd" style="--layer: 1" title="43:9-43:15: @22[0]: _6 = const 30_i32
+42:5-45:6: @22[1]: _42 = const ()"><span class="annotation">@22⦊</span>{</span></span>
+<span class="line"><span class="code odd" style="--layer: 1" title="43:9-43:15: @22[0]: _6 = const 30_i32
+42:5-45:6: @22[1]: _42 = const ()">        b = 30</span></span>
+<span class="line"><span class="code odd" style="--layer: 1" title="43:9-43:15: @22[0]: _6 = const 30_i32
+42:5-45:6: @22[1]: _42 = const ()">        ;</span></span>
+<span class="line"><span class="code odd" style="--layer: 1" title="43:9-43:15: @22[0]: _6 = const 30_i32
+42:5-45:6: @22[1]: _42 = const ()">    }<span class="annotation">⦉@22</span></span></span><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0">    else</span></span>
-<span class="line"><span class="code" style="--layer: 0">    </span><span><span class="code even" style="--layer: 1" title="48:9-48:16: @27[0]: _7 = const 400_i32
-47:5-50:6: @27[1]: _42 = const ()"><span class="annotation">@27⦊</span>{</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="48:9-48:16: @27[0]: _7 = const 400_i32
-47:5-50:6: @27[1]: _42 = const ()">        c = 400</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="48:9-48:16: @27[0]: _7 = const 400_i32
-47:5-50:6: @27[1]: _42 = const ()">        ;</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="48:9-48:16: @27[0]: _7 = const 400_i32
-47:5-50:6: @27[1]: _42 = const ()">    }<span class="annotation">⦉@27</span></span></span><span class="code" style="--layer: 0"></span></span>
+<span class="line"><span class="code" style="--layer: 0">    </span><span><span class="code even" style="--layer: 1" title="48:9-48:16: @23[0]: _7 = const 400_i32
+47:5-50:6: @23[1]: _42 = const ()"><span class="annotation">@23⦊</span>{</span></span>
+<span class="line"><span class="code even" style="--layer: 1" title="48:9-48:16: @23[0]: _7 = const 400_i32
+47:5-50:6: @23[1]: _42 = const ()">        c = 400</span></span>
+<span class="line"><span class="code even" style="--layer: 1" title="48:9-48:16: @23[0]: _7 = const 400_i32
+47:5-50:6: @23[1]: _42 = const ()">        ;</span></span>
+<span class="line"><span class="code even" style="--layer: 1" title="48:9-48:16: @23[0]: _7 = const 400_i32
+47:5-50:6: @23[1]: _42 = const ()">    }<span class="annotation">⦉@23</span></span></span><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0"></span></span>
-<span class="line"><span class="code" style="--layer: 0">    if </span><span><span class="code odd" style="--layer: 1" title="52:9-52:16: @28[5]: _46 = _1
-52:8-52:16: @28[6]: _45 = Not(move _46)"><span class="annotation">@28⦊</span>!is_true<span class="annotation">⦉@28</span></span></span><span class="code" style="--layer: 0"> </span><span><span class="code even" style="--layer: 1" title="53:9-53:14: @29[0]: _5 = const 2_i32
-52:17-54:6: @29[1]: _44 = const ()"><span class="annotation">@29⦊</span>{</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="53:9-53:14: @29[0]: _5 = const 2_i32
-52:17-54:6: @29[1]: _44 = const ()">        a = 2;</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="53:9-53:14: @29[0]: _5 = const 2_i32
-52:17-54:6: @29[1]: _44 = const ()">    }<span class="annotation">⦉@29</span></span></span><span><span class="code odd" style="--layer: 1" title="54:6-54:6: @30[0]: _44 = const ()"><span class="annotation">@30⦊</span>‸<span class="annotation">⦉@30</span></span></span><span class="code" style="--layer: 0"></span></span>
+<span class="line"><span class="code" style="--layer: 0">    if </span><span><span class="code odd" style="--layer: 1" title="52:9-52:16: @24[5]: _46 = _1
+52:8-52:16: @24[6]: _45 = Not(move _46)"><span class="annotation">@24⦊</span>!is_true<span class="annotation">⦉@24</span></span></span><span class="code" style="--layer: 0"> </span><span><span class="code even" style="--layer: 1" title="53:9-53:14: @25[0]: _5 = const 2_i32
+52:17-54:6: @25[1]: _44 = const ()"><span class="annotation">@25⦊</span>{</span></span>
+<span class="line"><span class="code even" style="--layer: 1" title="53:9-53:14: @25[0]: _5 = const 2_i32
+52:17-54:6: @25[1]: _44 = const ()">        a = 2;</span></span>
+<span class="line"><span class="code even" style="--layer: 1" title="53:9-53:14: @25[0]: _5 = const 2_i32
+52:17-54:6: @25[1]: _44 = const ()">    }<span class="annotation">⦉@25</span></span></span><span><span class="code odd" style="--layer: 1" title="54:6-54:6: @26[0]: _44 = const ()"><span class="annotation">@26⦊</span>‸<span class="annotation">⦉@26</span></span></span><span class="code" style="--layer: 0"></span></span>
 <span class="line"><span class="code" style="--layer: 0"></span></span>
-<span class="line"><span class="code" style="--layer: 0">    if </span><span><span class="code even" style="--layer: 1" title="56:8-56:15: @31[3]: _47 = _1"><span class="annotation">@31⦊</span>is_true<span class="annotation">⦉@31</span></span></span><span class="code" style="--layer: 0"> </span><span><span class="code odd" style="--layer: 1" title="57:9-57:15: @32[0]: _6 = const 30_i32
-56:16-58:6: @32[1]: _0 = const ()"><span class="annotation">@32⦊</span>{</span></span>
-<span class="line"><span class="code odd" style="--layer: 1" title="57:9-57:15: @32[0]: _6 = const 30_i32
-56:16-58:6: @32[1]: _0 = const ()">        b = 30;</span></span>
-<span class="line"><span class="code odd" style="--layer: 1" title="57:9-57:15: @32[0]: _6 = const 30_i32
-56:16-58:6: @32[1]: _0 = const ()">    }<span class="annotation">⦉@32</span></span></span><span class="code" style="--layer: 0"> else </span><span><span class="code even" style="--layer: 1" title="59:9-59:16: @33[0]: _7 = const 400_i32
-58:12-60:6: @33[1]: _0 = const ()"><span class="annotation">@33⦊</span>{</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="59:9-59:16: @33[0]: _7 = const 400_i32
-58:12-60:6: @33[1]: _0 = const ()">        c = 400;</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="59:9-59:16: @33[0]: _7 = const 400_i32
-58:12-60:6: @33[1]: _0 = const ()">    }<span class="annotation">⦉@33</span></span></span><span class="code" style="--layer: 0"></span></span>
-<span class="line"><span class="code" style="--layer: 0">}</span><span><span class="code odd" style="--layer: 1" title="61:2-61:2: @34.Return: return"><span class="annotation">@34⦊</span>‸<span class="annotation">⦉@34</span></span></span></span></div>
+<span class="line"><span class="code" style="--layer: 0">    if </span><span><span class="code even" style="--layer: 1" title="56:8-56:15: @27[3]: _47 = _1"><span class="annotation">@27⦊</span>is_true<span class="annotation">⦉@27</span></span></span><span class="code" style="--layer: 0"> </span><span><span class="code odd" style="--layer: 1" title="57:9-57:15: @28[0]: _6 = const 30_i32
+56:16-58:6: @28[1]: _0 = const ()"><span class="annotation">@28⦊</span>{</span></span>
+<span class="line"><span class="code odd" style="--layer: 1" title="57:9-57:15: @28[0]: _6 = const 30_i32
+56:16-58:6: @28[1]: _0 = const ()">        b = 30;</span></span>
+<span class="line"><span class="code odd" style="--layer: 1" title="57:9-57:15: @28[0]: _6 = const 30_i32
+56:16-58:6: @28[1]: _0 = const ()">    }<span class="annotation">⦉@28</span></span></span><span class="code" style="--layer: 0"> else </span><span><span class="code even" style="--layer: 1" title="59:9-59:16: @29[0]: _7 = const 400_i32
+58:12-60:6: @29[1]: _0 = const ()"><span class="annotation">@29⦊</span>{</span></span>
+<span class="line"><span class="code even" style="--layer: 1" title="59:9-59:16: @29[0]: _7 = const 400_i32
+58:12-60:6: @29[1]: _0 = const ()">        c = 400;</span></span>
+<span class="line"><span class="code even" style="--layer: 1" title="59:9-59:16: @29[0]: _7 = const 400_i32
+58:12-60:6: @29[1]: _0 = const ()">    }<span class="annotation">⦉@29</span></span></span><span class="code" style="--layer: 0"></span></span>
+<span class="line"><span class="code" style="--layer: 0">}</span><span><span class="code odd" style="--layer: 1" title="61:2-61:2: @30.Return: return"><span class="annotation">@30⦊</span>‸<span class="annotation">⦉@30</span></span></span></span></div>
 </body>
 </html>
------------------------------------------
stderr:
------------------------------------------
warning: function is never used: `unused_private_function`
---
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

warning: function is never used: `unused_fn`
  --> ../coverage/dead_code.rs:15:4
   |
15 | fn unused_fn() {
   |
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted

make: *** [Makefile:74: lazy_boolean] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/coverage-spanview

test result: FAILED. 215 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out; finished in 29.64s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:40:07
