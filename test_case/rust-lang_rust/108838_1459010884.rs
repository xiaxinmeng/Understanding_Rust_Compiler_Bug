plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:46007752205b5430f5cabe1357251ea7621a9e98)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 10824/14571
........................................................................................ 10912/14571
........................................................................................ 11000/14571
........................................................................................ 11088/14571
.............F..F....................................................................... 11176/14571
........................................................................................ 11352/14571
........................................................................................ 11440/14571
..........................iiiii...i....i.i.............................................. 11528/14571
........................................................................................ 11616/14571
---
failures:

---- [ui] tests/ui/repr/16-bit-repr-c-enum.rs#avr stdout ----

error in revision `avr`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/repr/16-bit-repr-c-enum.rs" "-Zthreads=1" "--cfg" "avr" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/16-bit-repr-c-enum.avr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/16-bit-repr-c-enum.avr/auxiliary" "--target=avr-unknown-gnu-atmega328" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/repr/16-bit-repr-c-enum.rs#msp430 stdout ----

error in revision `msp430`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/repr/16-bit-repr-c-enum.rs" "-Zthreads=1" "--cfg" "msp430" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/16-bit-repr-c-enum.msp430" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/16-bit-repr-c-enum.msp430/auxiliary" "--target=msp430-none-elf" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error
------------------------------------------


