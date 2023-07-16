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
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/while.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/while.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.while
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.while/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "while" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.while
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.while/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.while/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.while/
diff -u --strip-trailing-cr -r expected_mir_dump.while/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.while/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/loops_branches.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/loops_branches.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.loops_branches
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.loops_branches/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "loops_branches" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.loops_branches
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.loops_branches/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.loops_branches/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.loops_branches/
diff -u --strip-trailing-cr -r expected_mir_dump.loops_branches/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.loops_branches/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/abort.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/abort.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.abort
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.abort/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "abort" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.abort
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.abort/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.abort/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.abort/
diff -u --strip-trailing-cr -r expected_mir_dump.abort/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.abort/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/async.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/async.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.async
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.async/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "async" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.async
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.async/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.async/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.async/
diff -u --strip-trailing-cr -r expected_mir_dump.async/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.async/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/simple_match.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/simple_match.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.simple_match
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.simple_match/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "simple_match" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.simple_match
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.simple_match/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.simple_match/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.simple_match/
diff -u --strip-trailing-cr -r expected_mir_dump.simple_match/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.simple_match/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/tight_inf_loop.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/tight_inf_loop.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.tight_inf_loop
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.tight_inf_loop/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "tight_inf_loop" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.tight_inf_loop
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.tight_inf_loop/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.tight_inf_loop/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.tight_inf_loop/
diff -u --strip-trailing-cr -r expected_mir_dump.tight_inf_loop/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.tight_inf_loop/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/closure.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/closure.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.closure
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.closure/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "closure" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.closure
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.closure/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.closure/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.closure/
diff -u --strip-trailing-cr -r expected_mir_dump.closure/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.closure/
diff -u --strip-trailing-cr -r expected_mir_dump.closure/closure.main-{closure#5}.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.closure/closure.main-{closure#5}.-------.InstrumentCoverage.0.html
--- expected_mir_dump.closure/closure.main-{closure#5}.-------.InstrumentCoverage.0.html 2021-02-17 02:45:27.809163051 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.closure/closure.main-{closure#5}.-------.InstrumentCoverage.0.html 2021-02-17 03:26:28.297844008 +0000
@@ -69,47 +69,47 @@
 </style>
 </head>
 <body>
-<div class="code" style="counter-reset: line 110"><span class="line">                      <span><span class="code even" style="--layer: 1" title="111:23-113:6: @0[5]: _15 = const main::{closure#5}::promoted[1]
-111:23-113:6: @0[6]: _7 = &amp;(*_15)
-111:23-113:6: @0[7]: _6 = &amp;(*_7)
-111:23-113:6: @0[8]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
-112:28-112:61: @0[14]: _13 = ()
-112:28-112:61: @0[15]: FakeRead(ForMatchedPlace, _13)
-112:28-112:61: @0[16]: _14 = const main::{closure#5}::promoted[0]
-112:28-112:61: @0[17]: _11 = &amp;(*_14)
-112:28-112:61: @0[18]: _10 = &amp;(*_11)
-112:28-112:61: @0[19]: _9 = move _10 as &amp;[std::fmt::ArgumentV1] (Pointer(Unsize))
-112:28-112:61: @0.Call: _4 = std::fmt::Arguments::new_v1(move _5, move _9) -&gt; [return: bb1, unwind: bb3]
-112:9-112:62: @1.Call: _3 = std::io::_print(move _4) -&gt; [return: bb2, unwind: bb3]
-111:23-113:6: @2[5]: _0 = const ()
-111:23-113:6: @2.Return: return"><span class="annotation">@0,1,2⦊</span>{</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="111:23-113:6: @0[5]: _15 = const main::{closure#5}::promoted[1]
-111:23-113:6: @0[6]: _7 = &amp;(*_15)
-111:23-113:6: @0[7]: _6 = &amp;(*_7)
-111:23-113:6: @0[8]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
-112:28-112:61: @0[14]: _13 = ()
-112:28-112:61: @0[15]: FakeRead(ForMatchedPlace, _13)
-112:28-112:61: @0[16]: _14 = const main::{closure#5}::promoted[0]
-112:28-112:61: @0[17]: _11 = &amp;(*_14)
-112:28-112:61: @0[18]: _10 = &amp;(*_11)
-112:28-112:61: @0[19]: _9 = move _10 as &amp;[std::fmt::ArgumentV1] (Pointer(Unsize))
-112:28-112:61: @0.Call: _4 = std::fmt::Arguments::new_v1(move _5, move _9) -&gt; [return: bb1, unwind: bb3]
-112:9-112:62: @1.Call: _3 = std::io::_print(move _4) -&gt; [return: bb2, unwind: bb3]
-111:23-113:6: @2[5]: _0 = const ()
-111:23-113:6: @2.Return: return">        $crate::io::_print($crate::format_args_nl!($($arg)*));</span></span>
-<span class="line"><span class="code even" style="--layer: 1" title="111:23-113:6: @0[5]: _15 = const main::{closure#5}::promoted[1]
-111:23-113:6: @0[6]: _7 = &amp;(*_15)
-111:23-113:6: @0[7]: _6 = &amp;(*_7)
-111:23-113:6: @0[8]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
-112:28-112:61: @0[14]: _13 = ()
-112:28-112:61: @0[15]: FakeRead(ForMatchedPlace, _13)
-112:28-112:61: @0[16]: _14 = const main::{closure#5}::promoted[0]
-112:28-112:61: @0[17]: _11 = &amp;(*_14)
-112:28-112:61: @0[18]: _10 = &amp;(*_11)
-112:28-112:61: @0[19]: _9 = move _10 as &amp;[std::fmt::ArgumentV1] (Pointer(Unsize))
-112:28-112:61: @0.Call: _4 = std::fmt::Arguments::new_v1(move _5, move _9) -&gt; [return: bb1, unwind: bb3]
-112:9-112:62: @1.Call: _3 = std::io::_print(move _4) -&gt; [return: bb2, unwind: bb3]
-111:23-113:6: @2[5]: _0 = const ()
-111:23-113:6: @2.Return: return">    }<span class="annotation">⦉@0,1,2</span></span></span></span></div>
+<div class="code" style="counter-reset: line 95"><span class="line">                      <span><span class="code even" style="--layer: 1" title="96:23-98:6: @0[5]: _15 = const main::{closure#5}::promoted[1]
+96:23-98:6: @0[6]: _7 = &amp;(*_15)
+96:23-98:6: @0[7]: _6 = &amp;(*_7)
+96:23-98:6: @0[8]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
+97:28-97:61: @0[14]: _13 = ()
+97:28-97:61: @0[15]: FakeRead(ForMatchedPlace, _13)
+97:28-97:61: @0[16]: _14 = const main::{closure#5}::promoted[0]
+97:28-97:61: @0[17]: _11 = &amp;(*_14)
+97:28-97:61: @0[18]: _10 = &amp;(*_11)
+97:28-97:61: @0[19]: _9 = move _10 as &amp;[std::fmt::ArgumentV1] (Pointer(Unsize))
+97:28-97:61: @0.Call: _4 = std::fmt::Arguments::new_v1(move _5, move _9) -&gt; [return: bb1, unwind: bb3]
+97:9-97:62: @1.Call: _3 = std::io::_print(move _4) -&gt; [return: bb2, unwind: bb3]
+96:23-98:6: @2[5]: _0 = const ()
+96:23-98:6: @2.Return: return"><span class="annotation">@0,1,2⦊</span>{</span></span>
+<span class="line"><span class="code even" style="--layer: 1" title="96:23-98:6: @0[5]: _15 = const main::{closure#5}::promoted[1]
+96:23-98:6: @0[6]: _7 = &amp;(*_15)
+96:23-98:6: @0[7]: _6 = &amp;(*_7)
+96:23-98:6: @0[8]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
+97:28-97:61: @0[14]: _13 = ()
+97:28-97:61: @0[15]: FakeRead(ForMatchedPlace, _13)
+97:28-97:61: @0[16]: _14 = const main::{closure#5}::promoted[0]
+97:28-97:61: @0[17]: _11 = &amp;(*_14)
+97:28-97:61: @0[18]: _10 = &amp;(*_11)
+97:28-97:61: @0[19]: _9 = move _10 as &amp;[std::fmt::ArgumentV1] (Pointer(Unsize))
+97:28-97:61: @0.Call: _4 = std::fmt::Arguments::new_v1(move _5, move _9) -&gt; [return: bb1, unwind: bb3]
+97:9-97:62: @1.Call: _3 = std::io::_print(move _4) -&gt; [return: bb2, unwind: bb3]
+96:23-98:6: @2[5]: _0 = const ()
---
test result: FAILED. 210 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out; finished in 28.04s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "11.0.1-rust-1.52.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:39:33
