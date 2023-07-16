plain
status: exit code: 2
command: "make"
stdout:
------------------------------------------
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/lib/doctest_crate.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lib/doctest_crate.rs && echo "--edition=2018" ) \
  --crate-type rlib \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate
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
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lib/used_crate.rs && echo "--edition=2018" ) \
  --crate-type rlib \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_crate
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
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/dead_code.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.dead_code
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
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/if.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/if.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if
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
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/yield.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/yield.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.yield
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.yield/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "yield" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.yield
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.yield/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.yield/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.yield/
diff -u --strip-trailing-cr -r expected_mir_dump.yield/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.yield/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/doctest.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/doctest.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "doctest" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest/
diff -u --strip-trailing-cr -r expected_mir_dump.doctest/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/while_early_ret.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/while_early_ret.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.while_early_ret
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
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/assert.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.assert
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
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/drop_trait.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.drop_trait
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
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/inner_items.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/inner_items.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.inner_items
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.inner_items/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "inner_items" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.inner_items
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.inner_items/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.inner_items/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.inner_items/
diff -u --strip-trailing-cr -r expected_mir_dump.inner_items/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.inner_items/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/uses_crate.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/uses_crate.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.uses_crate
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
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lazy_boolean.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.lazy_boolean
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
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/generics.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/generics.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.generics
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.generics/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "generics" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.generics
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.generics/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.generics/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.generics/
diff -u --strip-trailing-cr -r expected_mir_dump.generics/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.generics/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/if_else.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/if_else.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if_else
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if_else/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "if_else" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if_else
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if_else/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if_else/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if_else/
diff -u --strip-trailing-cr -r expected_mir_dump.if_else/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if_else/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/overflow.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/overflow.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.overflow
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.overflow/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "overflow" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.overflow
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.overflow/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.overflow/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.overflow/
diff -u --strip-trailing-cr -r expected_mir_dump.overflow/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.overflow/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/partial_eq.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/partial_eq.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.partial_eq
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.partial_eq/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "partial_eq" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.partial_eq
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.partial_eq/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.partial_eq/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.partial_eq/
diff -u --strip-trailing-cr -r expected_mir_dump.partial_eq/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.partial_eq/
Only in expected_mir_dump.partial_eq/: partial_eq.{impl#2}-ge-{closure#0}-{closure#0}.-------.InstrumentCoverage.0.html
Only in expected_mir_dump.partial_eq/: partial_eq.{impl#2}-ge-{closure#0}.-------.InstrumentCoverage.0.html
Only in expected_mir_dump.partial_eq/: partial_eq.{impl#2}-ge.-------.InstrumentCoverage.0.html
Only in expected_mir_dump.partial_eq/: partial_eq.{impl#2}-gt-{closure#0}-{closure#0}.-------.InstrumentCoverage.0.html
Only in expected_mir_dump.partial_eq/: partial_eq.{impl#2}-gt-{closure#0}.-------.InstrumentCoverage.0.html
Only in expected_mir_dump.partial_eq/: partial_eq.{impl#2}-gt.-------.InstrumentCoverage.0.html
Only in expected_mir_dump.partial_eq/: partial_eq.{impl#2}-le-{closure#0}-{closure#0}.-------.InstrumentCoverage.0.html
Only in expected_mir_dump.partial_eq/: partial_eq.{impl#2}-le-{closure#0}.-------.InstrumentCoverage.0.html
Only in expected_mir_dump.partial_eq/: partial_eq.{impl#2}-le.-------.InstrumentCoverage.0.html
Only in expected_mir_dump.partial_eq/: partial_eq.{impl#2}-lt-{closure#0}-{closure#0}.-------.InstrumentCoverage.0.html
Only in expected_mir_dump.partial_eq/: partial_eq.{impl#2}-lt-{closure#0}.-------.InstrumentCoverage.0.html
Only in expected_mir_dump.partial_eq/: partial_eq.{impl#2}-lt.-------.InstrumentCoverage.0.html
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

warning: function is never used: `unused_fn`
  --> ../coverage/dead_code.rs:15:4
   |
15 | fn unused_fn() {
   |
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted

make: *** [Makefile:78: partial_eq] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/coverage-spanview

test result: FAILED. 209 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out; finished in 26.20s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "11.0.1-rust-1.52.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:38:11
